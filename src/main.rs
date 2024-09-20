pub mod cell;
pub mod grid;
use raylib::prelude::*;
use cell::*;
use grid::*;



fn main() {
     let (mut rl, thread) = raylib::init()
        .size(WINDOW_HEIGHT, WINDOW_WIDTH)
        .title("Hello, World")
        .build();

    rl.set_target_fps(60);
    let mut grid = Grid::new();
    let mut mouse_position: Vector2; 
    //let mut cells: [[Cell; WINDOW_WIDTH as usize]; WINDOW_HEIGHT as usize] = [[Cell::default(); WINDOW_WIDTH as usize]; WINDOW_HEIGHT as usize];
    //for i in 0..COLUMN {
    //    for j in 0..ROW {
    //        cells[i as usize][j as usize] = Cell { position: Vector2 {x: (j as f32 * CELL_SIZE.x) , y: (i  as f32 * CELL_SIZE.y) }, cell_type: CellTypes::Air};
    //    }
    //}
    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);

        mouse_position = d.get_mouse_position();
        grid.update();
        grid.draw(&mut d);
        if d.is_mouse_button_down(MouseButton::MOUSE_BUTTON_LEFT) &&
                     (mouse_position.x > 0.0 || mouse_position.x < WINDOW_WIDTH as f32) &&
                    (mouse_position.y > 0.0 || mouse_position.y < WINDOW_HEIGHT as f32)
        {
            grid.add_object(mouse_position, CellTypes::Sand);
        }

    }   
}
