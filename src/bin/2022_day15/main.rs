use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn distance(&self, target: &Self) -> usize {
        let x_distance = self.x.abs_diff(target.x);
        let y_distance = self.y.abs_diff(target.y);
        x_distance + y_distance
    }

    // fn get_neighbors(&self, distance: usize) -> Vec<Point> {
    //     let mut buf: Vec<Point> = Vec::with_capacity(2 * distance.pow(2) - 2 * distance);
    //     for d1 in 1..=distance {
    //         for d2 in d1..=distance {}
    //     }
    //     buf
    // }
}

#[derive(Debug, PartialEq, Eq)]
struct Reading {
    sensor: Point,
    beacon: Point,
    range: usize,
}

impl Reading {
    fn new(reading: &Vec<isize>) -> Self {
        if reading.len() != 4 {
            panic!();
        }
        let sensor = Point {
            x: reading[0],
            y: reading[1],
        };
        let beacon = Point {
            x: reading[2],
            y: reading[3],
        };
        let range = sensor.distance(&beacon);
        Self {
            sensor,
            beacon,
            range,
        }
    }
}

#[derive(Debug)]
struct Cave {
    readings: Vec<Reading>,
    occupied: HashSet<Point>,
    min_x: isize,
    max_x: isize,
    min_y: isize,
    max_y: isize,
}

impl Cave {
    fn new(mut readings: Vec<Reading>) -> Self {
        let mut occupied = HashSet::new();
        let mut min_x: isize = isize::MAX;
        let mut max_x: isize = isize::MIN;
        let mut min_y: isize = isize::MAX;
        let mut max_y: isize = isize::MIN;

        readings.sort_by_key(|reading| reading.sensor.x);

        for reading in &readings {
            if reading
                .sensor
                .x
                .checked_add_unsigned(reading.range)
                .unwrap()
                > max_x
            {
                max_x = reading
                    .sensor
                    .x
                    .checked_add_unsigned(reading.range)
                    .unwrap();
            }
            if reading
                .sensor
                .x
                .checked_sub_unsigned(reading.range)
                .unwrap()
                < min_x
            {
                min_x = reading
                    .sensor
                    .x
                    .checked_sub_unsigned(reading.range)
                    .unwrap();
            }
            if reading
                .sensor
                .y
                .checked_add_unsigned(reading.range)
                .unwrap()
                > max_y
            {
                max_y = reading
                    .sensor
                    .y
                    .checked_add_unsigned(reading.range)
                    .unwrap();
            }
            if reading
                .sensor
                .y
                .checked_sub_unsigned(reading.range)
                .unwrap()
                < min_y
            {
                min_y = reading
                    .sensor
                    .y
                    .checked_sub_unsigned(reading.range)
                    .unwrap();
            }
            occupied.insert(reading.sensor);
            occupied.insert(reading.beacon);
        }

        Self {
            readings,
            occupied,
            min_x,
            max_x,
            min_y,
            max_y,
        }
    }

    fn get_row(&self, y: isize) -> usize {
        let mut counter = 0;
        for x in self.min_x..=self.max_x {
            let point = Point { x, y };
            if self.occupied.contains(&point) {
                continue;
            }
            for reading in &self.readings {
                if point.distance(&reading.sensor) <= reading.range {
                    counter += 1;
                    break;
                }
            }
        }
        counter
    }

    fn check_row(&self, y: isize) -> Point {
        let mut row: Vec<[isize; 2]> = Vec::new();
        for reading in &self.readings {
            let y_diff = y.abs_diff(reading.sensor.y);
            let row_range = reading.range as isize - y_diff as isize;
            if row_range <= 0 {
                continue;
            }

            row.push([reading.sensor.x - row_range, reading.sensor.x + row_range]);
        }
        dbg!(&row);
        row.sort_by_key(|range| range[0]);
        // row.sort();

        let mut ranges_to_pop: Vec<usize> = Vec::new();
        for (i, ranges) in row.windows(2).enumerate() {
            if ranges[1][1] < ranges[0][1] {
                ranges_to_pop.push(i + 1);
            }
        }
        for i in ranges_to_pop.into_iter().rev() {
            row.remove(i);
        }
        dbg!(&row);

        for ranges in row.windows(2) {
            if ranges[1][0] - ranges[0][1] == 2 {
                return Point {
                    x: ranges[0][1] + 1,
                    y,
                };
            }
        }
        panic!()
    }

