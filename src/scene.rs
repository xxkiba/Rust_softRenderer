use crate::{get_depth, set_depth, set_pixel, clear};
use crate::float4::Float4;
use crate::boundingbox::{BoundingBox2D, BoundingBox3D};


// render a triangle in screen space
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

pub fn render(_delta_time: f64){
    //model space -> world space -> view space -> clip space/ndc space[-1,1] -> screen space[w,h]
    // model matrix -> view matrix -> projection matrix -> viewport transform

    unsafe {

        clear(0, 0, 0, 255);


        //clear(41, 77, 121, 255); // Clear the framebuffer with black color.
        
        let a = Float4::from_xyz((-0.5, -0.5, 0.0));
        let b = Float4::from_xyz((0.5, -0.5, 0.0));
        let c = Float4::from_xyz((0.0, 0.5, 0.0));

        render_triangle(&a, &b, &c);
        // for x in 0..crate::WIDTH{
        //     set_pixel(x, 150, 255, 255, 255, 255);
        // }

        // Test: draw a counter clockwise triangle
        // set_pixel(a.x as i32, a.y as i32, 255, 0, 0, 255);
        // set_pixel(b.x as i32, b.y as i32, 0, 255, 0, 255);
        // set_pixel(c.x as i32, c.y as i32, 0, 0, 255, 255);
    }
}