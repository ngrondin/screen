use std::cmp::{max, min};

use crate::framebuffer::{Color, FrameBuffer};

use super::PaintAction;


pub struct Fill {
    color: Color,
    x1: u32,
    y1: u32,
    x2: u32,
    y2: u32
}

impl Fill {
    #[allow(dead_code)]
    pub fn new(x1: u32, y1: u32, x2: u32, y2: u32, c: &Color) -> Self {
        Fill { color: c.clone(), x1, y1, x2, y2 }
    }
}

impl PaintAction for Fill {
    fn paint(&self, fb: &mut FrameBuffer) {
        let sx = min(self.x1, self.x2) as i32;
        let ex = max(self.x1, self.x2) as i32;
        let sy = min(self.y1, self.y2) as i32;
        let ey = max(self.y1, self.y2) as i32;
        for y in sy..ey {
            for x in sx..ex {
                fb.poke(x, y, self.color.red, self.color.green, self.color.blue, 1.0);
            }
        }
    }
}