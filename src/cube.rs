

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

//
//   C ----------------- B
//    /                /|
//   / |              / |
// D/  |           A /  |
// -----------------/   |
// |   |            |   |
// |   |            |   |
// |  G ------------|-- | F
// |  /             |   /
// | /              |  /
// |/               | /
// ------------------
// H                E

use cgmath::InnerSpace;

pub struct Point3 {
    x: f32,
    y: f32,
    z: f32,
}

pub struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
}

pub struct Color3 {
    r: f32,
    g: f32,
    b: f32,
}

pub struct Quad {
    a: cgmath::Point3<f32>,
    b: cgmath::Point3<f32>,
    c: cgmath::Point3<f32>,
    d: cgmath::Point3<f32>,
    normal: cgmath::Vector3<f32>,
    color: Color3,
}

impl Quad {
    fn new(a: cgmath::Point3<f32>, b: cgmath::Point3<f32>, c: cgmath::Point3<f32>, d: cgmath::Point3<f32>, color: Color3) -> Self {
        let normal = (c - b).cross(a - b).normalize();
        let normal2 = (c - d).cross(a - d).normalize();

        assert!(normal == normal2);

        Self{
            a,
            b, 
            c, 
            d, 
            normal, 
            color,
        }

    }
}

/// A cube made of vertices, indices and colors
pub struct Cube {
    position: [Point3; 24],
    color: [Point3; 24],
    normal: [Point3; 24],
    indices: [u16; 36],
}

// impl Cube {
//     fn new() -> Self {
//         let a: f32 = 0.5;
//         let color = Color3{ r: 0.2, g: 0.4, b: 0.6 };

//         let A = Point3{ x:  a, y: -a, z:  a };    
//         let B = Point3{ x:  a, y:  a, z:  a };   
//         let C = Point3{ x: -a, y:  a, z:  a }; 
//         let D = Point3{ x: -a, y: -a, z:  a };     
    
//         let E = Point3{ x:  a, y: -a, z: -a };  
//         let F = Point3{ x:  a, y:  a, z: -a };    
//         let G = Point3{ x: -a, y:  a, z: -a };   
//         let H = Point3{ x: -a, y: -a, z: -a };   

//         let top = Quad{ a: A, b: B, c: C, d: D,  }
//     }
// }