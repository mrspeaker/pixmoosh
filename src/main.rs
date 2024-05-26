use macroquad::prelude::*;
use macroquad::experimental::animation::*;
use macroquad::ui::{root_ui, hash, widgets};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum CellType {
    AntiSand,
    Bedrock,
    Empty,
    Sand,
    Water,
    Wood,
}

struct Ground {
    w: usize,
    h: usize,
    cells: Vec<CellType>,
    buf: Vec<CellType>
}

impl Ground {
    fn init(&mut self) {
        for y in 0..self.h as i32{
            for x in 0..self.w as i32 {
                if y as usize > self.h / 2 + self.h / 4 {
                    self.set_cell(x, y, CellType::Sand);
                } else {
                    if rand::gen_range(0, 5) == 0 {
                        self.set_cell(x, y, CellType::Sand);
                    }
                }
                let off = y as usize * self.w + x as usize;
                self.cells[off] = self.buf[off];
            }
        }
    }

    fn get_cell(&self, x: i32, y: i32) -> CellType {
        if x < 0 || x > (self.w - 1) as i32 || y < 0 || y > (self.h - 1) as i32 {
            return CellType::Bedrock;
        }
        return self.cells[y as usize * self.w + x as usize];
    }

    fn set_cell(&mut self, x: i32, y: i32, val: CellType) {
        if x < 0 || x > (self.w - 1) as i32 || y < 0 || y > (self.h - 1) as i32 {
            return;
        }
        self.buf[y as usize * self.w + x as usize] = val;
    }
}

struct Dino {
    x: f32,
    y: f32,
    anim: String
}

#[macroquad::main("Life")]
async fn main() {
    let w = screen_width() as usize;
    let h = screen_height() as usize;

    let mut selected = CellType::Sand;

    let mut dino = Dino {
        x: 1.0,
        y: 0.0,
        anim: "walk".to_string(),
    };

    let tex2: Texture2D = load_texture("res/dino-Sheet.png").await.unwrap();

    let mut ground = Ground {
        w,
        h,
        cells:  vec![CellType::Empty; w * h],
        buf:  vec![CellType::Empty; w * h]
    };

    ground.init();
    dino.y = (ground.h / 2 + ground.h / 4) as f32;

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

    loop {
        clear_background(WHITE);

        if is_mouse_button_down(MouseButton::Left) {
            //let is_shift = is_key_down(KeyCode::LeftShift);
            let c = selected;// if is_shift {CellType::Wood } else { CellType::Sand };

            let (x, y) = mouse_position();
            let size = 8;
            for i in -size..size {
                for j in -size..size {
                    ground.set_cell((x as i32)+i, (y as i32)-j, c);
                }
            }
        }

        if is_mouse_button_down(MouseButton::Right) {
            let (x, y) = mouse_position();
            for i in -10..10 {
                for j in -10..10 {
                    ground.set_cell((x as i32)+i, (y as i32)-j, CellType::AntiSand);
                }
            }
        }

        for y in 0..h as i32 {
            for x in 0..w as i32 {
                let cell = ground.get_cell(x, y);
                if cell == CellType::Empty { continue; }

                if cell == CellType::Wood {
                    continue;
                }

                let cell_d = ground.get_cell(x, y+1);
                if cell_d == CellType::Empty {
                    ground.set_cell(x, y, CellType::Empty);
                    ground.set_cell(x, y+1, cell);
                    continue;
                }
                if cell == CellType::AntiSand && cell_d != CellType::AntiSand {
                    ground.set_cell(x, y, CellType::Empty);
                    ground.set_cell(x, y+1, CellType::Empty);
                    continue;
                }

                let cell_bl = ground.get_cell(x-1, y+1);
                let cell_br = ground.get_cell(x+1, y+1);

                match (cell_bl, cell_br) {
                    (CellType::Empty, CellType::Empty) => {
                        ground.set_cell(x, y, CellType::Empty);
                        let dir = match rand::gen_range(0, 10) {
                            v if v < 5 => -1,
                            _ => 1
                        };
                        ground.set_cell(x+dir, y+1,cell );

                    },
                    (CellType::Empty, _) => {
                        ground.set_cell(x, y, CellType::Empty);
                        ground.set_cell(x-1, y+1, cell);
                    },
                    (_, CellType::Empty) => {
                        ground.set_cell(x, y, CellType::Empty);
                        ground.set_cell(x+1, y+1, cell);
                    },
                    _ => {
                        if cell != CellType::Water {
                            continue;
                        }
                        let cell_t = ground.get_cell(x, y-1);
                        if cell_t == CellType::Water {
                            let cell_l = ground.get_cell(x-1, y);
                            let cell_r = ground.get_cell(x+1, y);
                            if cell_l != CellType::Empty && cell_r != CellType::Empty {
                                continue;
                            }
                            ground.set_cell(x, y, CellType::Empty);
                            if cell_l == CellType::Empty {
                                ground.set_cell(x-1, y, cell);
                            } else {
                                ground.set_cell(x+1, y, cell);
                            }
                        }

                    }
                }
            }
        }

        for i in 0..ground.buf.len() {
            ground.cells[i] = ground.buf[i];

            image.set_pixel(
                (i % w) as u32,
                (i / w) as u32,
                match ground.buf[i as usize] {
                    CellType::Empty => BLANK,
                    CellType::Bedrock => RED,
                    CellType::Sand => BROWN,
                    CellType::AntiSand => GREEN,
                    CellType::Water => BLUE,
                    CellType::Wood => BLACK,
                },
            );
        }

        texture.update(&image);

        dino.x += 0.2;

        let g = ground.get_cell(dino.x as i32 +8, dino.y as i32 +16);
        let g2 = ground.get_cell(dino.x as i32 +8, dino.y as i32 +17);

        if g == CellType::Sand && g2 == CellType::Sand {
            dino.y -= 1.0;
        }
        if g == CellType::Empty && g2 == CellType::Empty {
            dino.y += 1.0;
        }
        if dino.x as usize > w  {
            dino.x = -16.0;
        }

        draw_texture(&texture, 0., 0., WHITE);
        draw_texture_ex(
            &tex2,
            dino.x,
            dino.y,
            WHITE,
            DrawTextureParams {
                source: Some(sprite.frame().source_rect),
                dest_size: Some(sprite.frame().dest_size),
                ..Default::default()
            }
        );
        sprite.update();

        let _ = root_ui()
            .style_builder()
            .text_color(Color::from_rgba(180, 180, 120, 255))
            .font_size(30)
            .build();

        root_ui().window(hash!(), Vec2::new(10., 10.), Vec2::new(150., 25.), |ui| {
            //let (mouse_wheel_x, _) = mouse_wheel();
            //ui.label(None, &format!("wheel x: {}", mouse_wheel_x));
            ui.same_line(0.0);
            if ui.button(None, "Sand") {
                selected = CellType::Sand;
            }
            ui.same_line(35.);
            if ui.button(None, "Wood") {
                selected = CellType::Wood;
            }
            ui.same_line(70.);
            if ui.button(None, "Water") {
                selected = CellType::Water;
            }
        });

        next_frame().await
    }
}
