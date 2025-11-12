use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Twist {
    pub linear_x: f32,
    pub linear_y: f32,
    pub linear_z: f32,
    pub angular_x: f32,
    pub angular_y: f32,
    pub angular_z: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Odom {
    pub x: f32,
    pub y: f32,
    pub theta: f32,
    pub linear_velocity: f32,
    pub angular_velocity: f32,
}

