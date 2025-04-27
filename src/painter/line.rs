use crate::framebuffer::{Color, FrameBuffer};

use super::PaintAction;


pub struct PaintLine {
    color: Color,
    x1: u32,
    y1: u32,
    x2: u32,
    y2: u32
}

impl PaintLine {
    #[allow(dead_code)]
    pub fn new(x1: u32, y1: u32, x2: u32, y2: u32, c: &Color) -> Self {
        PaintLine { color: c.clone(), x1, y1, x2, y2 }
    }
}

impl PaintAction for PaintLine {
    fn paint(&self, fb: &mut FrameBuffer) {
        let sx: i32;
        let sy: i32;
        let ex: i32;
        let ey: i32;
        if self.x2.abs_diff(self.x1) > self.y2.abs_diff(self.y1) {
            if self.x1 < self.x2 {
                sx = self.x1 as i32;
                sy = self.y1 as i32;
                ex = self.x2 as i32;
                ey = self.y2 as i32;
            } else {
                sx = self.x2 as i32;
                sy = self.y2 as i32;
                ex = self.x1 as i32;
                ey = self.y1 as i32; 
            }
            for x in sx..ex {
                let y = sy + ((x - sx) * (ey - sy) / (ex - sx));
                fb.poke(x, y, self.color.red, self.color.green, self.color.blue, 1.0);
            }
        } else {
            if self.y1 < self.y2 {
                sx = self.x1 as i32;
                sy = self.y1 as i32;
                ex = self.x2 as i32;
                ey = self.y2 as i32;
            } else {
                sx = self.x2 as i32;
                sy = self.y2 as i32;
                ex = self.x1 as i32;
                ey = self.y1 as i32; 
            }
            for y in sy..ey {
                let x = sx + ((y - sy) * (ex - sx) / (ey - sy));
                fb.poke(x, y, self.color.red, self.color.green, self.color.blue, 1.0);
            }
        }  
    }
}