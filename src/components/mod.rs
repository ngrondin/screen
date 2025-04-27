
use crate::{data::DataStore, layout::LayoutItem, painter::Painter};
pub mod factory;
pub mod container;
pub mod text;
pub mod image;
pub mod news;
pub mod openweather;
pub mod openweatherforecast;
pub mod datetime;

pub trait Component {
    fn produce(&self, data_store: &DataStore) -> Box<dyn LayoutItem>;
}


pub struct Page {
    top_component: Box<dyn Component>
}

impl Page {
    pub fn new(comp: Box<dyn Component>) -> Self {
        Page{ top_component: comp }
    }

    pub fn produce(&self, data_store: &DataStore) -> Painter {
        let mut painter = Painter::new();
        let mut layout = self.top_component.produce(data_store);
        layout.run_layout_top_down(1920, 1080);
        layout.run_layout_position(0, 0);
        //layout.print_layout(0);
        painter.add_actions(&mut layout.get_paint_actions());
        painter
    }
}