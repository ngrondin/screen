pub mod text;
pub mod line;
pub mod fill;
pub mod rect;
pub mod image;


use crate::framebuffer::FrameBuffer;

pub trait PaintAction {
    fn paint(&self, fb: &mut FrameBuffer);
}


pub struct Painter {
    actions: Vec<Box<dyn PaintAction>>
}

impl Painter {
    pub fn new() -> Self {
        Painter{ actions: vec![] }
    }

    #[allow(dead_code)]
    pub fn add_action(&mut self, action: Box<dyn PaintAction>) {
        self.actions.push(action);
    }

    pub fn add_actions(&mut self, actions: &mut Vec<Box<dyn PaintAction>>) {
        self.actions.append(actions);
    }

    pub fn paint_on(&self, fb: &mut FrameBuffer) {
        for action in self.actions.iter() {
            action.paint(fb);
        }
    }

    #[allow(dead_code)]
    pub fn clear(&mut self) {
        self.actions.clear();
    }
}

