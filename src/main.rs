use ray_game_of_life::{Cell, Coord, Universe};

fn main() {
    println!("Hello");

    let mut uni = Universe::new(7, 7);
    uni.toggle_pixel(Coord::new(2, 3));
    println!("{uni}");
    uni.set_dimensions(Coord::new(5, 6));
    println!("{uni}");
    uni.set_dimensions(Coord::new(10, 4));
    println!("{uni}");
}
