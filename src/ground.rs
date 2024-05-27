use macroquad::prelude::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum CellType {
    AntiSand,
    Bedrock,
    Empty,
    Sand,
    Water,
    Wood,
}

pub struct Ground {
    pub w: usize,
    pub h: usize,
    pub cells: Vec<CellType>,
    pub buf: Vec<CellType>,
    pub moved: Vec<bool>,
}

impl Ground {
    pub fn new(w: usize, h: usize) -> Ground {
        Ground {
            w,
            h,
            cells:  vec![CellType::Empty; w * h],
            buf:  vec![CellType::Empty; w * h],
            moved: vec![false; w * h]
        }
    }

    pub fn init(&mut self) {
        for y in 0..self.h as i32{
            for x in 0..self.w as i32 {
                if y as usize > self.h / 2 + self.h / 4 {
                    self.set_cell(x, y, CellType::Sand);
                }
                if y as usize > self.h / 2  {
                    if rand::gen_range(0, 5) == 0 {
                        self.set_cell(x, y, CellType::Sand);
                    }
                }
                let off = y as usize * self.w + x as usize;
                self.cells[off] = self.buf[off];
            }
        }
    }

    pub fn get_cell(&self, x: i32, y: i32) -> CellType {
        if x < 0 || x > (self.w - 1) as i32 || y < 0 || y > (self.h - 1) as i32 {
            return CellType::Bedrock;
        }
        return self.cells[y as usize * self.w + x as usize];
    }

    pub fn set_cell(&mut self, x: i32, y: i32, val: CellType) -> bool {
        if x < 0 || x > (self.w - 1) as i32 || y < 0 || y > (self.h - 1) as i32 {
            return false;
        }
        let cell = y as usize * self.w + x as usize;
        let moved = self.moved[cell] && val != CellType::Empty;
        if !moved {
            self.buf[cell] = val;
            self.moved[cell] = true;
        }
        return !moved;
    }
}
