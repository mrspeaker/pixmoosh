use crate::ground::{Ground, CellType};
use macroquad::prelude::*;

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn init_ground() {
        let g = Ground::new(2,2);
        let c = g.get_cell(0,0);
        assert_eq!(c, CellType::Empty);
    }

    #[test]
    fn fall_sand_down() {
        let mut g = Ground::new(1,2);
        g.set_cell(0, 0, CellType::Sand);
        g.update();
        assert_eq!(g.get_cell(0,0), CellType::Empty);
        assert_eq!(g.get_cell(0,1), CellType::Sand);
    }

    #[test]
    fn fall_sand_right() {
        let mut g = Ground::new(2,2);
        g.set_cell(0, 0, CellType::Sand);
        g.set_cell(0, 1, CellType::Sand);
        g.update();
        println!("{}", g);
        assert_eq!(g.get_cell(0,0), CellType::Empty);
        assert_eq!(g.get_cell(0,1), CellType::Sand);
        assert_eq!(g.get_cell(1,1), CellType::Sand);
    }

    #[test]
    fn fall_sand_left() {
        let mut g = Ground::new(2,2);
        g.set_cell(1, 0, CellType::Sand);
        g.set_cell(1, 1, CellType::Sand);
        g.update();
        assert_eq!(g.get_cell(1,0), CellType::Empty);
        assert_eq!(g.get_cell(0,1), CellType::Sand);
        assert_eq!(g.get_cell(1,1), CellType::Sand);
    }

    #[test]
    fn fall_sand_either_right() {
        let mut g = Ground::new(4,2);
        g.set_cell(1, 0, CellType::Sand);
        g.set_cell(1, 1, CellType::Sand);
        g.update();
        assert_eq!(g.get_cell(1,0), CellType::Empty);
        assert_eq!(g.get_cell(1,1), CellType::Sand);
        assert_eq!(g.get_cell(2,1), CellType::Sand);
    }

    #[test]
    fn fall_sand_either_left() {
        let mut g = Ground::new(4,2);
        g.set_cell(2, 0, CellType::Sand);
        g.set_cell(2, 1, CellType::Sand);
        g.update();
        assert_eq!(g.get_cell(2,0), CellType::Empty);
        assert_eq!(g.get_cell(2,1), CellType::Sand);
        assert_eq!(g.get_cell(1,1), CellType::Sand);
    }

}
