use crate::{get_depth, set_depth, set_pixel, clear};
use crate::float4::Float4;

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
    unsafe {

        clear(0, 0, 0, 255);


        //clear(41, 77, 121, 255); // Clear the framebuffer with black color.
        
        let a = Float4::from_xyz((100.0, 100.0, 0.0));
        let b = Float4::from_xyz((540.0, 100.0, 0.0));
        let c = Float4::from_xyz((320.0, 260.0, 0.0));

        for y in 0..crate::HEIGHT {
            for x in 0..crate::WIDTH {
                
                let barycentric_coords = compute_barycentric_coords(x as f32, y as f32, &a, &b, &c);
                if barycentric_coords.x < 0.0 { // alpha < 0, outside the triangle
                    continue; //Outside the triangle, skip.
                }
                set_pixel(x, y, 41, 77, 121, 255); //Set pixel color based on position.
                set_depth(x, y, 1.0f32); //Set depth value to infinity.
            }
        }
        // for x in 0..crate::WIDTH{
        //     set_pixel(x, 150, 255, 255, 255, 255);
        // }

        // Test: draw a counter clockwise triangle


        // set_pixel(a.x as i32, a.y as i32, 255, 0, 0, 255);
        // set_pixel(b.x as i32, b.y as i32, 0, 255, 0, 255);
        // set_pixel(c.x as i32, c.y as i32, 0, 0, 255, 255);
    }
}