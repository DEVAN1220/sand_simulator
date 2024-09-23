use raylib::prelude::*;
pub const WINDOW_HEIGHT: i32 = 400;
pub const WINDOW_WIDTH: i32 = 400;
const SIZE_PARAM: i32 = 320;
pub const CELL_SIZE: Vector2 = Vector2 { x: (WINDOW_WIDTH / SIZE_PARAM) as f32, y: (WINDOW_HEIGHT / SIZE_PARAM) as f32};
pub const ROW: i32 = WINDOW_WIDTH / CELL_SIZE.x as i32;
pub const COLUMN: i32 = WINDOW_HEIGHT / CELL_SIZE.y as i32 - 1;


#[derive(Clone, Copy, PartialEq)]
pub enum CellTypes {
    Air,
    Sand
}

#[derive(Clone, Copy)]
pub struct Cell {
    pub position: Vector2,
    pub cell_type: CellTypes,
    pub has_updated: bool,
}

impl Cell {
    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        //d.draw_rectangle_v(self.position, CELL_SIZE, Color::RED);
        match self.cell_type {
            CellTypes::Air => (),// d.draw_rectangle_lines(self.position.x as i32, self.position.y as i32, CELL_SIZE.y as i32, CELL_SIZE.x as i32, Color::WHITE),
            CellTypes::Sand => d.draw_rectangle_v(self.position, CELL_SIZE, Color::YELLOW),
        }
    }

    pub fn change_type(&mut self, r#type: CellTypes) {
        self.cell_type = r#type;
    }
}

impl Default for Cell {
    fn default() -> Self {
        Cell {
            position: Vector2 {x: 0.0, y: 0.0},
            cell_type: CellTypes::Air,
            has_updated: false,
        }
    }
}
