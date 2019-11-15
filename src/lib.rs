mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen(js_namespace=consoso)]
pub fn greet2() {
    alert("Hello, chart-action2!");
}

// #[wasm_bindgen(js_namespace = normal_square_chart)]
#[wasm_bindgen]
pub struct NormalSquareChartUniverse {
    length: u32,
    count: u32,
    view_info: Vec<u8>,
}

// for to_string method
use std::fmt;
impl fmt::Display for NormalSquareChartUniverse {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.view_info.as_slice().chunks(self.length as usize) {
            for &cell in line {
                let symbol = if cell == 0 { '◻' } else { '◼' };
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

#[wasm_bindgen]
impl NormalSquareChartUniverse {
    pub fn countup(&mut self) {
        self.count += 1;
    }

    pub fn new(length: u32) -> NormalSquareChartUniverse {
        let view_info = vec![0u8; (length * length) as usize];
        NormalSquareChartUniverse {
            length,
            count: 0u32,
            view_info,
        }
    }

    pub fn get_length(&self) -> u32 {
        self.length
    }

    pub fn get_count(&self) -> u32 {
        self.count
    }

    pub fn get_view_info(&self) -> *const u8 {
        self.view_info.as_ptr()
    }

    pub fn reset(&mut self, length: u32) {
        self.length = length;
        self.count = 0u32;
        self.view_info = vec![0u8; (length * length) as usize];
    }

    pub fn reverse(&mut self) {
        let size = self.view_info.len();
        for i in 0..size {
            if self.view_info[i as usize] == 0 {
                self.view_info[i as usize] = 1;
            } else {
                self.view_info[i as usize] = 0;
            }
        }
    }

    pub fn tick(&mut self) {
        let count = self.count as usize;
        if self.view_info[count as usize] == 0 {
            self.view_info[count as usize] = 1;
        } else {
            self.view_info[count as usize] = 0;
        }
        let size = self.view_info.len();
        self.count = (self.count + 1) % size as u32;
    }

    pub fn render(&self) -> String {
        self.to_string()
    }
}

pub type Universe = NormalSquareChartUniverse;
