fn main() {
    println!("Hello, world!");
}

pub struct Universe {
    length: u32,
    count: u32,
    view_info: Vec<u8>,
}

impl Universe {
    pub fn countup(&mut self) {
        self.count += 1;
    }

    pub fn new(length: u32) -> Universe {
        let view_info = vec![0u8; (length * length) as usize];
        Universe {
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
        let size = self.view_info.len();
        for i in 0..size {
            self.view_info[i as usize] = 0;
        }
    }

    pub fn reverse(&mut self) {
        
    }

}
