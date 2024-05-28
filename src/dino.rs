use macroquad::prelude::*;
use macroquad::experimental::animation::*;
use crate::ground::{Ground, CellType};

pub struct Dino {
    pub x: f32,
    pub y: f32,
    pub vy: f32,
    pub dir: Dir,
    pub job: Job,
    pub sp: f32,
    pub sprite: AnimatedSprite,
}

#[derive(PartialEq, Eq)]
pub enum Dir {
    North,
    East,
    South,
    West
}

#[derive(PartialEq, Eq)]
pub enum Job {
    Idle,
    Walk,
    Build,
}

impl Dir {
    pub fn op(&self) -> Dir {
        match self {
            Dir::North => Dir::South,
            Dir::East => Dir::West,
            Dir::South => Dir::North,
            Dir::West => Dir::East,
        }
    }
}

fn one_in(num: i32) -> bool {
    return rand::gen_range(0, num) == 1;
}

impl Dino {
    pub fn new(x:f32, y:f32, sp: f32) -> Dino {
        Dino {
            x,
            y,
            sp,
            vy: 0.0,
            dir: Dir::East,
            job: Job::Walk,
            sprite: AnimatedSprite::new(
                16,
                16,
                &[
                    Animation {
                        name: "walk".to_string(),
                        row: 0,
                        frames: 4,
                        fps: 6,
                    },
                ],
                true,
            )
        }
    }

    pub fn update(&mut self, ground: &Ground, w:usize) {

        if one_in(1000) {
            self.dir = self.dir.op();
        }

        match self.job {
            Job::Idle => {
                if one_in(500) {
                    self.job = Job::Walk;
                }
            },
            Job::Walk => {
                if one_in(500) {
                    if one_in(2) {
                        self.job = Job::Idle;
                    } else {
                        self.job = Job::Build;
                    }
                }
            },
            Job::Build => {
                if one_in(500) {
                    self.job = Job::Idle;
                }
            }
        }

        let mut sp = if self.dir == Dir::West { -0.2 } else { 0.2 };
        if self.job == Job::Idle {
            sp = 0.0;
        }
        self.x += sp;


        let g = ground.get_cell(self.x as i32 +8, self.y as i32 +16);
        let g2 = ground.get_cell(self.x as i32 +8, self.y as i32 +17);
        let g3 = ground.get_cell(self.x as i32 +8, self.y as i32 +18);
        // Climb
        if g != CellType::Empty && g2 != CellType::Empty {
            self.y -= 1.0;
        }
        // Fall
        if g == CellType::Empty && g2 == CellType::Empty {
            self.vy += 1.0;
            self.y += self.vy;
            if g3 == CellType::Empty {
                self.x -= sp;
                self.y += 1.0;
            }
        } else {
            self.vy = 0.0;
        }
        // Wrap
        if self.x as usize > w  {
            self.x = -16.0;
        }
        if self.x < -16.0 {
            self.x = w as f32;
        }
        self.sprite.update();
    }

}
