use crate::{get_depth, set_depth, set_pixel, clear};
use crate::float4::Float4;
use std::sync::{Mutex, OnceLock};
use crate::matrix4::Matrix4;
use crate::boundingbox::{BoundingBox2D, BoundingBox3D};

static S_PROJECTION_MATRIX: OnceLock<Mutex<Matrix4>> = OnceLock::new();
static S_VIEW_MATRIX: OnceLock<Mutex<Matrix4>> = OnceLock::new();

static S_CAMERA_POSITION: OnceLock<Mutex<Float4>> = OnceLock::new();
static S_CAMERA_TARGET: OnceLock<Mutex<Float4>> = OnceLock::new();

#[derive(Clone, Copy, Default, Debug)]
pub struct Vertex {
    pub position: [f32; 4],  // model position
    pub tex_coord: [f32; 4], // UV coordinates
    pub normal: [f32; 4],    // normal
    pub tangent: [f32; 4],   // tangent
}

pub struct VSOut {
    pub position_cs: Float4,   // clip space position
    pub position_ws: Float4,   // world space, used for lighting calculation
    pub texcoord: Float4,

    /// normal mapping 
    pub normal_ws: Float4,     // world space normal
    pub tangent_ws: Float4,    // world space tangent
    pub bitangent_ws: Float4,  // world space cotangent
}

pub fn vertex_shader(
    constant_buffer: &ConstantBuffer,
    position_ms: Float4,   // model space position
    texcoord: Float4,
    normal_ms: Float4,     // model space normal
    tangent_ms: Float4,    // model space tangent
) -> VSOut {

    // model space -> world space
    let position_ws = position_ms * constant_buffer.model_matrix;

    // world space -> view space
    let position_vs = position_ws * constant_buffer.view_matrix;

    // view space -> clip space
    let position_cs = position_vs * constant_buffer.projection_matrix;

    // normal transform
    let mut normal_ws = normal_ms * constant_buffer.normal_matrix;
    normal_ws.normalize();

    // tangent transform
    let mut tangent_ws = tangent_ms * constant_buffer.model_matrix;
    tangent_ws.normalize();

    // bitangent = normal × tangent
    let mut bitangent_ws = normal_ws.cross(&tangent_ws);
    bitangent_ws.normalize();

    VSOut {
        position_cs,
        position_ws,
        texcoord,
        normal_ws,
        tangent_ws,
        bitangent_ws,
    }
}


pub fn render_triangle_with_vs(
    constant_buffer: &ConstantBuffer,
    v0: &Vertex,
    v1: &Vertex,
    v2: &Vertex,
    viewport_width: i32,
    viewport_height: i32,
) {
    // vertex shader
    let mut out0 = vertex_shader(
        constant_buffer,
        Float4::from_array4(v0.position),
        Float4::from_array4(v0.tex_coord),
        Float4::from_array4(v0.normal),
        Float4::from_array4(v0.tangent),
    );
    let mut out1 = vertex_shader(
        constant_buffer,
        Float4::from_array4(v1.position),
        Float4::from_array4(v1.tex_coord),
        Float4::from_array4(v1.normal),
        Float4::from_array4(v1.tangent),
    );
    let mut out2 = vertex_shader(
        constant_buffer,
        Float4::from_array4(v2.position),
        Float4::from_array4(v2.tex_coord),
        Float4::from_array4(v2.normal),
        Float4::from_array4(v2.tangent),
    );

    // clip space -> NDC
    out0.position_cs /= out0.position_cs.w;
    out1.position_cs /= out1.position_cs.w;
    out2.position_cs /= out2.position_cs.w;

    // NDC -> screen space, set pixel color
    render_triangle(&out0.position_cs, &out1.position_cs, &out2.position_cs);
}

// render a triangle in screen space
// input: 3 vertices in ndc space [-1,1], output: render the triangle to framebuffer
pub fn render_triangle(a: &Float4, b: &Float4, c: &Float4) {

    //transform from ndc space to screen space
    //[-1,1] -> [-0.5,0.5] -> [0,1] -> [0,w-1] [0,h-1]
    let mut vec1 = *a;
    let mut vec2 = *b;
    let mut vec3 = *c;
    vec1 *= 0.5;
    vec2 *= 0.5;
    vec3 *= 0.5;

    vec1 += Float4::from_xyzw((0.5, 0.5, 0.5, 0.5));
    vec2 += Float4::from_xyzw((0.5, 0.5, 0.5, 0.5));
    vec3 += Float4::from_xyzw((0.5, 0.5, 0.5, 0.5));

    vec1.x *= crate::WIDTH as f32 - 1.0;
    vec1.y *= crate::HEIGHT as f32 - 1.0;
    vec2.x *= crate::WIDTH as f32 - 1.0;
    vec2.y *= crate::HEIGHT as f32 - 1.0;
    vec3.x *= crate::WIDTH as f32 - 1.0;
    vec3.y *= crate::HEIGHT as f32 - 1.0;

    let bbox = BoundingBox2D::from_triangle(&vec1, &vec2, &vec3, crate::WIDTH, crate::HEIGHT);
    if !bbox.is_valid() {
        log::error!("Invalid bounding box, skipping triangle");
        return; //Invalid bounding box, skip rendering this triangle.
    }

    for y in bbox.min_y as i32..=bbox.max_y as i32 {
        for x in bbox.min_x as i32..=bbox.max_x as i32 {
            let bary = compute_barycentric_coords(x as f32, y as f32, &vec1, &vec2, &vec3);
            if bary.x < 0.0 || bary.y < 0.0 || bary.z < 0.0 {
                continue; //Pixel is outside the triangle, skip.
            }
            set_pixel(x, y, 255, 255, 255, 255); //Set pixel color to white for now.
        }
    }
}

