use macroquad::prelude::*;
use learning_graphics::geometry::*;
use std::f64::consts;
use learning_graphics::draw::Drawable;

fn window_conf() -> Conf {
    Conf {
        window_title: "Graphics".to_owned(),
        window_width: 2000,
        window_height: 2000,
        high_dpi: true,
        ..Default::default()
    }
}

// FOW pi radiants x pi radiants

#[macroquad::main(window_conf)]
async fn main() {
    let line1 = EucLine::new(
        5.0, 0.0, 0.0, 5.0, 5.0, 0.0
        );
    let line2 = EucLine::new(
        5.0, 0.0, 0.0, 5.0, 0.0, 5.0
        );
    let line3 = EucLine::new(
        5.0, 5.0, 5.0, 5.0, 5.0, 0.0
        );
    let line4 = EucLine::new(
        5.0, 5.0, 5.0, 5.0, 0.0, 5.0
        );
    loop {
        clear_background(WHITE);
        line1.draw();
        line2.draw();
        line3.draw();
        line4.draw();
        draw_line(500.0, 0.0, 500.0, 1000.0, 2.0, BLACK);
        draw_line(0.0, 500.0, 1000.0, 500.0, 2.0, BLACK);
        next_frame().await;
    }
}
