use macroquad::prelude::*;
use std::fmt;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum CellType {
    AntiSand,
    Bedrock,
    Empty,
    Sand,
    Water,
    Wood,
}
impl fmt::Display for CellType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CellType::Empty => write!(f, "_"),
            CellType::Sand => write!(f, "S"),
            _ => write!(f, "?")
        }
    }
}

pub fn is_solid(t: CellType) -> bool {
    match t {
        CellType::Wood => true,
        CellType::Sand => true,
        CellType::Bedrock => true,
        _ => false
    }
}

pub fn is_free(t: CellType) -> bool {
    return !is_solid(t);
}

pub type GroundChange = (i32, i32, CellType);

pub struct Cell {
    kind: CellType
}

pub struct Ground {
    pub w: usize,
    pub h: usize,
    pub cells: Vec<CellType>,
    pub buf: Vec<CellType>,
    pub moved: Vec<bool>,
}
impl fmt::Display for Ground {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let w = self.w.min(10);
        let h = self.h.min(10);
        for j in 0..h {
            for i in 0..w {
                write!(f, "{}", self.cells[j * self.w + i])?;
            }
            write!(f, "\n")?;
        }
        return Ok(());
    }
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
                self.set_cell(x as i32, y as i32, CellType::Wood);
            }
        }
    }

    pub fn init(&mut self) {
        for y in 0..self.h as i32{
            for x in 0..self.w as i32 {
                if y as usize > self.h / 2 + self.h / 4 {
                    self.set_cell(x, y, CellType::Wood);
                }
                else  {
                    if rand::gen_range(0, 5) == 0 {
                        self.set_cell(x, y, CellType::Sand);
                    }
                }
            }
        }
        let num_lines = 30;// rand::gen_range(20, 40);
        for _ in 0..num_lines {
            self.rnd_line();
        }

        //for i in 0..self.cells.len() {
            //self.cells[i] = self.buf[i];
        //}
    }

    fn swap(&mut self, x: i32, y: i32, xo:i32, yo: i32) {
        let src = self.get_cell(x, y);
        let dst = self.get_cell(x+xo, y+yo);
        if self.set_cell(x + xo, y + yo, src) {
          self.set_cell(x, y, dst);
        }
    }

    pub fn rnd(&mut self) -> i32 {
        return rand::gen_range(0, 10);
    }

    pub fn update(&mut self) {
        for i in 0..self.cells.len() {
            self.moved[i] = false;
        }

        for y in (0..self.h as i32).rev() {
            for x in 0..self.w as i32 {
                let cell = self.get_cell(x, y);
                //println!("{} {} {}", x, y, cell);

                if cell == CellType::Empty { continue; }
                if cell == CellType::Wood { continue; }
                let i = y as usize * self.w + x as usize;
                if self.moved[i] {
                    //println!("moved");
                    continue;
                }

                let cell_d = self.get_cell(x, y+1);

                // Everything falls down...
                if cell_d == CellType::Empty {
                    self.swap(x, y, 0, 1);
                    continue;
                }

                if cell == CellType::AntiSand && cell_d != CellType::AntiSand {
                    self.set_cell(x, y, CellType::Empty);
                    self.set_cell(x, y+1, CellType::Empty);
                    continue;
                }

                let dir = if i % 2 == 0 { -1} else {1};
                let cell_l = self.get_cell(x-1, y);
                let cell_r = self.get_cell(x+1, y);
                let cell_bl = if is_free(cell_l) { self.get_cell(x-1, y+1) } else { CellType::Bedrock } ;
                let cell_br = if is_free(cell_r) { self.get_cell(x+1, y+1) } else { CellType::Bedrock };

                if cell != CellType::Water {
                    match (cell_bl, cell_br) {
                        (CellType::Empty, CellType::Empty) => {
                            self.swap(x, y, dir, 1);
                        },
                        (_, CellType::Empty) => {
                            self.swap(x, y, 1, 1);
                        },
                        (CellType::Empty, _) => {
                            self.swap(x, y, -1, 1);
                        },
                        _ => {
                            continue
                        }
                    }
                } else {
                    match (cell_bl, cell_br) {
                        (CellType::Empty, CellType::Empty) => {
                            self.swap(x, y, dir, 1);
                        },
                        (_, CellType::Empty) => {
                            self.swap(x, y, 1, 1);
                        },
                        (CellType::Empty, _) => {
                            self.swap(x, y, -1, 1);
                        },
                        _ => {
                            if cell_l == CellType::Empty && cell_r == CellType::Empty {
                                self.swap(x, y, dir, 0);
                            } else if cell_l == CellType::Empty {
                                self.swap(x, y, -1, 0);
                            } else {
                                self.swap(x, y, 1, 0);
                            }
                        }
                    }
                }
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
        //let moved = self.moved[cell];
        //if !moved {
         self.cells[cell] = val;
         self.moved[cell] = true;// val != CellType::Empty;
        //}
        //return !moved;
        return true;
    }
}
