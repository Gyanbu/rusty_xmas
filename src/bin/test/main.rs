#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let mut buf: Vec<Point> = Vec::new();
    let point = Point { x: 0, y: 0 };
    let distance = 3;
    for x_offset in 0..distance {
        for y_offset in 0..distance - x_offset {
            if x_offset == 0 && y_offset == 0 {
                continue;
            }
            buf.push(Point {
                x: point.x + x_offset,
                y: point.y + y_offset,
            });
            if x_offset != 0 {
                buf.push(Point {
                    x: point.x - x_offset,
                    y: point.y + y_offset,
                });
            }
            if y_offset != 0 {
                buf.push(Point {
                    x: point.x + x_offset,
                    y: point.y - y_offset,
                });
            }
        }
    }
    dbg!(&buf, 2 * distance.pow(2) - 2 * distance, buf.len());
}
