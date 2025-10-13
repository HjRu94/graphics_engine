use macroquad::prelude::*;
use std::f64::consts;

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


struct EucPoint {
    x: f64,
    y: f64,
    z: f64,
}

struct SphPoint {
    r: f64,
    po: Option<f64>,
    az: Option<f64>,
}

fn convert(point: &EucPoint) -> SphPoint {
    let x = point.x;
    let y = point.y;
    let z = point.z;
    let r = (x.powi(2) + y.powi(2) + z.powi(2)).sqrt();
    let xy = (x.powi(2) + y.powi(2)).sqrt();
    // po
    let po: Option<f64> = if z > 0.0 {
        Some((xy / z).atan())
    }
    else if z < 0.0 {
        Some(consts::PI + (xy / z).atan())
    }
    else {
        if xy != 0.0 {
            Some(consts::PI / 2.0)
        }
        else {
            None
        }
    };
    // az
    let az: Option<f64> = if x > 0.0 {
        Some((y / x).atan())
    }
    else if x < 0.0 {
        if y >= 0.0 {
            Some((y / x).atan() + consts::PI)
        }
        else {
            Some((y / x).atan() - consts::PI)
        }
    }
    else {
        if y > 0.0 {
            Some(consts::PI / 2.0)
        }
        else if y < 0.0 {
            Some(- consts::PI / 2.0)
        }
        else {
            None
        }
    };
    SphPoint {
        r: r,
        po: po,
        az: az
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut points: Vec<EucPoint> = vec![];
    for i in 0..10 {
        for j in 0..10 {
            points.push(
                EucPoint {
                    x: 10.0,
                    y: i as f64,
                    z: j as f64,
                }
            )
        }
    }
    loop {
        clear_background(WHITE);
        for point in &points {
            let new_point = convert(point);
            draw_circle((new_point.az.expect("HEJ") * 2.0 / consts::PI * 500.0 + 500.0) as f32,(new_point.po.expect("HEJ") * 2.0 / consts::PI * 500.0) as f32, 20.0, RED);
        }
        draw_line(500.0, 0.0, 500.0, 1000.0, 2.0, BLACK);
        draw_line(0.0, 500.0, 1000.0, 500.0, 2.0, BLACK);
        next_frame().await;
    }
}
