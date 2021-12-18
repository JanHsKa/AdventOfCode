use std::collections::HashSet;

type Coordinate = (i32, i32);

#[derive(Debug)]
struct Probe {
    pub position: Coordinate,
    pub velocity: Coordinate,
    pub highest_y: i32,
}

impl Probe {
    fn throw(&mut self, bounds: (Coordinate, Coordinate)) -> Result<i32, ()> {
        while !self.too_far(bounds) {
            self.position.0 += self.velocity.0;
            self.position.1 += self.velocity.1;

            if self.position.1 > self.highest_y {
                self.highest_y = self.position.1;
            }

            if self.in_bounds(bounds) {
                return Ok(self.highest_y);
            }

            if self.velocity.0 > 0 {
                self.velocity.0 -= 1;
            }

            self.velocity.1 -= 1;
        }

        Err(())
    }

    fn in_bounds(&self, bounds: (Coordinate, Coordinate)) -> bool {
        if self.position.0 >= bounds.0 .0 && self.position.0 <= bounds.1 .0 {
            if self.position.1 >= bounds.0 .1 && self.position.1 <= bounds.1 .1 {
                return true;
            }
        }

        false
    }

    fn too_far(&self, bounds: (Coordinate, Coordinate)) -> bool {
        if self.position.0 > bounds.1 .0 || self.position.1 < bounds.0 .1 {
            return true;
        }

        if self.velocity.0 == 0 && self.position.0 < bounds.0 .0 {
            return true;
        }

        false
    }
}

fn main() {
    let input = get_input_field();

    let result = try_throws(input);

    println!("The highest y is : {}", result);
}

fn try_throws(input: (Coordinate, Coordinate)) -> i32 {
    let mut highest_y = 0;
    let mut start_velocity;
    let mut results: HashSet<Coordinate> = HashSet::new();
    // for i in 0..100 {
    //     let mut new_probe = Probe {
    //         position: (0, 0),
    //         velocity: start_velocity,
    //         highest_y: 0,
    //     };
    //     if new_probe.throw(input).is_ok() {
    //         if new_probe.highest_y > highest_y {
    //             highest_y = new_probe.highest_y;
    //         }
    //     }

    //     println!("{:?}", new_probe);
    //     start_velocity = calculate_new_velocity(input, new_probe, start_velocity);
    //     println!("new velocity {:?}", start_velocity);
    // }

    for x in -1000..1000 {
        for y in -1000..1000 {
            start_velocity = (x, y);
            let mut new_probe = Probe {
                position: (0, 0),
                velocity: start_velocity,
                highest_y: 0,
            };
            if new_probe.throw(input).is_ok() {
                if new_probe.highest_y > highest_y {
                    highest_y = new_probe.highest_y;
                }
                results.insert(start_velocity);
            }
        }
    }

    results.len() as i32
}

fn calculate_new_velocity(
    bounds: (Coordinate, Coordinate),
    probe: Probe,
    start_velocity: Coordinate,
) -> Coordinate {
    let mut new_x = start_velocity.0;
    let mut new_y = start_velocity.1;

    if probe.in_bounds(bounds) {
        println!("test");
        println!("{:?}", probe);

        new_y += 1;
    } else {
        if probe.position.0 < bounds.0 .0 {
            new_x += 1;
        } else if probe.position.0 > bounds.1 .0 {
            new_x -= 1;
        }

        if probe.position.1 < bounds.0 .1 && probe.velocity.0 == 0 {
            new_x += 1;
        } else if probe.position.1 < bounds.0 .1 && start_velocity.1 > 0 {
            new_y -= 1;
        }
    }

    (new_x, new_y)
}

fn get_test_input_field() -> (Coordinate, Coordinate) {
    ((20, -10), (30, -5))
}

fn get_input_field() -> (Coordinate, Coordinate) {
    ((169, -108), (206, -68))
}
