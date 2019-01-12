pub fn hello_world(name: String) -> String {
    if name == String::from("") {
        String::from("hello world")
    } else {
        String::from("hello ") + &name
    }
}

//let grid_size = 5;

type coord2d = (i32, i32);

 # [derive(PartialEq, Debug)]
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

 # [derive(Clone)]
struct Rover {
    name: String,
    orientation: Cardinals,
    position: coord2d,
    grid_size: i32,
}

enum DirectionY {
    Forward,
    Backward,
}

enum DirectionX {
    Left,
    Right
}
 # [derive(Clone)]
struct Grid {
    size: i32,
    obstacles: Vec<coord2d>
}

trait Move {
    fn move_rover(&self, dy: DirectionY) -> Rover;
    fn turn_rover(&self, dx: DirectionX) -> Rover;
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

fn opposite(c: &Cardinals) -> Cardinals {
    match *c {
        Cardinals::North => Cardinals::South,
        Cardinals::South => Cardinals::North,
        Cardinals::East => Cardinals::West,
        Cardinals::West => Cardinals::East,
    }
}

fn has_obstacle(grid: Grid, pos: coord2d) -> bool {
    grid.obstacles.contains(&pos)
}

fn command(r : Rover, cardinal : Cardinals, grid: Grid) -> Rover {
    let pending_rover =if r.orientation == cardinal {
        r.move_rover(DirectionY::Forward)
    } else if r.orientation == opposite(&cardinal) {
        r.move_rover(DirectionY::Backward)
    }
    else if cardinal == r.orientation.next() {
        r.turn_rover(DirectionX::Right).move_rover(DirectionY::Forward)
    } else {
        r.turn_rover(DirectionX::Left).move_rover(DirectionY::Forward)
    };
    if has_obstacle(grid, pending_rover.position) {
        r.clone()
    }else{
        pending_rover
    }
}

impl Move for Rover {
    fn move_rover(&self, dy: DirectionY) -> Rover {
        if let DirectionY::Forward = dy {
            Rover {
                name: self.name.clone(),
                orientation: self.orientation.clone(),
                grid_size: self.grid_size,
                position: match self.orientation {
                    Cardinals::North => (self.position.0, (self.position.1 + 1) % self.grid_size),
                    Cardinals::South => (self.position.0, (self.position.1 - 1) % self.grid_size),
                    Cardinals::East => ((self.position.0 + 1)% self.grid_size , self.position.1),
                    Cardinals::West => ((self.position.0 - 1)% self.grid_size, self.position.1),
                }
            }
        } else {
            Rover {
                name: self.name.clone(),
                orientation: self.orientation.clone(),
                grid_size: self.grid_size,
                position: match self.orientation {
                    Cardinals::North => (self.position.0, (self.position.1 - 1)% self.grid_size),
                    Cardinals::South => (self.position.0, (self.position.1 + 1)% self.grid_size),
                    Cardinals::East => ((self.position.0 - 1)% self.grid_size , self.position.1),
                    Cardinals::West => ((self.position.0 + 1)% self.grid_size, self.position.1),
                }
            }
        }
    }

    fn turn_rover(&self, dx: DirectionX) -> Rover {
        if let DirectionX::Right = dx {
            Rover {
                name: self.name.clone(),
                orientation : self.orientation.next(),
                grid_size: self.grid_size,
                position: self.position
            }
        } else {
            Rover {
                name: self.name.clone(),
                orientation : self.orientation.previous(),
                grid_size: self.grid_size,
                position: self.position
            }
        }
    }
}


fn char_to_cardinals (commands: Vec<char>) -> Vec<Cardinals> {
    let slice_commands = commands.as_slice();
    let mut cardinals: Vec<Cardinals> = Vec::new();
    for com in slice_commands.iter() {
        match com {
                'N' => cardinals.push(Cardinals::North),
                'S' => cardinals.push(Cardinals::South),
                'E' => cardinals.push(Cardinals::East),
                'W' => cardinals.push(Cardinals::West),
                _ => (),
        }
    };
    cardinals
}

fn commands (commands: Vec<Cardinals>, r: Rover, grid: Grid) -> Rover {
    let slice_commands = commands.as_slice();
    let mut inter_rover = r;
    for com in slice_commands.iter() {
        inter_rover = command(inter_rover.clone(), com.clone(), grid.clone());
    }
    inter_rover
}

#[cfg(test)]
mod tests {
    use super::*;

