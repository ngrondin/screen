use std::cmp::max;

use crate::{fonts::Font, framebuffer::Color, painter::{text::WriteText, PaintAction}};

use super::{Layout, LayoutItem};


pub struct TextBox {
    layout: Layout,
    text: String,
    font: Font,
    color: Color,
    line_height: u32,
    lines: Vec<String>
}

impl TextBox {
    #[allow(dead_code)]
    pub fn new(text: &str, font: &Font, color: &Color) -> Self {
        TextBox { 
            layout: Layout::grow(1, 0), 
            text: text.to_string(), 
            font: font.clone(), 
            color: color.clone(),
            line_height: font.get_height() as u32,
            lines: vec![]
        }
    }
}

impl LayoutItem for TextBox {

    fn get_layout(&self) -> &Layout {
        &self.layout
    }
    
    fn run_layout_top_down(&mut self, avail_width: u32, avail_height: u32) {
        self.lines.clear();
        let mut rem_line = self.text.clone();
        let mut width: u32 = 0;
        let mut height: u32 = 0;
        while rem_line.len() > 0 && height < avail_height {
            let mut pos = rem_line.len();
            let mut line_width;
            loop {
                line_width = self.font.get_width(&rem_line[0..pos]);
                if line_width > avail_width as usize {
                    pos = rem_line[0..pos].rfind(" ").unwrap();
                } else {
                    break;
                }
            };
            let line = rem_line[0..pos].trim().to_string();
            self.lines.push(line);
            rem_line = rem_line[pos..].to_string();
            width = max(width, line_width as u32);
            height += self.line_height;
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
        println!("{}-Textbox {}", pre, self.layout);
    }

    fn get_paint_actions(&self) -> Vec<Box<dyn PaintAction>> {
        let x = self.layout.x.unwrap_or(0);
        let mut y = self.layout.y.unwrap_or(0);
        let mut ret: Vec<Box<dyn PaintAction>> = vec![];
        for line in self.lines.iter() {
            ret.push(Box::new(WriteText::new(line, &self.font, &self.color, x, y)));
            y += self.line_height;
        }
        ret
    }


}
