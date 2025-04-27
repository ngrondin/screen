use std::fs::{self, File};

use png_encode_mini::write_rgba_from_u32;


#[derive(Debug, Clone)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Color{ red: r, green: g, blue: b }
    }

    pub fn from_string(str: &str) -> Self {
        if str.starts_with("#") && str.len() == 7 {
            let red = u8::from_str_radix(&str[1..3], 16).unwrap();
            let green = u8::from_str_radix(&str[3..5], 16).unwrap();
            let blue = u8::from_str_radix(&str[5..7], 16).unwrap();
            Color {red, green, blue}
        } else {
            Color {red: 0, green: 0, blue: 0}
        }
    }
}

pub struct FrameBuffer {
    buf: Vec<u8>,
    width: u32,
    height: u32,
    bpp: u8,
}

impl FrameBuffer {
    pub fn new() -> Self {
        let width: u32 = 1920;
        let height: u32 = 1080;
        let bpp: u8 = 16;
        let size: usize = (width * height * ((bpp / 8) as u32)) as usize;
        let buf:Vec<u8> = vec![0; size];
        FrameBuffer {buf, width, height, bpp}
    }

    #[allow(dead_code)]
    pub fn send(&self) {
        match fs::write("/dev/fb0", &self.buf) {
            Ok(_) => {},
            Err(error) => {
                println!("{}", error);
            },
        }
    }
    
    pub fn poke(&mut self, x: i32, y: i32, r: u8, g: u8, b: u8, o: f32) {
        let w: i32 = self.width as i32;
        let h: i32 = self.height as i32;
        if x >= 0 && x < w && y >= 0 && y < h {
            let i: usize = (x + (w * y)) as usize;
            if self.bpp == 16 {
                let e1 = self.buf[(i * 2) + 1];
                let e2 = self.buf[(i * 2) + 0];
                let erf = ((e1 & 248) as f32) / 255.0;
                let egf = ((((e1 << 3) | (e2 >> 5)) << 2) as f32) / 255.0;
                let ebf = (((e2 << 3) & 248) as f32) / 255.0;
                let trf = (r as f32) / 255.0;
                let tgf = (g as f32) / 255.0;
                let tbf = (b as f32) / 255.0;
                let nrf = erf + (o * (trf - erf));
                let ngf = egf + (o * (tgf - egf));
                let nbf = ebf + (o * (tbf - ebf));
                let nr = (nrf * 255.0) as u8;
                let ng = (ngf * 255.0) as u8;
                let nb = (nbf * 255.0) as u8;
                self.buf[(i * 2) + 1] = (nr & 248) | ((ng & 224) >> 5);
                self.buf[(i * 2) + 0] = ((ng & 28) << 3) | ((nb & 248) >> 3);
            }    
        }
    }

    pub fn clear(&mut self) {
        for i in 0..(2 * self.width * self.height) as usize {
            self.buf[i] = 0;
        }
    }

    #[allow(dead_code)]
    pub fn save_png(&self, filename: &str) {
        let mut buf32:Vec<u32> = vec![0; (self.width * self.height)as usize];
        for y in 0..self.height as usize {
            for x in 0..self.width as usize {
                let in_ptr = x + (y * self.width as usize);
                let out_ptr = x + ((self.height as usize - 1 - y) * self.width as usize);
                let b1 = self.buf[(in_ptr * 2) + 1];
                let b2 = self.buf[(in_ptr * 2) + 0];
                let r = b1 & 248;
                let g = ((b1 << 3) | (b2 >> 5)) << 2;
                let b = (b2 << 3) & 248;
                let a: u8 = 255;
                buf32[out_ptr] = ((r as u32) << 0) | ((g as u32) << 8) | ((b as u32) << 16) | ((a as u32) << 24);
            }
        }
        let mut file = File::create(filename).unwrap();
        write_rgba_from_u32(&mut file, &buf32, self.width, self.height).unwrap();
    }
}