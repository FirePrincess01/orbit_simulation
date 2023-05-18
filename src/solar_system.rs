

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

use glam::Vec3;

pub struct SolarSystem {

    // Sun
    sun_pos: Vec3,

    // Plane
    planet_initial_pos: Vec3, // Initial position of the planet. Is at minimal distance to the sun
    planet_pos: Vec3,
    rotation_axis: Vec3, // Axis on which the planet is rotating

    // Orbit
    eccentricity: f32, // [1]; 0 ... circle; 1 ... asymptote (e)
    rotational_speed: f32, // [m² * rad / s] (H)
    minimal_distance: f32, // [m] minimum distance from the sun to the planet (d)
    
    orbital_period: f32, // [s]
    main_axis_length: f32, // [m] length of the main axis of the ellipse (a)
    neutral_radius: f32, // [m] (p)
    
    phi: f32, // [rad]; angle to the radius
    radius: f32, // [m]; distance from the sun to the planet
    dt: f32, // [s]; time step
    t: f32, // [s]; sum of all time steps
}

impl SolarSystem {

    pub fn new(sun_pos: Vec3, planet_pos: Vec3, rotation_axis: Vec3, eccentricity: f32, rotational_speed: f32) -> Self
    {
        let minimal_distance = (planet_pos - sun_pos).length();

        let neutral_radius = SolarSystem::calc_neutral_radius(minimal_distance, eccentricity);
        let main_axis_length = SolarSystem::calc_main_axis_length(minimal_distance, eccentricity);
        let orbital_period = SolarSystem::calc_orbital_period(main_axis_length, neutral_radius, rotational_speed);

        let phi = 0.0;
        let radius = SolarSystem::calc_radius(neutral_radius, eccentricity, phi);

        Self{ 
            sun_pos,
            planet_initial_pos : planet_pos,
            planet_pos,
            rotation_axis,

            eccentricity,
            rotational_speed,
            
            orbital_period,
            main_axis_length,
            minimal_distance,
            neutral_radius,
            
            phi,
            radius,
            dt: 0.0,
            t: 0.0,
        }
    }

    fn calc_neutral_radius(minimal_distance: f32, eccentricity: f32) -> f32 {
        minimal_distance * ( 1.0 + eccentricity )
    }

    fn calc_main_axis_length(minimal_distance: f32, eccentricity: f32) -> f32 {
        minimal_distance / ( 1.0 - eccentricity )
    }

    fn calc_orbital_period(main_axis_length: f32, neutral_radius: f32, rotational_speed: f32) -> f32 {
        2.0 * PI * main_axis_length * f32::sqrt(main_axis_length * neutral_radius) / rotational_speed
    } 

    fn calc_radius(neutral_radius: f32, eccentricity: f32, phi:f32) -> f32 {
        neutral_radius / (1.0 + eccentricity * f32::cos(phi))
    }

    pub fn step(&mut self, dt: f32) {
        
        // calc delta phi 1
        let radius = self.radius;
        let delta_phi1 = dt * self.rotational_speed / (radius * radius);
        
        // calc delta phi 2
        let radius1 = SolarSystem::calc_radius(self.neutral_radius, self.eccentricity, self.phi + delta_phi1);
        let delta_phi2 = dt * self.rotational_speed / (radius1 * radius1);
        
        // use average of delta phi 1 + delta phi 2
        let phi1 = self.phi + (delta_phi1 + delta_phi2) / 2.0;
        let radius1 = SolarSystem::calc_radius(self.neutral_radius, self.eccentricity, phi1);
        
        // save result
        self.phi = phi1;
        self.radius = radius1;
        self.t += dt;
    }

    pub fn get_planet_position(&self) -> Vec3 {
        let rotation = glam::Quat::from_axis_angle(self.rotation_axis, self.phi);

        let r0 = (self.planet_initial_pos - self.sun_pos).normalize();

        let r1 =  rotation * r0;

        let r1 = r1 * self.radius;

        return self.sun_pos + r1;
    }

}