pub fn compute_barycentric_coords(
    in_x: f32,
    in_y: f32,
    in_a: &Float4,
    in_b: &Float4,
    in_c: &Float4, 
) -> Float4 {
    let beta_0 = (in_a.y - in_c.y) * in_x + (in_c.x - in_a.x) * in_y + in_a.x * in_c.y - in_c.x * in_a.y;
    let beta_1 = (in_a.y - in_c.y) * in_b.x + (in_c.x - in_a.x) * in_b.y + in_a.x * in_c.y - in_c.x * in_a.y;
    let beta = beta_0 / beta_1;
    

    if beta < 0.0 || beta > 1.0 {
        return Float4::from_xyzw((-1.0, -1.0, -1.0, -1.0)); // alpha,beta,gamma
    }
    

    let gamma_0 = (in_a.y - in_b.y) * in_x + (in_b.x - in_a.x) * in_y + in_a.x * in_b.y - in_b.x * in_a.y;
    let gamma_1 = (in_a.y - in_b.y) * in_c.x + (in_b.x - in_a.x) * in_c.y + in_a.x * in_b.y - in_b.x * in_a.y;
    let gamma = gamma_0 / gamma_1;

    if gamma < 0.0 || gamma > 1.0 {
        return Float4::from_xyzw((-1.0, -1.0, -1.0, -1.0)); // alpha,beta,gamma
    }
    

    let alpha = 1.0 - beta - gamma;
    

    Float4::from_xyzw((alpha, gamma, beta, 0.0))

}


pub struct ConstantBuffer {
    pub projection_matrix: Matrix4,
    pub view_matrix: Matrix4,
    pub model_matrix: Matrix4,
    pub normal_matrix: Matrix4,
    pub camera_world_position: Float4,
}
impl ConstantBuffer {
    fn init(in_projection_matrix: Matrix4,in_view_matrix : Matrix4,in_model_matrix : Matrix4,in_normal_matrix : Matrix4,in_camera_position : Float4) -> Self {
        Self {
            projection_matrix: in_projection_matrix,
            view_matrix: in_view_matrix,
            model_matrix: in_model_matrix,
            normal_matrix: in_normal_matrix,
            camera_world_position: in_camera_position,
        }
    }
}


pub fn init(_width: i32, _height: i32) {
    let camera_position= Float4::from_xyzw((0.0,0.0,0.0,1.0));
    let camera_taget =Float4::from_xyzw((0.0,0.0,-1.0,1.0));
    S_CAMERA_POSITION.set(Mutex::new(camera_position)).unwrap();
    S_CAMERA_TARGET.set(Mutex::new(camera_taget)).unwrap();
    
    let mut projection_matrix= Matrix4::default();
    projection_matrix.perspective(60.0, _width as f32 / _height as f32, 0.1, 1000.0);
    let mut view_matrix= Matrix4::default();
    view_matrix.look_at(
        (camera_position.x, camera_position.y, camera_position.z),
        (camera_taget.x, camera_taget.y, camera_taget.z),
        (0.0, 1.0, 0.0),
    );
    S_PROJECTION_MATRIX.set(Mutex::new(projection_matrix)).unwrap();
    S_VIEW_MATRIX.set(Mutex::new(view_matrix)).unwrap();
}


pub fn render(_delta_time: f64){
    //model space -> world space -> view space -> clip space/ndc space[-1,1] -> screen space[w,h]
    // model matrix -> view matrix -> projection matrix -> viewport transform

    unsafe {

        clear(0, 0, 0, 255);


        // model matrix
        let mut model_matrix = Matrix4::default();
        model_matrix.translate(0.0, 0.0, 0.0);

        // normal matrix = (model^-1)^T
        let normal_matrix = model_matrix
            .invert()
            .unwrap_or(Matrix4::identity())
            .transpose();

        let camera_position = S_CAMERA_POSITION.get().unwrap().lock().unwrap().clone();

        let cb = ConstantBuffer::init(
            S_PROJECTION_MATRIX.get().unwrap().lock().unwrap().clone(),
            S_VIEW_MATRIX.get().unwrap().lock().unwrap().clone(),
            model_matrix,
            normal_matrix,
            camera_position,
        );

        // test triangle, w=1.0
        let v0 = Vertex { position: [-0.5, -0.5, -2.0, 1.0], ..Default::default() };
        let v1 = Vertex { position: [ 0.5, -0.5, -2.0, 1.0], ..Default::default() };
        let v2 = Vertex { position: [ 0.0,  0.5, -2.0, 1.0], ..Default::default() };

        render_triangle_with_vs(&cb, &v0, &v1, &v2, crate::WIDTH, crate::HEIGHT);
    }
}