use std::rc::Rc;

use image::DynamicImage;

use crate::painter::image::PaintImage;

use super::{Layout, LayoutItem};



pub struct ImageBox {
    layout: Layout,
    image: Rc<DynamicImage>
}

impl ImageBox {
    pub fn new(image: Rc<DynamicImage>) -> Self {
        ImageBox { layout: Layout::default(), image }
    }
}

impl LayoutItem for ImageBox {
    fn get_layout(&self) -> &Layout {
        &self.layout
    }

    fn run_layout_top_down(&mut self, avail_width: u32, avail_height: u32) {
        let mut width = self.image.width();
        let mut height = self.image.height();
        if width > avail_width || height > avail_height {
            let scale = (avail_width as f32 / width as f32).min(avail_height as f32 / height as f32);
            width = (width as f32 * scale) as u32;
            height = (height as f32 * scale) as u32;
        }
        self.layout.width = Some(width);
        self.layout.height = Some(height);     
    }

    fn run_layout_position(&mut self, offsetx: u32, offsety: u32) {
        self.layout.x = Some(offsetx);
        self.layout.y = Some(offsety);
    }

    fn print_layout(&self, i: u8) {
        let pre = (0..i).map(|_| " ").collect::<String>();
        println!("{}-Image {}", pre, self.layout);
    }

    fn get_paint_actions(&self) -> Vec<Box<dyn crate::painter::PaintAction>> {
        let x = self.layout.x.unwrap_or(0);
        let y = self.layout.y.unwrap_or(0);
        let w = self.layout.width.unwrap_or(0);
        let h = self.layout.height.unwrap_or(0);
        vec![Box::new(PaintImage::new(x, y, w, h, self.image.clone()))]
    }
}