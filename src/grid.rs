//use std::usize;



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
        for col in 0..COLUMN+1 {
            for row in 0..ROW+1 {
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

    pub fn update(&mut self) {
        for y in 0..COLUMN {
            for x in 0..ROW {
                if self.cells[y as usize][x as usize].has_updated {
                    self.cells[y as usize][x as usize].has_updated = false;
                    continue;
                }
                match self.cells[y as usize][x as usize].cell_type {
                    CellTypes::Sand => {
                        if y < COLUMN {
                            if self.cells[(y+1) as usize][x as usize].cell_type == CellTypes::Air {
                                self.cells[y as usize][x as usize].cell_type = CellTypes::Air;
                                self.cells[(y+1) as usize][x as usize].cell_type = CellTypes::Sand;
                                self.cells[(y+1) as usize][x as usize].has_updated = true;
                            } else if x > 0 
                                && self.cells[(y+1)as usize][(x-1) as usize].cell_type == CellTypes::Air {
                                    self.cells[(y)as usize][ x   as usize].cell_type  = CellTypes::Air;
                                    self.cells[(y+1)as usize][(x-1) as usize].cell_type = CellTypes::Sand;
                                    self.cells[(y+1)as usize][(x-1) as usize].has_updated = true;
                                } else if x < ROW && self.cells[(y+1)as usize][(x+1) as usize].cell_type == CellTypes::Air {
                                    self.cells[(y  )as usize][ x   as usize].cell_type  = CellTypes::Air;
                                    self.cells[(y+1)as usize][(x+1) as usize].cell_type = CellTypes::Sand;
                                    self.cells[(y+1)as usize][(x+1) as usize].has_updated = true;

                                
                            }
                        }
                    }
                    _ => (),
                }
            }
        }
    }

    pub fn add_object(&mut self, r#position: Vector2, r#type: CellTypes) {
        let x: usize = (r#position.x.round() / CELL_SIZE.x) as usize;
        let y: usize = (r#position.y.round() / CELL_SIZE.y) as usize;
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
