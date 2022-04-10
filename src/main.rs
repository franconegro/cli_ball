use std::str;
use std::{thread, time};

const WIDTH: usize = 64 * 2;
const HEIGHT: usize = 32;

#[derive(Copy, Clone, Debug)]
enum Pixel {
    BACK = 0,
    FORE = 1,
}

#[derive(Copy, Clone, Debug)]
struct PointF {
    x: f32,
    y: f32,
}

#[derive(Copy, Clone, Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl PointF {
    fn sub(&self, p2: &Self) -> Self {
        PointF {
            x: self.x - p2.x,
            y: self.y - p2.y,
        }
    }
    fn sum(&self, p2: &Self) -> Self {
        PointF {
            x: self.x + p2.x,
            y: self.y + p2.y,
        }
    }
    fn mul(&self, p2: &Self) -> Self {
        PointF {
            x: self.x * p2.x,
            y: self.y * p2.y,
        }
    }
    fn floor(&mut self) -> &Self {
        self.x = (*self).x.floor();
        self.y = (*self).y.floor();
        self
    }
    fn ceil(&mut self) -> &Self {
        self.x = (*self).x.ceil();
        self.y = (*self).y.ceil();
        self
    }
    fn to_i32(&self) -> Point {
        Point {
            x: self.x as i32,
            y: self.y as i32,
        }
    }
    fn to_sqrlen(&self) -> f32 {
        self.x * self.x + self.y * self.y
    }
}

impl Point {
    fn to_f32(&self) -> PointF {
        PointF {
            x: self.x as f32,
            y: self.y as f32,
        }
    }
}

fn circle(c: &PointF, r: &f32, display: &mut [Pixel; WIDTH * HEIGHT]) {
    let radius = PointF { x: *r, y: *r };

    let b = c.sub(&radius).floor().to_i32();
    let e = c.sum(&radius).ceil().to_i32();

    for y in b.y..=e.y {
        for x in b.x..=e.x {
            let p = Point { x: x, y: y }.to_f32().sum(&PointF { x: 0.5, y: 0. });
            let d = c.sub(&p);
            if d.to_sqrlen() <= r * r {
                if 0 <= x && x < WIDTH as i32 && 0 <= y && y < HEIGHT as i32 {
                    display[(y * WIDTH as i32 + x) as usize] = Pixel::FORE;
                }
            }
        }
    }
}

fn show(display: &[Pixel; WIDTH * HEIGHT]) {
    let mut row: [u8; WIDTH] = [0; WIDTH];
    let table = " _^C".as_bytes();

    for y in 0..(HEIGHT / 2) {
        for x in 0..WIDTH {
            let t = display[(2 * y) * WIDTH + x];
            let b = display[(2 * y + 1) * WIDTH + x];

            let i: usize = (((t as u8) << 1) | (b as u8)) as usize;
            let c = (*table)[i];
            row[x] = c;
        }
        let s = match str::from_utf8(&row) {
            Ok(v) => v,
            Err(e) => panic!("Ungültige UTF-8 Reihenfolge: {}", e),
        };
        println!("{}", s);
    }
}
fn back() {
    print!("\x1b[{}D", WIDTH);
    print!("\x1b[{}A", (HEIGHT / 2) as i32);
}

fn main() {
    let mut display: [Pixel; WIDTH * HEIGHT];
    let gravity = 100.;
    let dt = 1. / 30.; // FPS (30)

    let radius = (HEIGHT / 4) as f32;

    let mut pos = PointF {
        x: -radius,
        y: -radius,
    };
    let mut vel = PointF { x: 50., y: 0. };

    loop {
        vel = vel.sum(&PointF {
            x: 0.,
            y: gravity * dt,
        });
        pos = pos.sum(&vel.mul(&PointF { x: dt, y: dt }));

        if pos.y > (HEIGHT as f32 - radius) {
            pos.y = HEIGHT as f32 - radius;
            vel.y *= -0.65;
        }

        if pos.x >= (WIDTH as f32 + radius + (radius * 2.)) as f32 {
            pos = PointF {
                x: -radius,
                y: -radius,
            };
            vel = PointF { x: 50., y: 0. };
        }
        display = [Pixel::BACK; WIDTH * HEIGHT];
        circle(&pos, &radius, &mut display);
        show(&display);
        back();

        let fps_millis = time::Duration::from_millis(1000 / 30);
        thread::sleep(fps_millis);
    }
}
