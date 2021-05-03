use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, ImageData};
use wasm_bindgen::Clamped;

mod utils;

const MAX_ITERATIONER: usize = 500;

#[derive(Copy, Clone)]
struct Complex {
    a: f64,
    b: f64
}

impl Complex {
    fn square(&mut self) {
        let tmp_a = self.a; 
        self.a = self.a.powi(2) - self.b.powi(2);
        self.b = 2.0 * tmp_a * self.b;
    }

    fn norm(&self) -> f64 {
        self.a.powi(2) + self.b.powi(2)
    }
}

#[wasm_bindgen()]
pub fn draw(
    ctx: &CanvasRenderingContext2d, 
    width: usize,
    height: usize,
    bing: f64,
    bong: f64,
    kong: f64
) {
    utils::init_panic_hook();

    let mut data: Vec<u8> = vec![0; width * height * 4];

    for x in 0..width {
        for y in 0..height {
            // mappa x och y till koordinater i grafen
            let a = map_range(x as f64, 0.0, width as f64, -2.0, 2.0);
            let b = map_range(y as f64, 0.0, height as f64, -2.0, 2.0);

            let c = Complex {a, b};
    
            let mut z = Complex {a: 0f64, b: 0f64}; // Z0

            let mut iteration = 0;
            while iteration < MAX_ITERATIONER {
                z.square();

                z.a += c.a;
                z.b += c.b;

                if z.norm() > 4.0 {
                    break;
                }

                iteration += 1;
            }

            let nsmooth: f64 = match iteration {
                MAX_ITERATIONER => iteration as f64,
                _ => {
                    // log_zn := log(x*x + y*y) / 2
                    let log_zn = (z.norm()).ln() / 2f64;
                    // nu := log(log_zn / log(2)) / log(2)
                    let nu = (log_zn / 2f64.ln()).ln() / 2f64.ln();
                    // iteration := iteration + 1 - nu
                    iteration as f64 + 1.0 - nu
                }
            };

            // Color.HSBtoRGB(0.95f + 10 * smoothcolor ,0.6f,1.0f);
            
            let index = (x + y * width) * 4;

            data[index + 0] = ((bing * nsmooth as f64 + 1.0).sin() * 200.0 + 25.0) as u8;
            data[index + 1] = ((bong * nsmooth as f64 + 2.0).sin() * 230.0 + 25.0) as u8;
            data[index + 2] = ((kong * nsmooth as f64 + 1.0).sin() * 230.0 + 25.0) as u8;
            data[index + 3] = 255;
            /* 
            c.channels[0] = (unsigned char)(sin(0.016 * continuous_index + 4) * 230 + 25);
            c.channels[1] = (unsigned char)(sin(0.013 * continuous_index + 2) * 230 + 25);
            c.channels[2] = (unsigned char)(sin(0.01 * continuous_index + 1) * 230 + 25);
            c.channels[3] = 255; //alpha bit

            */

        }
    }
    let img = ImageData::new_with_u8_clamped_array_and_sh(Clamped(&mut data), width as u32, height as u32).unwrap();
    ctx.put_image_data(&img, 0.0, 0.0).unwrap();
    utils::log("klar");
}

fn map_range(s: f64, a1: f64, b1: f64, a2:f64, b2: f64) -> f64 {
    ((s - a1) / (b1 - a1)) * (b2 - a2) + a2
}