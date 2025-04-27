pub mod containerbox;
pub mod textbox;
pub mod imagebox;

use std::fmt::Display;

use crate::painter::PaintAction;

pub struct Layout {
    x: Option<u32>,
    y: Option<u32>,
    width: Option<u32>,
    height: Option<u32>,
    width_grow: u8,
    height_grow: u8
}

impl Layout {
    pub fn default() -> Self {
        Layout {
            x: None,
            y: None,
            width: None,
            height: None,
            width_grow: 0,
            height_grow: 0
        }
    }

    pub fn grow_all(grow: u8) -> Self {
        Layout {
            x: None,
            y: None,
            width: None,
            height: None,
            width_grow: grow,
            height_grow: grow
        }
    }

    pub fn grow(gw: u8, gh: u8) -> Self {
        Layout {
            x: None,
            y: None,
            width: None,
            height: None,
            width_grow: gw,
            height_grow: gh
        }
    }
}

impl Display for Layout {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let x = if let Some(x) = self.x {x as i32} else {-1};
        let y = if let Some(x) = self.y {x as i32} else {-1};
        let w = if let Some(x) = self.width {x as i32} else {-1};
        let h = if let Some(x) = self.height {x as i32} else {-1};
        write!(f, "x:{}, y:{}, w:{}, h:{}", x, y, w, h)
    }
}

pub trait LayoutItem {
    fn get_layout(&self) -> &Layout;
    fn run_layout_top_down(&mut self, avail_width: u32, avail_height: u32);
    fn run_layout_position(&mut self, offsetx: u32, offsety: u32);
    fn print_layout(&self, i: u8);
    fn get_paint_actions(&self) -> Vec<Box<dyn PaintAction>>;
}