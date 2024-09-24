use crate::cell::*;
use raylib::prelude::*;

pub struct Grid {
    cells: [[Cell; WINDOW_WIDTH as usize]; WINDOW_HEIGHT as usize],
}

impl Grid {
    pub fn new() -> Self {
        let mut _grid = Grid {
            cells: [[Cell::default(); WINDOW_WIDTH as usize]; WINDOW_HEIGHT as usize],
        };
        for col in 0..COLUMN {
            for row in 0..ROW {
                _grid.cells[col as usize][row as usize].position = Vector2 {
                    x: row as f32 * CELL_SIZE.x,
                    y: col as f32 * CELL_SIZE.y 
                };
            }
        }

        _grid 
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        for row in self.cells {
            for cell in row{
                cell.draw(d);
            }
        }
    }

    pub fn move_to(&mut self, cx: usize, cy: usize, dx: usize, dy: usize) -> bool {  
        let cur_cell =  self.cells[cy][cx];
        let dest =  self.cells[dy][dx];

        match cur_cell.cell_type {
            CellTypes::Air =>  true,
            CellTypes::Sand => {
                if dest.cell_type == CellTypes::Air {
                    self.cells[cy][cx].cell_type = CellTypes::Air;
                    self.cells[dy][dx].cell_type = CellTypes::Sand;
                    self.cells[dy][dx].has_updated = true;
                    true
                }  else {
                    false
                }
            }
        }
    }


    pub fn update(&mut self) {
        for y in 0..COLUMN as usize{
            for x in 0..ROW as usize{
                if self.cells[y][x].has_updated {
                    self.cells[y][x].has_updated = false;
                    continue;
                }
                match self.cells[y][x].cell_type {
                    CellTypes::Air => (),
                    CellTypes::Sand => {
                        if y < COLUMN as usize && !self.move_to(x, y, x, y+1) && x > 0 && !self.move_to(x, y, x-1, y+1) && x < ROW as usize {
                            self.move_to(x, y, x+1, y+1);
                        }
                    }
                }
            }
        }
    }

    pub fn add_object(&mut self, r#position: Vector2, r#type: CellTypes) {
        let x: usize = (r#position.x.round() / CELL_SIZE.x) as usize;
        let y: usize = (r#position.y.round() / CELL_SIZE.y) as usize;
        if x + 3 > ROW as usize || y + 3 > COLUMN as usize {return;}
        self.cells[y][x].change_type(r#type);
    }
}

impl Default for Grid {
    fn default() -> Self {
         let mut _grid = Grid {
            cells: [[Cell::default(); WINDOW_WIDTH as usize]; WINDOW_HEIGHT as usize],
        };
        for col in 0..COLUMN {
            for row in 0..ROW {
                _grid.cells[col as usize][row as usize].position = Vector2 {
                    x: row as f32 * CELL_SIZE.x,
                    y: col as f32 * CELL_SIZE.y 
                };
            }
        }

        _grid 
    }
}
