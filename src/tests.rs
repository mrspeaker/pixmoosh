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

    #[test]
    fn fall_sand_either_tower() {
        let mut g = Ground::new(3,3);
        g.set_cell(1, 0, CellType::Sand);
        g.set_cell(1, 1, CellType::Sand);
        g.update();

        assert_eq!(g.get_cell(1,0), CellType::Empty);
        assert_eq!(g.get_cell(1,1), CellType::Sand);
        assert_eq!(g.get_cell(1,2), CellType::Sand);

        g.update();
        assert_eq!(g.get_cell(1,2), CellType::Sand);
        assert_eq!(g.get_cell(0,2), CellType::Sand);
    }


    #[test]
    fn fall_sand_3_x_2() {
        // .SSS.
        // .SSS.
        let mut g = Ground::new(5,2);
        // ground
        g.set_cell(1, 1, CellType::Sand);
        g.set_cell(2, 1, CellType::Sand);
        g.set_cell(3, 1, CellType::Sand);
        // falling
        g.set_cell(1, 0, CellType::Sand);
        g.set_cell(2, 0, CellType::Sand);
        g.set_cell(3, 0, CellType::Sand);
        println!("{}", g);

        g.update();

        println!("{}", g);

        assert_eq!(g.get_cell(0,1), CellType::Sand);
        assert_eq!(g.get_cell(4,1), CellType::Sand);
        g.update();

        println!("{}", g);

        assert_eq!(g.get_cell(0,1), CellType::Sand);
        assert_eq!(g.get_cell(4,1), CellType::Sand);
    }

    #[test]
    fn fall_sand_3_x_3() {
        // ..SSS..
        // ..SSS..
        // ..SSS..
        let mut g = Ground::new(7,3);
        for i in 0..3 {
            g.set_cell(2, i, CellType::Sand);
            g.set_cell(3, i, CellType::Sand);
            g.set_cell(4, i, CellType::Sand);
        }
        println!("{}", g);
        g.update();

        // Should be:
        // ...S...
        // ..SSS..
        // .SSSSS.

        // but is:
        // .......
        // ..SSSS.
        // .SSSSS.

        // I think the top-center sand sneaks in...

        println!("{}", g);

        // Bottom layer
        assert_eq!(g.get_cell(1,2), CellType::Sand);
        assert_eq!(g.get_cell(5,2), CellType::Sand);
        // mid layer
        assert_eq!(g.get_cell(2,1), CellType::Sand);
        assert_eq!(g.get_cell(4,2), CellType::Sand);
        // top layer
        assert_eq!(g.get_cell(3,0), CellType::Sand);

    }


}
