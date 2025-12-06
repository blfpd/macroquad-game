use macroquad::prelude::*;

#[macroquad::main("My game")]
async fn main() {
    loop {
        clear_background(DARKBROWN);
        next_frame().await
    }
}
