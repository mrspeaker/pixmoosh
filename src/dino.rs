use macroquad::experimental::animation::*;
use crate::ground::{Ground, CellType};

pub struct Dino {
    pub x: f32,
    pub y: f32,
    pub sprite: AnimatedSprite,
}

impl Dino {
    pub fn new(x:f32, y:f32) -> Dino {
        Dino {
            x,
            y,
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
        let sp = 0.2;
        self.x += sp;

        let g = ground.get_cell(self.x as i32 +8, self.y as i32 +16);
        let g2 = ground.get_cell(self.x as i32 +8, self.y as i32 +17);

        // Climb
        if g != CellType::Empty && g2 != CellType::Empty {
            self.y -= 1.0;
        }
        // Fall
        if g == CellType::Empty && g2 == CellType::Empty {
            self.y += 1.0;
            self.x -= sp;
        }
        // Wrap
        if self.x as usize > w  {
            self.x = -16.0;
        }
        self.sprite.update();
    }

}
