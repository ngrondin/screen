use std::{cell::RefCell, collections::HashMap, fs};

use rusttype::{point, Font as rtFont, PositionedGlyph, Scale};

#[derive(Debug, Clone)]
pub struct Font {
    font: rtFont<'static>,
    size: f32
}

impl Font {

    fn get_glyphs(&self, text: &str) -> Vec<PositionedGlyph> {
        let scale = Scale { x: self.size * 1.0, y: self.size };
        let v_metrics = self.font.v_metrics(scale);
        let offset = point(0.0, v_metrics.ascent);
        let glyphs: Vec<_> = self.font.layout(text, scale, offset).collect();
        glyphs
    }

    #[allow(dead_code)]
    pub fn get_width(&self, text: &str) -> usize {
        let glyphs = self.get_glyphs(text);
        let width = glyphs.iter().rev()
            .map(|g| g.position().x as f32 + g.unpositioned().h_metrics().advance_width)
            .next().unwrap_or(0.0).ceil() as usize;
        width
    }  

    pub fn get_height(&self) -> usize {
        let scale = Scale { x: self.size * 1.0, y: self.size };
        let v_metrics = self.font.v_metrics(scale);
        (v_metrics.ascent - v_metrics.descent) as usize
    }

    pub fn draw<D>(&self, text: &str, mut drawer: D)
    where D: FnMut(i32, i32, f32) {
        let glyphs = self.get_glyphs(text);
        for g in glyphs {
            if let Some(bb) = g.pixel_bounding_box() {
                g.draw(|x, y, v| {
                    let gx = bb.min.x + x as i32;
                    let gy = bb.min.y + y as i32;
                    drawer(gx, gy, v);
                })
            }
        }
    } 
}


#[derive(Debug, Clone)]
pub struct FontFactory {
    ttf_path_map: HashMap<String, String>,
    fonts: RefCell<HashMap<String, rtFont<'static>>>
}


impl FontFactory {
    pub fn new() -> Self {
        let mut ttf_path_map: HashMap<String, String> = HashMap::new();
        let paths = fs::read_dir("/usr/share/fonts/truetype").unwrap();
        for subpath in paths {
            let subpath_buf = subpath.unwrap().path();
            let ttfpaths = fs::read_dir(subpath_buf).unwrap();
            for ttfpath in ttfpaths {
                let ttfpath_buf = ttfpath.unwrap().path();
                let file_name = ttfpath_buf.file_name().unwrap().to_str().unwrap();                
                if file_name.ends_with(".ttf") {
                    let name = &file_name[0..file_name.len() - 4];
                    let fullpath = ttfpath_buf.to_str().unwrap();
                    //println!("{}", name);
                    ttf_path_map.insert(name.to_string(), fullpath.to_string() );
                }
            }
        }
        FontFactory { ttf_path_map, fonts: RefCell::new(HashMap::new()) }
    }

    pub fn get_font(&self, name: &str, size: f32) -> Option<Font> {
        if let Some(rt_font) = self.fonts.borrow().get(name) {
            return Some(Font { font: rt_font.clone(), size: size });
        }

        if !self.ttf_path_map.contains_key(name) {
            return None;
        }
        
        let path = self.ttf_path_map.get(name).unwrap();
        let font_data = std::fs::read(path).unwrap();
        let rt_font = rtFont::try_from_vec(font_data).unwrap();
        self.fonts.borrow_mut().insert(name.to_string(), rt_font.clone());
        return Some(Font { font: rt_font.clone(), size: size });
    }
}