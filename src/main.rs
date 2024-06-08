use ray_game_of_life::{Cell, Coord, Universe};

fn main() {
    println!("Hello");

    let mut uni = Universe::new(10, 10);
    println!("{uni}");
    uni.set_pixel(Coord::new(2, 3), Cell::Alive);

    println!("{uni}");
}
