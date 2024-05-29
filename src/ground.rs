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

    fn rnd_line(&mut self) {
        let x1 = rand::gen_range(0, self.w);
        let w = rand::gen_range(20, 130);
        let y1 = rand::gen_range(40, self.h / 2 +100);
        let h = rand::gen_range(5,10);
        for x in x1..x1+w {
            for y in y1..y1+h {
                self.set_cell(x as i32, y as i32, CellType::Wood, false);
            }
        }
    }

    pub fn init(&mut self) {
        for y in 0..self.h as i32{
            for x in 0..self.w as i32 {
                if y as usize > self.h / 2 + self.h / 4 {
                    self.set_cell(x, y, CellType::Wood, false);
                }
                else  {
                    if rand::gen_range(0, 5) == 0 {
                        self.set_cell(x, y, CellType::Sand, false);
                    }
                }
            }
        }
        let num_lines = 30;// rand::gen_range(20, 40);
        for _ in 0..num_lines {
            self.rnd_line();
        }

        for i in 0..self.cells.len() {
            self.cells[i] = self.buf[i];
        }
    }

    pub fn get_cell(&self, x: i32, y: i32) -> CellType {
        if x < 0 || x > (self.w - 1) as i32 || y < 0 || y > (self.h - 1) as i32 {
            return CellType::Bedrock;
        }
        return self.cells[y as usize * self.w + x as usize];
    }

    pub fn set_cell(&mut self, x: i32, y: i32, val: CellType, do_move: bool) -> bool {
        if x < 0 || x > (self.w - 1) as i32 || y < 0 || y > (self.h - 1) as i32 {
            return false;
        }
        let cell = y as usize * self.w + x as usize;
        let moved = self.moved[cell] && val != CellType::Empty;
        if !moved {
            self.buf[cell] = val;
            if do_move {
                self.moved[cell] = true;
            }
        }
        return !moved;
    }
}
