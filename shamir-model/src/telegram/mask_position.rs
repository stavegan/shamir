use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct MaskPosition {
    pub point: String,
    pub x_shift: f64,
    pub y_shift: f64,
    pub scale: f64,
}

impl MaskPosition {
    pub fn from(point: String, x_shift: f64, y_shift: f64, scale: f64) -> Self {
        Self {
            point,
            x_shift,
            y_shift,
            scale,
        }
    }
}
