#[rustfmt::skip]
pub const TEST_DATA: [&str; 2] = 
    ["-L|F7
7S-7|
L|7||
-L-J|
L|-JF",
    "7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ"];

#[test]
fn test1() {
    let (pos, parsed) = parse(TEST_DATA[1]);
    let mut curr_pos = Position::new(pos);
	
    loop {
        curr_pos.walk(&parsed)
    }
}

pub struct Part1;

impl Part1 {
    fn solution() {}
}

fn parse(input: &str) -> ((isize, isize), Vec<Vec<Pipe>>) {
    let mut vec: Vec<Vec<Pipe>> = vec![];
    let mut start_pos = None;

    for (i, line) in input.lines().enumerate() {
        let x = line
            .chars()
            .enumerate()
            .map(|(j, ch)| {
				dbg!(ch);
                match ch {
                    'S' => start_pos = Some((j as isize, i as isize)),
                    _ => (),
                }
                Pipe::from(ch)
            })
            .collect();

        vec.push(x)
    }

    let Some(start_pos) = start_pos else {
        panic!("No starting position")
    };

    return (start_pos, vec);
}

#[derive(Debug, Clone, Copy)]
pub struct Pipe(char);

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    U,
    D,
    L,
    R,
}

pub struct Position {
    pub x: isize,
    pub y: isize,
    pub direction: Direction,
}

impl Position {
    fn walk(&mut self, map: &Vec<Vec<Pipe>>) {
        // get the next pipe by calculating the offset
        let (dx, dy): (isize, isize) = match self.direction {
            Direction::U => (0, 1),
            Direction::D => (0, -1),
            Direction::L => (-1, 0),
            Direction::R => (1, 0),
        };

        let next_pipe = map[(self.y + dy) as usize][(self.y + dy) as usize];
        dbg!(next_pipe);
        self.x += dx;
        self.y += dy;
    }

    fn new(coords: (isize, isize)) -> Self {
        let (x, y) = coords;

        Self {
            x,
            y,
            direction: Direction::R,
        }
    }
}

impl Pipe {
    fn from(ch: char) -> Self {
        Self(ch)
    }
    /// returns a function that takes a direction and returns a new direction
    pub fn to_direction(&self) -> impl Fn(Direction) -> Direction {
        match self.0 {
            '|' => |d| match d {
                Direction::U => Direction::D,
                Direction::D => Direction::U,
                _ => panic!("{d:?}"),
            },
            '-' => |d| match d {
                Direction::L => Direction::R,
                Direction::R => Direction::L,
                _ => panic!("{d:?}"),
            },
            'L' => |d| match d {
                Direction::L => Direction::U,
                Direction::D => Direction::R,
                _ => panic!("{d:?}"),
            },
            'J' => |d| match d {
                Direction::R => Direction::U,
                Direction::D => Direction::L,
                _ => panic!("{d:?}"),
            },
            '7' => |d| match d {
                Direction::U => Direction::L,
                Direction::R => Direction::D,
                _ => panic!("{d:?}"),
            },
            'F' => |d| match d {
                Direction::L => Direction::D,
                Direction::D => Direction::R,
                _ => panic!("{d:?}"),
            },
            _ => panic!("Unknown character"),
        }
    }
}