    fn find_distress_beacon(&self) -> Point {
        use indicatif::{ProgressBar, ProgressStyle};
        use rayon::prelude::*;
        use std::sync::Mutex;
        use std::time::Instant;

        const MIN_COORDINATE: isize = 0;
        // const MAX_COORDINATE: isize = 20;
        const MAX_COORDINATE: isize = 4000000;

        let start = Instant::now();
        let bar = ProgressBar::new((MAX_COORDINATE - MIN_COORDINATE + 1) as u64);
        bar.set_style(ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {pos}/{len} ({eta})")
            .unwrap()
            .progress_chars("#>-"));

        let result = Mutex::new(None);

        (MIN_COORDINATE..=MAX_COORDINATE)
            .into_par_iter()
            .for_each(|y| {
                if result.lock().unwrap().is_some() {
                    return;
                }

                let mut row: Vec<[isize; 2]> = Vec::new();
                for reading in &self.readings {
                    let y_diff = y.abs_diff(reading.sensor.y);
                    let row_range = reading.range as isize - y_diff as isize;
                    if row_range <= 0 {
                        continue;
                    }

                    row.push([reading.sensor.x - row_range, reading.sensor.x + row_range]);
                }

                // Remove fully overlapped ranges
                row.sort_by_key(|range| range[0]);
                let mut ranges_to_pop: Vec<usize> = Vec::new();
                for (i, ranges) in row.windows(2).enumerate() {
                    if ranges[1][1] < ranges[0][1] {
                        ranges_to_pop.push(i + 1);
                    }
                }
                let row_clone = row.clone();
                for i in ranges_to_pop.into_iter().rev() {
                    row.remove(i);
                }

                'outer: for ranges in row.windows(2) {
                    if ranges[1][0] - ranges[0][1] == 2 {
                        let point = Point {
                            x: ranges[0][1] + 1,
                            y,
                        };

                        // idc anymore
                        for reading in &self.readings {
                            if point.distance(&reading.sensor) <= reading.range {
                                break 'outer;
                            }
                        }

                        dbg!(&row_clone, &row, point);
                        *result.lock().unwrap() = Some(point);
                        return;
                    }
                }

                bar.inc(1);
            });

        bar.finish_with_message("Search complete");

        let beacon = match *result.lock().unwrap() {
            Some(point) => {
                println!("Time elapsed: {:?}", start.elapsed());
                point
            }
            None => panic!(),
        };
        beacon
    }
}

fn main() {
    let input = rusty_xmas::load_input!();
    // let input = std::fs::read_to_string("input.txt").unwrap();
    let input: Vec<Vec<isize>> = input
        .lines()
        .map(|line| {
            line.split([',', ':'])
                .map(|str| {
                    str.matches(|char| "-0123456789".contains(char))
                        .collect::<String>()
                        .parse()
                        .unwrap()
                })
                .collect()
        })
        .collect();
    let readings: Vec<Reading> = input.iter().map(|reading| Reading::new(reading)).collect();

    let cave = Cave::new(readings);
    println!("Part 1: {}", cave.get_row(2000000));

    // dbg!(cave.check_row(2062167));
    // return;
    let distress_beacon = cave.find_distress_beacon();
    dbg!(distress_beacon);
    println!(
        "Part 2: {}",
        distress_beacon.x * 4000000 + distress_beacon.y
    );
}
