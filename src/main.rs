use macroquad::prelude::*;
use macroquad::experimental::animation::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum CellState {
    Empty,
    Sand,
    AntiSand,
    Bedrock
}

struct Ground {
    w: usize,
    h: usize,
    cells: Vec<CellState>,
    buf: Vec<CellState>
}

impl Ground {
    fn init(&mut self) {
        for y in 0..self.h as i32{
            for x in 0..self.w as i32 {
                if y as usize > self.h / 2 + self.h / 4 {
                    self.set_cell(x, y, CellState::Sand);
                } else {
                    if rand::gen_range(0, 5) == 0 {
                        self.set_cell(x, y, CellState::Sand);
                    }
                }
                let off = y as usize * self.w + x as usize;
                self.cells[off] = self.buf[off];
            }
        }
    }

    fn get_cell(&self, x: i32, y: i32) -> CellState {
        if x < 0 || x > (self.w - 1) as i32 || y < 0 || y > (self.h - 1) as i32 {
            return CellState::Bedrock;
        }
        return self.cells[y as usize * self.w + x as usize];
    }

    fn set_cell(&mut self, x: i32, y: i32, val: CellState) {
        if x < 0 || x > (self.w - 1) as i32 || y < 0 || y > (self.h - 1) as i32 {
            return;
        }
        self.buf[y as usize * self.w + x as usize] = val;
    }
}

#[macroquad::main("Life")]
async fn main() {
    let w = screen_width() as usize;
    let h = screen_height() as usize;

    let mut x = 1.0;
    let mut y;

    let tex2: Texture2D = load_texture("res/dino-Sheet.png").await.unwrap();

    let mut ground = Ground {
        w,
        h,
        cells:  vec![CellState::Empty; w * h],
        buf:  vec![CellState::Empty; w * h]
    };

    ground.init();

    let mut image = Image::gen_image_color(w as u16, h as u16, BLACK);
    let texture = Texture2D::from_image(&image);

    let mut sprite = AnimatedSprite::new(
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
    );
        y = (ground.h / 2 + ground.h / 4) as f32;

    loop {
        clear_background(DARKBLUE);

        x += 0.2;


        if is_mouse_button_down(MouseButton::Left) {
            let (x, y) = mouse_position();
            for i in -10..10 {
                for j in -10..10 {
                    ground.set_cell((x as i32)+i, (y as i32)-j, CellState::Sand);
                }
            }
        }

        if is_mouse_button_down(MouseButton::Right) {
            let (x, y) = mouse_position();
            for i in -10..10 {
                for j in -10..10 {
                    ground.set_cell((x as i32)+i, (y as i32)-j, CellState::AntiSand);
                }
            }
        }
        for y in 0..h as i32 {
            for x in 0..w as i32 {
                let cell = ground.get_cell(x, y);
                if cell == CellState::Empty { continue; }
                let cell_d = ground.get_cell(x, y+1);
                if cell_d == CellState::Empty {
                    ground.set_cell(x, y, CellState::Empty);
                    ground.set_cell(x, y+1, cell);
                    continue;
                }
                if cell == CellState::AntiSand && cell_d != CellState::AntiSand {
                    ground.set_cell(x, y, CellState::Empty);
                    ground.set_cell(x, y+1, CellState::Empty);
                    continue;
                }
                let cell_bl = ground.get_cell(x-1, y+1);
                let cell_br = ground.get_cell(x+1, y+1);
                match (cell_bl, cell_br) {
                    (CellState::Empty, CellState::Empty) => {
                        ground.set_cell(x, y, CellState::Empty);
                        let dir = match rand::gen_range(0, 10) {
                            v if v < 5 => -1,
                            _ => 1
                        };
                        ground.set_cell(x+dir, y+1,cell );

                    },
                    (CellState::Empty, _) => {
                        ground.set_cell(x, y, CellState::Empty);
                        ground.set_cell(x-1, y+1, cell);
                    },
                    (_, CellState::Empty) => {
                        ground.set_cell(x, y, CellState::Empty);
                        ground.set_cell(x+1, y+1, cell);
                    },
                    _ => continue
                }
            }
        }

        for i in 0..ground.buf.len() {
            ground.cells[i] = ground.buf[i];

            image.set_pixel(
                (i % w) as u32,
                (i / w) as u32,
                match ground.buf[i as usize] {
                    CellState::Sand => BROWN,
                    CellState::AntiSand => GREEN,
                    CellState::Empty => BLANK,
                    CellState::Bedrock => RED
                },
            );
        }

        texture.update(&image);

        let g = ground.get_cell(x as i32 +8, y as i32 +16);
        let g2 = ground.get_cell(x as i32 +8, y as i32 +17);

        if g == CellState::Sand && g2 == CellState::Sand {
            y -= 1.0;
        }
        if g == CellState::Empty && g2 == CellState::Empty {
            y+=1.0;
        }
        if x as usize > w  {
            x = -16.0;
        }


        draw_texture(&texture, 0., 0., WHITE);
        draw_texture_ex(
            &tex2,
            x,
            y,
            WHITE,
            DrawTextureParams {
                source: Some(sprite.frame().source_rect),
                dest_size: Some(sprite.frame().dest_size),
                ..Default::default()
            }
        );
        sprite.update();

        next_frame().await
    }
}
