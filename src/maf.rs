use macroquad::prelude::*;

pub fn one_in(num: i32) -> bool {
    return rand::gen_range(0, num) == 1;
}
