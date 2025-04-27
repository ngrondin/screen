use std::cmp::max;

use crate::{framebuffer::Color, painter::{fill::Fill, PaintAction}};

use super::{Layout, LayoutItem};

#[derive(Debug, Clone)]
pub enum ContainerDir {
    Row,
    Column
}

#[derive(Debug, Clone)]
pub enum ContainerAlign {
    Start,
    Center,
    End
}

#[derive(Debug, Clone)]
pub enum ContainerJustify {
    Start,
    Center,
    End
}

pub struct ContainerFixedSize {
    pub width: u32,
    pub height: u32
}

pub struct ContainerBox {
    layout: Layout,
    fixed_size: Option<ContainerFixedSize>,
    dir: ContainerDir,
    align: ContainerAlign,
    justify: ContainerJustify,
    pad: u32,
    color: Option<Color>,
    content: Vec<Box<dyn LayoutItem>>
}

impl ContainerBox {
    pub fn new(dir: ContainerDir, align: ContainerAlign, justify: ContainerJustify, grow: u8, pad:u32, color: Option<Color>) -> Self {
        ContainerBox { 
            layout: Layout::grow_all(grow),
            fixed_size: None,
            dir, 
            align,
            justify,
            pad,
            color, 
            content: vec![] 
        }
    }

    pub fn new_fixed_size(dir: ContainerDir, align: ContainerAlign, justify: ContainerJustify, size: ContainerFixedSize, pad:u32, color: Option<Color>) -> Self {
        ContainerBox { 
            layout: Layout::default(),
            fixed_size: Some(size),
            dir, 
            align,
            justify,
            pad,
            color, 
            content: vec![] 
        }
    }

    pub fn add_content(&mut self, c: Box<dyn LayoutItem>)  {
        self.content.push(c);
    }
 }

impl LayoutItem for ContainerBox {

    fn get_layout(&self) -> &Layout {
        &self.layout
    }
    
    fn run_layout_top_down(&mut self, avail_width: u32, avail_height: u32) {
        let mut content_width = 0;
        let mut content_height = 0;
        let inner_avail_width = if avail_width > (2*self.pad) {avail_width - (2 * self.pad)} else {0};
        let inner_avail_height = if avail_height > (2*self.pad) {avail_height - (2 * self.pad)} else {0};
        match self.dir {
            ContainerDir::Row => {
                let mut sum_width_grow = 0;
                let mut remain_width = inner_avail_width;
                for child in self.content.iter_mut() {
                    if child.get_layout().width_grow == 0 {
                        child.run_layout_top_down(remain_width, inner_avail_height);
                        let child_width = child.get_layout().width.unwrap_or(0);
                        content_width += child_width;
                        content_height = max(content_height, child.get_layout().height.unwrap_or(0));
                        remain_width -= child_width;
                    } else {
                        sum_width_grow += child.get_layout().width_grow;
                    }
                }
                let remain_width = inner_avail_width - content_width;
                for child in self.content.iter_mut() {
                    if child.get_layout().width_grow > 0 {
                        let child_avail_width = (child.get_layout().width_grow as u32) * remain_width / (sum_width_grow as u32);
                        child.run_layout_top_down(child_avail_width, inner_avail_height);
                        content_width += child.get_layout().width.unwrap_or(0);
                        content_height = max(content_height, child.get_layout().height.unwrap_or(0));
                    }
                }
            },
            ContainerDir::Column => {
                let mut sum_height_grow = 0;
                let mut remain_height = inner_avail_height;
                for child in self.content.iter_mut() {
                    if child.get_layout().height_grow == 0 {
                        child.run_layout_top_down(inner_avail_width, remain_height);
                        let child_height = child.get_layout().height.unwrap_or(0);
                        content_height += child_height;
                        content_width = max(content_width, child.get_layout().width.unwrap_or(0));
                        remain_height -= child_height;
                    } else {
                        sum_height_grow += child.get_layout().height_grow;
                    }
                }
                for child in self.content.iter_mut() {
                    if child.get_layout().height_grow > 0 {
                        let child_avail_height = (child.get_layout().height_grow as u32) * remain_height / (sum_height_grow as u32);
                        child.run_layout_top_down(inner_avail_width, child_avail_height);
                        content_height += child.get_layout().height.unwrap_or(0);
                        content_width = max(content_width, child.get_layout().width.unwrap_or(0));
                    }
                }
            }
        }
        if let Some(size) = &self.fixed_size {
            self.layout.width = Some(size.width);
            self.layout.height = Some(size.height);
        } else {
            if self.layout.width_grow == 0 {
                self.layout.width = Some(content_width + (2 * self.pad));
                self.layout.height = Some(content_height + (2 * self.pad));
            } else {
                self.layout.width = Some(avail_width);
                self.layout.height = Some(avail_height);
            }    
        }
    }
    
