use serde_json::Value;

use crate::{data::DataStore, framebuffer::Color, layout::{containerbox::{ContainerAlign, ContainerBox, ContainerDir}, LayoutItem}};

use super::Component;


pub struct Container {
    dir: ContainerDir,
    align: ContainerAlign,
    grow: u8,
    pad: u32,
    color: Option<Color>,
    children: Vec<Box<dyn Component>>
}

impl Container {
    pub fn new(value: &Value) -> Self {
        let dir = match value["dir"].as_str() {
            Some(dir_str) => { if dir_str == "row" { ContainerDir::Row } else { ContainerDir::Column } },
            None => ContainerDir::Column
        };
        let align = match value["align"].as_str() {
            Some(align_str) => { 
                match align_str {
                    "start" => ContainerAlign::Start,
                    "center" => ContainerAlign::Center,
                    "end" => ContainerAlign::End,
                    _ => ContainerAlign::Start
                }
            },
            None => ContainerAlign::Start
        };
        let grow = match value["grow"].as_u64() {
            Some(grow_num) => grow_num as u8,
            None => 0
        };
        let pad = match value["pad"].as_u64() {
            Some(pad_num) => pad_num as u32,
            None => 0
        };
        let color = match value["color"].as_str() {
            Some(color_str) => Some(Color::from_string(color_str)),
            None => None
        };
        Container{ dir, align, grow, pad, color, children: vec![] }
    }

    pub fn add_child(&mut self, child: Box<dyn Component>) {
        self.children.push(child);
    }
}

impl Component for Container {
    fn produce(&self, data_store: &DataStore) -> Box<dyn LayoutItem> {
        let mut container_box = ContainerBox::new(self.dir.clone(), self.align.clone(), self.grow, self.pad, self.color.clone());
        for child in self.children.iter() {
            let childbox = child.produce(data_store);
            container_box.add_content(childbox);
        }
        Box::new(container_box)
    }
}