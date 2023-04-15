

// Vulkan Coordinate System (right handed system)
//  z
//	 /|    
//   |  __ y
//   |   /| 
//   |  /
//   | /
//   |_____________> x
//
//

use std::f32::consts::PI;

use cgmath::InnerSpace;

#[derive(Copy, Clone, Debug)]
pub struct Quad {
    a: cgmath::Point3<f32>,
    b: cgmath::Point3<f32>,
    c: cgmath::Point3<f32>,
    d: cgmath::Point3<f32>,
    normal: cgmath::Vector3<f32>,
    area: f32,
    heat: u8,
}

impl Quad {
    fn new(a: cgmath::Point3<f32>, 
        b: cgmath::Point3<f32>, 
        c: cgmath::Point3<f32>, 
        d: cgmath::Point3<f32>) -> Self 
    {
        let normal = (c - b).cross(a - b).normalize();
        let normal2 = (c - d).cross(a - d).normalize();

        assert!(normal == normal2, "make sure it is really a quad");

        let area = (c - b).cross(a - b).magnitude() / 2.0 + (c - d).cross(a - d).magnitude()  / 2.0;

        Self{
            a,
            b, 
            c, 
            d, 
            normal, 
            area,
            heat: 0,
        }
    }
}

const N: usize = 10;
pub struct Sphere {
    quads: [[Quad; N]; N]
}

impl Sphere {
    fn new() -> Self {
        let r : f32 = 0.5;

        const ZERO : cgmath::Point3<f32> = cgmath::Point3::new(0.0, 0.0, 0.0);

        // create points
        let mut points: [[cgmath::Point3<f32>; N]; N] = [[ZERO; N]; N];

        for j in 0..N {

            let beta = (PI / (N - 1) as f32) * j as f32; 
            
            let z = f32::cos(beta) * r;
            let sub_r = (f32::sin(beta) * r).abs();

            for i in 0..N {
                
                let alpha = (2.0 * PI / N as f32) * i as f32;

                let x = f32::cos(alpha) * sub_r;
                let y = f32::sin(alpha) * sub_r;

                points[j][i] = cgmath::Point3{x, y, z};
            }
        }

        // create quads
        let mut quads: [[Quad; N]; N] = [[Quad::new(ZERO, ZERO, ZERO, ZERO); N]; N];

        for j in 0..N-1 {
            for i in 0..N {
                let a = points[j+1][i];
                let b = points[j+1][(i+1)%N];
                let c = points[j]  [(i+1)%N];
                let d = points[j]  [i];

                quads[j][i] = Quad::new(a, b, c, d);
            }
        }

        Self { quads }
    }
}