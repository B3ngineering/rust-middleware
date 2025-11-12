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

#[derive(Debug, Clone)]
pub enum Message {
    Twist,
    Odom,
}

impl Message {
    // Convert from &str to Message enum
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "twist" => Some(Message::Twist),
            "odom" => Some(Message::Odom),
            _ => None,
        }
    }

    // Get default topic for each message type
    pub fn default_topic(&self) -> &str {
        match self {
            Message::Twist => "/cmd_vel",
            Message::Odom => "/odom",
        }
    }
}