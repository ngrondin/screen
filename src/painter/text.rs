use crate::{fonts::Font, framebuffer::{Color, FrameBuffer}};

use super::PaintAction;



pub struct WriteText {
    text: String,
    font: Font,
    color: Color,
    x: u32,
    y: u32
}

impl WriteText {
    pub fn new(t: &str, f: &Font, c: &Color, x: u32, y: u32) -> Self {
        WriteText { text: t.to_string(), font: f.clone(), color: c.clone(), x, y }
    }
}

impl PaintAction for WriteText {
    fn paint(&self, fb: &mut FrameBuffer) {
        self.font.draw(&self.text, |x, y, v| {
            let px = self.x as i32 + x;
            let py = self.y as i32 + y;
            if v > 0.5 {
                fb.poke(px, py, self.color.red, self.color.green, self.color.blue, v);
            }
        });
    }
}