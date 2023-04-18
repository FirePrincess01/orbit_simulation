

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

use std::{f32::consts::PI, time::Duration};
use glam::Vec3;

#[derive(Copy, Clone, Debug)]
pub struct Quad {
    a: Vec3,
    b: Vec3,
    c: Vec3,
    d: Vec3,
    centroid: Vec3, // geometric center
    normal: Vec3,
    area: f32,
    heat: f32,
}

impl Quad {
    fn new(a: Vec3, b: Vec3, c: Vec3, d: Vec3) -> Self 
    {
        let centroid = (a + b + c + d) / 4.0;
        let normal = centroid.normalize();
        let area = (c - b).cross(a - b).length() / 2.0 + (c - d).cross(a - d).length() / 2.0;

        Self{
            a,
            b, 
            c, 
            d, 
            centroid,
            normal, 
            area,
            heat: 0.0,
        }
    }
}

pub struct Sphere<const N: usize> {
    quads: [[Quad; N]; N]
}

impl<const N: usize> Sphere<N> {
    pub fn new() -> Self {
        let r : f32 = 0.5;

        // create points
        let mut points: Vec<[Vec3; N]> = vec![[Vec3::ZERO; N]; N+1];

        for j in 0..N+1 {

            let beta = (PI / (N) as f32) * j as f32; 
            
            let z = f32::cos(beta) * r;
            let sub_r = (f32::sin(beta) * r).abs();

            for i in 0..N {
                
                let alpha = (2.0 * PI / N as f32) * i as f32;

                let x = f32::cos(alpha) * sub_r;
                let y = f32::sin(alpha) * sub_r;

                points[j][i] = Vec3{x, y, z};
            }
        }

        // create quads
        let mut quads: [[Quad; N]; N] = [[Quad::new(Vec3::ZERO, Vec3::ZERO, Vec3::ZERO, Vec3::ZERO); N]; N];

        for j in 0..N {
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

    pub fn get_vertices(&self) -> (Vec<f32>, Vec<u16>)
    {
        let vertices_size: usize = N*N*12;
        let indices_size: usize = N*N*6;

        let mut vertices: Vec<f32> = vec![0.0; vertices_size];
        let mut indices: Vec<u16> = vec![0; indices_size];

        let mut k = 0;
        let mut l = 0;
        for j in 0..N {
            for i in 0..N {
                let quad = &self.quads[j][i]; 

                // A
                vertices[k+0] = quad.a.x;
                vertices[k+1] = quad.a.y;
                vertices[k+2] = quad.a.z;

                // B
                vertices[k+3] = quad.b.x;
                vertices[k+4] = quad.b.y;
                vertices[k+5] = quad.b.z;

                // C
                vertices[k+6] = quad.c.x;
                vertices[k+7] = quad.c.y;
                vertices[k+8] = quad.c.z;

                // D
                vertices[k+9] = quad.d.x;
                vertices[k+10] = quad.d.y;
                vertices[k+11] = quad.d.z;

                // A, B, C,
                indices[l+0] = (k/3 + 0) as u16;
                indices[l+1] = (k/3 + 1) as u16;
                indices[l+2] = (k/3 + 2) as u16;

                // C, D, A,
                indices[l+3] = (k/3 + 2) as u16;
                indices[l+4] = (k/3 + 3) as u16;
                indices[l+5] = (k/3 + 0) as u16;

                k += 12;
                l += 6;
            }
        }

        (vertices, indices)
    }

    pub fn get_colors(&self) -> Vec<f32>
    {
        let vertices_size: usize = N*N*12;
        let mut colors: Vec<f32> = vec![0.0; vertices_size];

        let gradient = colorous::MAGMA;

        let mut i = 0;
        while(i < vertices_size)
        {
            let color = gradient.eval_rational(i, vertices_size);

            colors[i] =   color.r as f32 / 255.0;
            colors[i+1] = color.g as f32 / 255.0;
            colors[i+2] = color.b as f32 / 255.0;

            colors[i+3] = color.r as f32 / 255.0;
            colors[i+4] = color.g as f32 / 255.0;
            colors[i+5] = color.b as f32 / 255.0;

            colors[i+6] = color.r as f32 / 255.0;
            colors[i+7] = color.g as f32 / 255.0;
            colors[i+8] = color.b as f32 / 255.0;

            colors[i+9] =  color.r as f32 / 255.0;
            colors[i+10] = color.g as f32 / 255.0;
            colors[i+11] = color.b as f32 / 255.0;

            i += 12;
        }

        colors
    }

    fn update(&mut self, light: f32, dt: Duration) 
    {
        let dt = dt.as_secs_f32();
        let light_intensity: f32 = 1.0;
        let radiation_factor: f32 = 0.1;

        for j in 0..N {
            for i in 0..N {
                let quad = &self.quads[j][i]; 

                let light_dir = (light - quad.centroid).normalize();

                let dheat = light_dir.dot(quad.normal).min(0.0) * light_intensity * dt - (quad.heat * radiation_factor) * dt;

                let quad = &mut self.quads[j][i]; 
                quad.heat = (quad.heat + dheat).min(0.0); 
            }
        }
    }
}