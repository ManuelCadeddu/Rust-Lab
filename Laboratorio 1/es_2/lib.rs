// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(PartialEq, Eq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}
struct Position {
    x: i32,
    y: i32,
}
pub struct Robot {
    p: Position,
    d: Direction,
}

impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        Robot { p : Position {x,y}, d }
    }

    #[must_use]
    pub fn turn_right(self) -> Self {
        match self.d {
            Direction::North => Self{ p: self.p, d: Direction::East },
            Direction::East => Self{ p: self.p, d: Direction::South },
            Direction::South => Self{ p: self.p, d: Direction::West },
            Direction::West => Self{ p: self.p, d: Direction::North },
        }
    }

    #[must_use]
    pub fn turn_left(self) -> Self {
        match self.d {
            Direction::North => Self{ p: self.p, d: Direction::West },
            Direction::East => Self{ p: self.p, d: Direction::North },
            Direction::South => Self{ p: self.p, d: Direction::East },
            Direction::West => Self{ p: self.p, d: Direction::South },
        }
    }

    #[must_use]
    pub fn advance(self) -> Self {
        match self.d {
            Direction::North => Self { p: Position { x: self.p.x, y: self.p.y + 1 }, d: self.d },
            Direction::East => Self { p: Position { x: self.p.x + 1, y: self.p.y }, d: self.d },
            Direction::South => Self { p: Position { x: self.p.x, y: self.p.y - 1 }, d: self.d },
            Direction::West => Self { p: Position { x: self.p.x - 1, y: self.p.y }, d: self.d },
        }
    }

    #[must_use]
    pub fn instructions(self, instructions: &str) -> Self {
        let mut robot = self;
        for c in instructions.chars() {
            match c {
                'L' => robot = robot.turn_left(),
                'R' => robot = robot.turn_right(),
                'A' => robot = robot.advance(),
                _ => (),
            }
        }
        robot
    }

    pub fn position(&self) -> (i32, i32) {
        (self.p.x, self.p.y)
    }

    pub fn direction(&self) -> &Direction {
        &self.d
    }
}
