
use std::rc::Rc;

use image::{DynamicImage, GenericImageView};

use crate::framebuffer::FrameBuffer;

use super::PaintAction;


pub struct PaintImage {
    x: u32,
    y: u32,
    w: u32,
    h: u32,
    image: Rc<DynamicImage>
}

impl PaintImage {
    #[allow(dead_code)]
    pub fn new(x: u32, y: u32, w: u32, h: u32, image:Rc<DynamicImage>) -> Self {
        PaintImage { x, y, w, h, image }
    }
}

impl PaintAction for PaintImage {
    fn paint(&self, fb: &mut FrameBuffer) {
        let final_img = if self.w != self.image.width() {
            Rc::new(self.image.resize(self.w, self.h, image::imageops::FilterType::Nearest))
        } else {
            self.image.clone()
        };
        for y in 0..final_img.height() {
            for x in 0..final_img.width() {
                let pix = final_img.get_pixel(x, y);
                let o = (pix[3] as f32) / 255.0;
                fb.poke((self.x + x) as i32, (self.y + y) as i32, pix[0], pix[1], pix[2], o);
            }
        }
    }
}