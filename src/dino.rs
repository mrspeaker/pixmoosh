use macroquad::prelude::*;
use macroquad::experimental::animation::*;
use crate::ground::{Ground, CellType, GroundChange, is_solid};
use crate::maf::one_in;

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
    Bridge,
    Dig,
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
                        name: "idle".to_string(),
                        row: 0,
                        frames: 1,
                        fps: 6,
                    },
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

    pub fn update(&mut self, ground: &Ground, w:usize, h:usize) -> Vec<GroundChange> {

        let mut v:Vec<GroundChange> = Vec::new();

        if one_in(1000) {
            self.dir = self.dir.op();
        }

        match self.job {
            Job::Idle => {
                if one_in(500) {
                    if one_in(5) {
                        self.job = Job::Dig;
                        self.sprite.set_animation(0);
                    } else {
                        self.job = Job::Walk;
                        self.sprite.set_animation(1);

                    }
                }
            },
            Job::Walk => {
                if one_in(500) {
                    if one_in(2) {
                        self.job = Job::Idle;
                        self.sprite.set_animation(0);
                    } else {
                        if one_in(2) {
                            self.job = Job::Build;
                        } else {
                            self.job = Job::Bridge;
                        }
                        self.sprite.set_animation(1);
                    }
                }
            },
            Job::Build => {
                if one_in(500) {
                    self.job = Job::Idle;
                    self.sprite.set_animation(0);
                }
            },
            Job::Bridge => {
                if one_in(500) {
                    self.job = Job::Idle;
                    self.sprite.set_animation(0);
                }
            },
            Job::Dig => {
                if one_in(50) {
                    self.job = Job::Idle;
                    self.sprite.set_animation(0);
                    v.push(((self.x as i32)+6, (self.y as i32) + 17, CellType::Tree));
                    v.push(((self.x as i32)+7, (self.y as i32) + 17, CellType::Tree));
                    v.push(((self.x as i32)+8, (self.y as i32) + 17, CellType::Tree));
                }
            }
        }

        let sp = if self.dir == Dir::West { -0.2 } else { 0.2 };
        let mut xo = sp;
        let is_idle = self.job == Job::Idle;
        if is_idle || self.job == Job::Dig {
            xo = 0.0;
        }
        let g = ground.get_cell(self.x as i32 +8, self.y as i32 +16);
        let g2 = ground.get_cell(self.x as i32 +8, self.y as i32 +17);
        // Climb
        if is_solid(g) && is_solid(g2) {
            self.y -= 1.0;
            if g == CellType::Tree {
                xo = 0.0;
            }
        }
        // Fall
        if !is_solid(g) && !is_solid(g2) {
            self.vy += 1.0;
            self.y += self.vy;
            let g3 = ground.get_cell(self.x as i32 +8, self.y as i32 +18);
            if !is_solid(g3) {
                self.x -= sp;
                self.y += 1.0;
            }
        } else {
            self.vy = 0.0;
        }

        self.x += xo;

        // Jobs
        if self.job == Job::Dig {
            self.vy = 0.0;
            v.push(((self.x as i32)+6, (self.y as i32) + 16, CellType::Empty));
            v.push(((self.x as i32)+7, (self.y as i32) + 16, CellType::Empty));
            v.push(((self.x as i32)+8, (self.y as i32) + 16, CellType::Empty));
            v.push(((self.x as i32)+9, (self.y as i32) + 16, CellType::Empty));
            v.push(((self.x as i32)+10, (self.y as i32) + 16, CellType::Empty));

            v.push(((self.x as i32)+6, (self.y as i32) + 17, CellType::Empty));
            v.push(((self.x as i32)+7, (self.y as i32) + 17, CellType::Empty));
            v.push(((self.x as i32)+8, (self.y as i32) + 17, CellType::Empty));
            v.push(((self.x as i32)+9, (self.y as i32) + 17, CellType::Empty));
            v.push(((self.x as i32)+10, (self.y as i32) + 17, CellType::Empty));
        }
        if self.job == Job::Build {
            let xoff = if self.dir == Dir::West { 4 } else { 10 };
            v.push(((self.x as i32)+xoff, (self.y as i32) + 16, CellType::Wood));
            v.push(((self.x as i32)+xoff, (self.y as i32) + 17, CellType::Wood));
            v.push(((self.x as i32)+xoff, (self.y as i32) + 18, CellType::Wood));
            v.push(((self.x as i32)+xoff+1, (self.y as i32) + 16, CellType::Wood));
            v.push(((self.x as i32)+xoff+1, (self.y as i32) + 17, CellType::Wood));
            v.push(((self.x as i32)+xoff+1, (self.y as i32) + 18, CellType::Wood));
        }

        if self.job == Job::Bridge {
            let xoff = if self.dir == Dir::West { 4 } else { 10 };
            v.push(((self.x as i32)+xoff, (self.y as i32) + 17, CellType::Wood));
            v.push(((self.x as i32)+xoff, (self.y as i32) + 18, CellType::Wood));
            v.push(((self.x as i32)+xoff, (self.y as i32) + 19, CellType::Wood));
            v.push(((self.x as i32)+xoff+1, (self.y as i32) + 17, CellType::Wood));
            v.push(((self.x as i32)+xoff+1, (self.y as i32) + 18, CellType::Wood));
            v.push(((self.x as i32)+xoff+1, (self.y as i32) + 19, CellType::Wood));
        }

        // Wrap
        if self.x as usize > w  {
            self.x = -16.0;
        }
        if self.x < -16.0 {
            self.x = (w as f32) - 1.0;
        }
        if self.y < -16.0 {
            self.y = (h as f32) - 16.0;
        }
        self.sprite.update();
        return v;
    }

}
