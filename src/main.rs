mod framebuffer;
mod fonts;
mod painter;
mod layout;
mod components;
mod providers;
mod runner;
mod data;
mod utils;
mod models;

use std::env;

use framebuffer::FrameBuffer;
use runner::Runner;

pub fn main()  {
    let args: Vec<String> = env::args().collect();
    let mut fb: FrameBuffer = FrameBuffer::new();
    let mut runner = Runner::new();
    if args.len() == 1 {
        runner.run(&mut fb);
    } else if args.len() == 2 {
        runner.save_page(&args[1], &mut fb);
    }
    
    /*let mut painter = Painter::new();
    let mut factory = ComponentFactory::new();
    let top_components = factory.load_all("./pages");
    let mut top_layout = top_components[0].produce();
    top_layout.run_layout_1();
    top_layout.run_layout_2(1920, 1080);
    top_layout.run_layout_3(0, 0);

    top_layout.log_layout(0);

    let mut paint_actions = top_layout.get_paint_actions();
    painter.add_actions(&mut paint_actions);

    painter.paint_on(&mut fb);

    //fb.send();
    fb.save_png("out.png");*/
}
