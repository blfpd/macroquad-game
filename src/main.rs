use macroquad::{prelude::*, rand::ChooseRandom};

struct Player {
    color: Color,
    speed: f32,
    shape: Circle,
}

struct Enemy {
    color: Color,
    speed: f32,
    shape: Rect,
}

impl Player {
    fn collide_with(&self, other: &Rect) -> bool {
        self.shape.overlaps_rect(other)
    }
}

#[macroquad::main("My game")]
async fn main() {
    rand::srand(miniquad::date::now() as u64);

    const MOVEMENT_SPEED: f32 = 200.0;

    let colors: Vec<Color> = vec![BLUE, PINK, GRAY];
    let mut gameover = false;

    let mut squares = vec![];
    let mut circle = Player {
        color: YELLOW,
        speed: MOVEMENT_SPEED,
        shape: Circle {
            r: 16.0,
            x: screen_width() / 2.0,
            y: screen_height() / 2.0,
        },
    };

    loop {
        let delta_time = get_frame_time();

        clear_background(DARKBROWN);

        if rand::gen_range(0, 99) >= 95 {
            let size = rand::gen_range(16.0, 64.0);
            let color = *colors.choose().unwrap();
            squares.push(Enemy {
                color,
                speed: rand::gen_range(56.0, 150.0),
                shape: Rect {
                    x: rand::gen_range(size / 2.0, screen_width() - size / 2.0),
                    y: -size,
                    h: size,
                    w: size,
                },
            })
        }

        for square in &mut squares {
            square.shape.y += square.speed * delta_time;
        }

        squares.retain(|square| square.shape.y < screen_height() + square.shape.h);

        if gameover && is_key_pressed(KeyCode::Space) {
            squares.clear();
            circle.shape.x = screen_width() / 2.0;
            circle.shape.y = screen_height() / 2.0;
            gameover = false;
        }

        if is_key_down(KeyCode::Right) {
            circle.shape.x += circle.speed * delta_time;
        }
        if is_key_down(KeyCode::Left) {
            circle.shape.x -= circle.speed * delta_time;
        }
        if is_key_down(KeyCode::Down) {
            circle.shape.y += circle.speed * delta_time;
        }
        if is_key_down(KeyCode::Up) {
            circle.shape.y -= circle.speed * delta_time;
        }

        circle.shape.x = clamp(
            circle.shape.x,
            circle.shape.r,
            screen_width() - circle.shape.r,
        );
        circle.shape.y = clamp(
            circle.shape.y,
            circle.shape.r,
            screen_height() - circle.shape.r,
        );

        if !gameover {
            for square in &squares {
                let rect = square.shape;
                draw_rectangle(rect.x, rect.y, rect.w, rect.h, square.color);
            }

            draw_circle(circle.shape.x, circle.shape.y, circle.shape.r, circle.color);
        }

        if squares
            .iter()
            .any(|square| circle.collide_with(&square.shape))
        {
            gameover = true;
        }

        if gameover {
            let text = "GAME OVER!";
            let text_dimensions = measure_text(text, None, 50, 1.0);
            draw_text(
                text,
                screen_width() / 2. - text_dimensions.width / 2.,
                screen_height() / 2.,
                50.0,
                RED,
            );
        }

        next_frame().await
    }
}
