
fn main() {
    println!("Hello, world!");
}

pub struct Universe {
    length: u32,
    count: u32,
}

impl Universe {

    pub fn countup(&mut self) {
        self.count += 1;
    }    

    pub fn new(length: u32) -> Universe {
        Universe {
            length,
            count: 0u32,
        }
    }

    pub fn get_length(&self) -> u32 {
        self.length
    }

    pub fn get_count(&self) -> u32 {
        self.count
    }
 
}
