pub struct AxisX {
    pub width: u32,
    pub height: u32,
    pub pad_left: u32,
    pub pad_bottom: u32,
    pub min_x: u32,
    pub max_x: u32,
    pub step_x: u32,
    pub color: String,
    pub text_color: String,
}

impl AxisX {
    pub fn new(width: u32, height: u32, n: u32) -> Self {
        Self {
            width,
            height,
            pad_left: 50,
            pad_bottom: 30,
            min_x: 0,
            max_x: n - 1,
            step_x: 1,
            color: "#ffffff",
            text_color: "#808080",
        }
    }
}

pub struct AxisY {
    pub width: u32,
    pub height: u32,
    pub pad_left: u32,
    pub pad_bottom: u32,
    pub min_y: u32,
    pub max_y: u32,
    pub step_y: u32,
    pub color: String,
    pub text_color: String,
}

impl AxisY {
    pub fn new(width: u32, height: u32, max_y: u32) -> Self {
        Self {
            width,
            height,
            pad_left: 50,
            pad_bottom: 30,
            min_y: 0,
            max_y: max_y,
            step_y: 1000,
            color: "#ffffff",
            text_color: "#808080",
        }
    }
}