    fn run_layout_position(&mut self, offsetx: u32, offsety: u32) {
        self.layout.x = Some(offsetx);
        self.layout.y = Some(offsety);
        let w = self.layout.width.unwrap();
        let h = self.layout.height.unwrap();
        let iw = w - (2*self.pad);
        let ih = h - (2*self.pad);
        let mut content_w = 0;
        let mut content_h = 0;
        for child in self.content.iter() {
            content_w += child.get_layout().width.unwrap_or(0);
            content_h += child.get_layout().height.unwrap_or(0);
        }
        let spare_w = if content_w < iw {iw - content_w} else {0};
        let spare_h = if content_h < ih {ih - content_h} else {0};
        let mut ox = offsetx + self.pad;
        let mut oy = offsety + self.pad;
        match self.dir {
            ContainerDir::Row => {
                ox = match self.justify {
                    ContainerJustify::Start => {offsetx + self.pad},
                    ContainerJustify::Center => {offsetx + self.pad + (spare_w / 2)},
                    ContainerJustify::End => {offsetx + self.pad + spare_w},
                }
            },
            ContainerDir::Column => {
                oy = match self.justify {
                    ContainerJustify::Start => {offsety + self.pad},
                    ContainerJustify::Center => {offsety + self.pad + (spare_h / 2)},
                    ContainerJustify::End => {offsety + self.pad},
                }
            }
        }
        for child in self.content.iter_mut() {
            let cw = child.get_layout().width.unwrap();
            let ch = child.get_layout().height.unwrap();
            match self.dir {
                ContainerDir::Row => {
                    oy = match self.align {
                        ContainerAlign::Start => offsety + self.pad,
                        ContainerAlign::Center => offsety + ((ih - ch) / 2) + self.pad,
                        ContainerAlign::End => offsety + h - ch - self.pad,         
                    }
                },
                ContainerDir::Column => {
                    ox = match self.align {
                        ContainerAlign::Start => offsetx + self.pad,
                        ContainerAlign::Center => offsetx + ((iw - cw) / 2) + self.pad,
                        ContainerAlign::End => offsetx + w - cw - self.pad
                    }
                }
            }
            child.run_layout_position(ox, oy);
            match self.dir {
                ContainerDir::Row => {
                    ox += child.get_layout().width.unwrap_or(0);
                },
                ContainerDir::Column => {
                    oy += child.get_layout().height.unwrap_or(0);
                },
            }
        }
    }

    fn print_layout(&self, i: u8) {
        let pre = (0..i).map(|_| " ").collect::<String>();
        println!("{}-Container {}", pre, self.layout);
        for child in self.content.iter() {
            child.print_layout(i + 1);
        }
    }

    fn get_paint_actions(&self) -> Vec<Box<dyn PaintAction>> {
        let mut ret: Vec<Box<dyn PaintAction>> = vec![];
        let x = self.layout.x.unwrap_or(0);
        let y = self.layout.y.unwrap_or(0);
        let w = self.layout.width.unwrap_or(0);
        let h = self.layout.height.unwrap_or(0);
        if let Some(color) = &self.color {
            ret.push(Box::new(Fill::new(x, y, x+w, y+h, &color)));
        }
        for child in self.content.iter() {
            let mut child_paint_actions = child.get_paint_actions();
            ret.append(&mut child_paint_actions);
        }
        ret
    }
}