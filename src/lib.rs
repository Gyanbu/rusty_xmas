#[macro_export]
macro_rules! load_input {
    () => {
        std::fs::read_to_string(format!("src/bin/{}/input.txt", module_path!())).unwrap()
    };
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Vector {
    pub delta_x: isize,
    pub delta_y: isize,
}

impl Vector {
    pub fn new(delta_x: isize, delta_y: isize) -> Vector {
        Vector { delta_x, delta_y }
    }

    pub fn rotate_clockwise(&mut self) {
        // 0, 1 -> -1, 0
        // -1, 0 -> 0, -1
        // 0, -1 -> 1, 0
        // 1, 0 -> 0, 1
        let delta_x = -self.delta_y;
        let delta_y = self.delta_x;
        self.delta_x = delta_x;
        self.delta_y = delta_y;
    }

    pub fn rotate_180(&mut self) {
        self.delta_x = -self.delta_x;
        self.delta_y = -self.delta_y;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl Point {
    pub fn default() -> Point {
        Point { x: 0, y: 0 }
    }

    pub fn new(x: usize, y: usize) -> Point {
        Point { x, y }
    }

    pub fn add_vector(&self, vector: &Vector) -> Option<Point> {
        let x = self.x.checked_add_signed(vector.delta_x)?;
        let y = self.y.checked_add_signed(vector.delta_y)?;
        Some(Point { x, y })
    }

    pub fn vector_between_points(&self, point: &Point) -> Vector {
        let delta_x = point.x as isize - self.x as isize;
        let delta_y = point.y as isize - self.y as isize;
        Vector::new(delta_x, delta_y)
    }
}
