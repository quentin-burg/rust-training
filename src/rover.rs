pub fn hello_world(name: String) -> String {
    if name == String::from("") {
        String::from("hello world")
    } else {
        String::from("hello ") + &name
    }
}

type coord2D = (i32, i32);

enum Cardinals {
    North,
    East,
    West,
    South
}


trait Clockwise {
    fn next(&self) -> Cardinals;
    fn previous(&self) -> Cardinals;
}

impl Clockwise for Cardinals {
    fn next(&self) -> Cardinals {
        match self {
            Cardinals::North => Cardinals::East,
            Cardinals::South => Cardinals::West,
            Cardinals::East => Cardinals::South,
            Cardinals::West => Cardinals::North,
        }
    }

    fn previous(&self) -> Cardinals {
        match self {
            Cardinals::North => Cardinals::West,
            Cardinals::South => Cardinals::East,
            Cardinals::East => Cardinals::North,
            Cardinals::West => Cardinals::South,
        }
    }
}

struct Rover {
    name: String,
    orientation: Cardinals,
    position: coord2D
}

enum Direction_y {
    Forward,
    Backward,
}

enum Direction_x {
    Left,
    Right
}

trait Move {
    fn move_rover(&self, dy: Direction_y) -> Rover;
    fn turn_rover(&self, dx: Direction_x) -> Rover;
}

impl Clone for Cardinals {
    fn clone(&self) -> Cardinals {
        match self {
            Cardinals::North => Cardinals::North,
            Cardinals::South => Cardinals::South,
            Cardinals::East => Cardinals::East,
            Cardinals::West => Cardinals::West,
        }
    }
}

fn command(r : Rover, cardinal : Cardinals) -> Rover {
    unimplemented!();
}

impl Move for Rover {
    fn move_rover(&self, dy: Direction_y) -> Rover {
        if let Direction_y::Forward = dy {
            Rover {
                name: self.name.clone(),
                orientation: self.orientation.clone(),
                position: match self.orientation {
                    Cardinals::North => (self.position.0, self.position.1 + 1),
                    Cardinals::South => (self.position.0, self.position.1 - 1),
                    Cardinals::East => (self.position.0 + 1 , self.position.1),
                    Cardinals::West => (self.position.0 - 1, self.position.1),
                }
            }
        } else {
            Rover {
                name: self.name.clone(),
                orientation: self.orientation.clone(),
                position: match self.orientation {
                    Cardinals::North => (self.position.0, self.position.1 - 1),
                    Cardinals::South => (self.position.0, self.position.1 + 1),
                    Cardinals::East => (self.position.0 - 1 , self.position.1),
                    Cardinals::West => (self.position.0 + 1, self.position.1),
                }
            }
        }
    }

    fn turn_rover(&self, dx: Direction_x) -> Rover {
        if let Direction_x::Right = dx {
            Rover {
                name: self.name.clone(),
                orientation : self.orientation.next(),
                position: self.position
            }
        } else {
            Rover {
                name: self.name.clone(),
                orientation : self.orientation.previous(),
                position: self.position
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn give_rover(orientation : Cardinals) -> Rover {
        Rover {
            name: String::from("curiosity"),
            orientation,
            position: (0,0)
        }
    }

    #[test]
    fn given_empty_then_return_hello_world() {
        let h1 = hello_world(String::from(""));
        assert_eq!(String::from("hello world"), h1);
    }

    #[test]
    fn given_cutii_then_return_hello_cutii() {
        let h2 = hello_world(String::from("cutii"));
        assert_eq!(String::from("hello cutii"), h2);
    }

    #[test]
    fn should_initialize_rover_named_curiosity(){
        let rover = give_rover(Cardinals::North);
        assert_eq!(String::from("curiosity"), rover.name);
    }

    #[test]
    fn given_north_when_north_should_forward() {
        let rover_start = give_rover(Cardinals::North);
        let rover_end = command(rover_start, Cardinals::North);
        assert_eq!((0,1), rover_end.position);
    }

    #[test]
    fn given_north_when_east_should_turn_right_forward() {
        let rover_start = give_rover(Cardinals::North);
        let rover_end = command(rover_start, Cardinals::East);
        // let rover_end = rover_start.turn_rover(Direction_x::Right);
        assert_eq!((1, 0), rover_end.position);
        // assert_eq!(Cardinals::East, rover_end.orientation);
    }

    #[test]
    fn given_north_when_north_east_should_forward_turn_right_forward() {
        let rover_start = give_rover(Cardinals::North);
        let rover_end = rover_start.move_rover(Direction_y::Forward).turn_rover(Direction_x::Right).move_rover(Direction_y::Forward);
        // let rover_end = rover_start.turn_rover(Direction_x::Right);
        assert_eq!((1, 1), rover_end.position);
        // assert_eq!(Cardinals::East, rover_end.orientation);
    }

    //

    #[test]
    fn given_east_when_north_should_turn_left_forward() {
        let rover_start = give_rover(Cardinals::East);
        let rover_end = command(rover_start, Cardinals::North);
        assert_eq!((0,1), rover_end.position);
    }

    #[test]
    fn given_east_when_east_north_should_forward_turn_left_forward() {
        let rover_start = give_rover(Cardinals::East);
        let rover_end = rover_start.move_rover(Direction_y::Forward).turn_rover(Direction_x::Left).move_rover(Direction_y::Forward);
        // let rover_end = rover_start.turn_rover(Direction_x::Right);
        assert_eq!((1, 1), rover_end.position);
        // assert_eq!(Cardinals::East, rover_end.orientation);
    }

    #[test]
    fn given_north_when_north_south_should_forward_backward() {
        let rover_start = give_rover(Cardinals::North);
        let rover_end = rover_start.move_rover(Direction_y::Forward).move_rover(Direction_y::Backward);
        // let rover_end = rover_start.turn_rover(Direction_x::Right);
        assert_eq!((0, 0), rover_end.position);
        // assert_eq!(Cardinals::East, rover_end.orientation);
    }

}
