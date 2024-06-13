use ray_game_of_life::{Cell, Coord, Universe};
use macroquad::prelude::*;



#[macroquad::main("Game of Life")]
async fn main() {
    // Variables
    let background_color = Color::from_rgba(24, 25, 38, 255);
    let grid_thickness: f32 = 3.0;
    let grid_color = Color::from_rgba(138, 173, 244, 255);
    let grid_spacing: usize = 40;

    // Simulation
    
    // Main loop
    loop {
        clear_background(background_color);

        //draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);
        //draw_circle(screen_width() - 30.0, screen_height() - 30.0, 15.0, YELLOW);
        //draw_text("HELLO", 20.0, 20.0, 20.0, DARKGRAY);

        draw_grid(grid_thickness, grid_color, grid_spacing);
        next_frame().await
    }
}


fn draw_grid(grid_thickness: f32, grid_color: Color, grid_spacing: usize) {
    for y in (0..screen_height() as usize).step_by(grid_spacing) {
        draw_line(0.0, y as f32,
                  screen_width(), y as f32,
                  grid_thickness, grid_color);
    }
    for x in (0..screen_width() as usize).step_by(grid_spacing) {
        draw_line(x as f32, 0.0,
                  x as f32, screen_height(),
                  grid_thickness, grid_color);
    }
}
