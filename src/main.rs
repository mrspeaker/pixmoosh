
use macroquad::prelude::*;
use macroquad::ui::{root_ui, hash};
use ground::{Ground, CellType};
use dino::{Dino, Dir, Job};
use person::{Person};
use maf::one_in;
use resources::load_resources;

mod ground;
mod dino;
mod person;
mod resources;
mod maf;
mod tests;

pub const BG: Color = Color::new(0.0, 0.423, 0.493, 1.00);
pub const WATER: Color = Color::new(0.325, 0.549, 0.549, 1.00);
pub const SAND: Color = Color::new(0.862, 0.549, 0.227, 1.00);
pub const SAND2: Color = Color::new(0.752, 0.49, 0.29, 1.00);
pub const WOOD: Color = Color::new(0.678, 0.419, 0.282, 1.00);
pub const TREE: Color = Color::new(0.3, 0.6, 0.2, 1.00);

#[macroquad::main("Life")]
async fn main() {
    let w = screen_width() as usize;
    let h = screen_height() as usize;

    rand::srand(macroquad::miniquad::date::now() as _);

    let mut selected = CellType::Sand;

    let resources = load_resources().await;

    let mut ground = Ground::new(w, h);
    ground.init();

    let mut dinos: Vec<Dino> = Vec::new();
    for _ in 0..5 {
        let mut d = Dino::new(
            rand::gen_range(0, w) as f32,
            rand::gen_range(0, h /2) as f32,
            rand::gen_range(10, 30) as f32 / 10.0);
        if one_in(2) {
            d.dir = Dir::West;
        }
        if one_in(2) {
            d.job = Job::Idle;
        }
        dinos.push(d);
    }

    let mut peeps: Vec<Person> = Vec::new();
    for _ in 0..5 {
        let mut d = Person::new(
            rand::gen_range(0, w) as f32,
            rand::gen_range(0, h /2) as f32,
            rand::gen_range(10, 30) as f32 / 10.0);
        if one_in(2) {
            d.dir = Dir::West;
        }
        if one_in(2) {
            d.job = Job::Idle;
        }
        peeps.push(d);
    }

    let mut image = Image::gen_image_color(w as u16, h as u16, BLACK);
    let texture = Texture2D::from_image(&image);

    loop {
        if is_mouse_button_down(MouseButton::Left) {
            //let is_shift = is_key_down(KeyCode::LeftShift);
            let c = selected;// if is_shift {CellType::Wood } else { CellType::Sand };

            let (x, y) = mouse_position();
            let size = 8;
            for i in -size..size {
                for j in -size..size {
                    if ((i * i + j * j) as f32).sqrt() < size as f32 {
                        if c == CellType::Wood || one_in(3) {
                            ground.set_cell((x as i32)+i, (y as i32)-j, c);
                        }
                    }
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

        ground.update();

        for i in 0..ground.cells.len() {
            //ground.cells[i] = ground.buf[i];
            ground.moved[i] = false;

            image.set_pixel(
                (i % w) as u32,
                (i / w) as u32,
                match ground.cells[i as usize] {
                    CellType::Empty => BLANK,
                    CellType::Bedrock => RED,
                    CellType::Sand if i % 2 == 0 => SAND2,
                    CellType::Sand => SAND,
                    CellType::AntiSand => GREEN,
                    CellType::Water => WATER,
                    CellType::Wood => WOOD,
                    CellType::Tree => TREE,
                },
            );
        }

        texture.update(&image);

        clear_background(BG);
        draw_texture(&texture, 0., 0., WHITE);
        for d in dinos.iter_mut() {
            let v = d.update(&ground, w, h);

            for gc in v {
                ground.set_cell(gc.0, gc.1, gc.2);
            }

            draw_texture_ex(
                &resources.dino,
                d.x,
                d.y,
                WHITE,
                DrawTextureParams {
                    source: Some(d.sprite.frame().source_rect),
                    dest_size: Some(d.sprite.frame().dest_size),
                    flip_x: d.dir == Dir::West,
                    ..Default::default()
                }
            );
        }

        for d in peeps.iter_mut() {
            let v = d.update(&ground, w, h);

            for gc in v {
                ground.set_cell(gc.0, gc.1, gc.2);
            }

            draw_texture_ex(
                &resources.walk,
                d.x,
                d.y,
                WHITE,
                DrawTextureParams {
                    source: Some(d.sprite.frame().source_rect),
                    dest_size: Some(d.sprite.frame().dest_size),
                    flip_x: d.dir == Dir::West,
                    ..Default::default()
                }
            );
        }


        let _ = root_ui()
            .style_builder()
            .text_color(Color::from_rgba(180, 180, 120, 255))
            .font_size(30)
            .build();

        root_ui().window(hash!(), Vec2::new(10., 10.), Vec2::new(150., 25.), |ui| {
            //let (mouse_wheel_x, _) = mouse_wheel();
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
            ui.same_line(100.);
            if ui.button(None, "Tree") {
                selected = CellType::Tree;
            }

        });

        next_frame().await
    }
}