    fn give_rover(orientation : Cardinals, position: coord2d) -> Rover {
        Rover {
            name: String::from("curiosity"),
            orientation,
            grid_size: 6,
            position: position
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
        let rover = give_rover(Cardinals::North, (0, 0));
        assert_eq!(String::from("curiosity"), rover.name);
    }

    #[test]
    fn given_north_when_north_should_forward() {
        let rover_start = give_rover(Cardinals::North,  (0, 0));
        let grid = Grid {
            size: 6,
            obstacles: Vec::new()
        };
        let rover_end = command(rover_start, Cardinals::North, grid);
        assert_eq!((0,1), rover_end.position);
    }

    #[test]
    fn given_south_when_north_4x4_should_backward() {
        let rover_start = give_rover(Cardinals::South,  (4, 4));
        let grid = Grid {
            size: 6,
            obstacles: Vec::new()
        };
        let rover_end = command(rover_start, Cardinals::North, grid);
        assert_eq!((4,5), rover_end.position);
    }

    #[test]
    fn given_south_when_north_5x5_should_backward() {
        let rover_start = give_rover(Cardinals::South,  (5, 5));
        let grid = Grid {
            size: 6,
            obstacles: Vec::new()
        };
        let rover_end = command(rover_start, Cardinals::North, grid);
        assert_eq!((5,0), rover_end.position);
    }

    #[test]
    fn given_north_when_east_should_turn_right_forward() {
        let rover_start = give_rover(Cardinals::North,  (0, 0));
        let grid = Grid {
            size: 6,
            obstacles: Vec::new()
        };
        let rover_end = command(rover_start, Cardinals::East, grid);
        // let rover_end = rover_start.turn_rover(DirectionX::Right);
        assert_eq!((1, 0), rover_end.position);
        // assert_eq!(Cardinals::East, rover_end.orientation);
    }

    #[test]
    fn given_north_when_north_east_should_forward_turn_right_forward() {
        let rover_start = give_rover(Cardinals::North,  (0, 0));
        let rover_end = rover_start.move_rover(DirectionY::Forward).turn_rover(DirectionX::Right).move_rover(DirectionY::Forward);
        // let rover_end = rover_start.turn_rover(DirectionX::Right);
        assert_eq!((1, 1), rover_end.position);
        // assert_eq!(Cardinals::East, rover_end.orientation);
    }

    //

    #[test]
    fn given_east_when_north_should_turn_left_forward() {
        let rover_start = give_rover(Cardinals::East,  (0, 0));
        let grid = Grid {
            size: 6,
            obstacles: Vec::new()
        };
        let rover_end = command(rover_start, Cardinals::North, grid);
        assert_eq!((0,1), rover_end.position);
    }

    #[test]
    fn given_east_when_east_north_should_forward_turn_left_forward() {
        let rover_start = give_rover(Cardinals::East,  (0, 0));
        let rover_end = rover_start.move_rover(DirectionY::Forward).turn_rover(DirectionX::Left).move_rover(DirectionY::Forward);
        // let rover_end = rover_start.turn_rover(DirectionX::Right);
        assert_eq!((1, 1), rover_end.position);
        // assert_eq!(Cardinals::East, rover_end.orientation);
    }

    #[test]
    fn given_north_when_north_south_should_forward_backward() {
        let rover_start = give_rover(Cardinals::North,  (0, 0));
        let rover_end = rover_start.move_rover(DirectionY::Forward).move_rover(DirectionY::Backward);
        // let rover_end = rover_start.turn_rover(DirectionX::Right);
        assert_eq!((0, 0), rover_end.position);
        // assert_eq!(Cardinals::East, rover_end.orientation);
    }

    #[test]
    fn given_north_when_north_with_obstacle_shouldnt_move() {
        let rover_start = give_rover(Cardinals::North,  (0, 0));
        let grid = Grid {
            size: 6,
            obstacles: vec![(0, 1)]
        };
        let rover_end = command(rover_start, Cardinals::North, grid);
        assert_eq!((0,0), rover_end.position);
    }

    #[test]
    fn given_commands_should_return_vec_cardinals () {
        let cardinals = vec![Cardinals::North, Cardinals::East, Cardinals::South, Cardinals::West];
        let commands = vec!['N', 'E', 'S', 'W', 'K'];
        assert_eq!(cardinals, char_to_cardinals(commands));
    }

    #[test]
    fn given_commands_should_move_rover () {
        let rover_start = give_rover(Cardinals::North,  (0, 0));
        let grid = Grid {
            size: 6,
            obstacles: Vec::new()
        };
        let orders = vec![Cardinals::North, Cardinals::East, Cardinals::South, Cardinals::West];
        assert_eq!((0, 0), commands(orders, rover_start, grid).position);
    }

}
