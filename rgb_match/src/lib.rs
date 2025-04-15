use std::mem::swap;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub fn swap(mut self, first: u8, second: u8) -> Color {
        match (first, second) {
            (f, s) if f == self.r && s == self.g => { std::mem::swap(&mut self.r, &mut self.g); }
            (f, s) if f == self.r && s == self.b => { std::mem::swap(&mut self.r, &mut self.b); }
            (f, s) if f == self.r && s == self.a => { std::mem::swap(&mut self.r, &mut self.a); }
          
            (f, s) if f == self.g && s == self.r => { std::mem::swap(&mut self.g, &mut self.r); }
            (f, s) if f == self.g && s == self.b => { std::mem::swap(&mut self.g, &mut self.b); }
            (f, s) if f == self.g && s == self.a => { std::mem::swap(&mut self.g, &mut self.a); }
           
            (f, s) if f == self.b && s == self.r => { std::mem::swap(&mut self.b, &mut self.r); }
            (f, s) if f == self.b && s == self.g => { std::mem::swap(&mut self.b, &mut self.g); }
            (f, s) if f == self.b && s == self.a => { std::mem::swap(&mut self.b, &mut self.a); }
            
            (f, s) if f == self.a && s == self.r => { std::mem::swap(&mut self.a, &mut self.r); }
            (f, s) if f == self.a && s == self.g => { std::mem::swap(&mut self.a, &mut self.g); }
            (f, s) if f == self.a && s == self.b => { std::mem::swap(&mut self.a, &mut self.b); }
            _=> {}
        }
        self
    }
}