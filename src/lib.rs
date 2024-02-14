use std::cmp::min;

pub mod part2;

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
fn test1() -> Result<(), ()> {
    let (pos, mut parsed) = parse(TEST_DATA[1]);
    let mut curr_pos0 = Position::new(pos, Direction::R);
    let mut curr_pos1 = Position::new(pos, Direction::D);

    curr_pos0.walk(&mut parsed)?;
    curr_pos1.walk(&mut parsed)?;

    while (curr_pos0.x, curr_pos0.y) != (curr_pos1.x, curr_pos1.y) {
        curr_pos0.walk(&mut parsed)?;
        curr_pos1.walk(&mut parsed)?;
    }

    println!(
        "Max seen: {}",
        max(curr_pos0.max_steps_seen, curr_pos1.max_steps_seen)
    );

    Ok(())
}

pub struct Part1;

impl Part1 {
    pub fn solve(&self) {
        let (pos, mut parsed) = parse(std::fs::read_to_string("data.txt").unwrap().as_str());
        let mut curr_pos0 = Position::new(pos, Direction::R);
        let mut curr_pos1 = Position::new(pos, Direction::L);

        curr_pos1.walk(&mut parsed).unwrap();
        // curr_pos0.walk(&mut parsed);

        while (curr_pos0.x, curr_pos0.y) != (curr_pos1.x, curr_pos1.y) {
            curr_pos0.walk(&mut parsed).unwrap();
            curr_pos1.walk(&mut parsed).unwrap();
        }

        println!(
            "Max seen: {}",
            max(curr_pos0.max_steps_seen, curr_pos1.max_steps_seen)
        );
    }
}

pub fn replace_start_with_pipe(map: &mut Vec<Vec<Pipe>>, start_pos: (isize, isize)) {
    fn get(map: &Vec<Vec<Pipe>>, pos: (isize, isize)) -> Option<&Pipe> {
        map.get(pos.1 as usize).and_then(|x| x.get(pos.0 as usize))
    }
    // get the 4 directly adjacent pipes and turn them into adjacency maps.
    let mut adj_pipes = Vec::with_capacity(4);
    // order the elements in the vec so it can be indexed by the direction from the start position
    // up
    get(map, (start_pos.0, start_pos.1 - 1)).and_then(|x| Some(adj_pipes.push(*x)));
    get(map, (start_pos.0 + 1, start_pos.1)).and_then(|x| Some(adj_pipes.push(*x)));
    get(map, (start_pos.0, start_pos.1 + 1)).and_then(|x| Some(adj_pipes.push(*x)));
    get(map, (start_pos.0 - 1, start_pos.1)).and_then(|x| Some(adj_pipes.push(*x)));

    dbg!(&adj_pipes);
}

/// parses `&str` to a map of pipes,
/// each holding the pipes shape and the distance to the starting position
///
/// ## Returns
/// * starting position as `(x, y)` and
/// * the map as `Vec<Vec<Pipe>>`
fn parse(input: &str) -> ((isize, isize), Vec<Vec<Pipe>>) {
    let mut vec: Vec<Vec<Pipe>> = vec![];
    let mut start_pos = None;

    for (i, line) in input.lines().enumerate() {
        let x = line
            .chars()
            .enumerate()
            .map(|(j, ch)| {
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

    (start_pos, vec)
}

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    U = 0,
    R,
    D,
    L = 3,
}

#[derive(Debug, Clone, Copy)]
pub struct Position {
    // todo: change to usize or u32
    pub x: isize,
    pub y: isize,
    pub direction: Direction,
    pub steps_taken: u32,
    pub max_steps_seen: u32,
}
use std::cmp::max;
use std::fmt::Debug;
impl Position {
    fn walk(&mut self, map: &mut Vec<Vec<Pipe>>) -> Result<(), ()> {
        // get the next pipe by calculating the offset
        let (dx, dy) = self.direction_to_offset();

        let next_pipe = map[(self.y + dy) as usize][(self.x + dx) as usize];

        // update direction
        let get_new_direction = next_pipe.to_direction()?;
        self.direction = get_new_direction(self.direction);

        // update position
        self.x += dx;
        self.y += dy;

        // set current pipe steps to distance from Start
        self.steps_taken += 1;
        let x = map
            .get_mut(self.y as usize)
            .unwrap()
            .get_mut(self.x as usize)
            .unwrap();
        x.1 = max(x.1, self.steps_taken);

        self.max_steps_seen = x.1;
        Ok(())
        // dbg!(&x);
    }

    fn direction_to_offset(&mut self) -> (isize, isize) {
        match self.direction {
            Direction::U => (0, -1),
            Direction::D => (0, 1),
            Direction::L => (-1, 0),
            Direction::R => (1, 0),
        }
    }

    fn new(coords: (isize, isize), dir: Direction) -> Self {
        let (x, y) = coords;

        Self {
            x,
            y,
            direction: dir,
            steps_taken: 0,
            max_steps_seen: 0,
        }
    }
}

/// holds the pipes shape as `char` and the steps it has taken to get there.
///
/// `u32` Not zero if inside the loop
#[derive(Clone, Copy)]
pub struct Pipe(char, pub u32);

impl From<char> for Pipe {
    fn from(ch: char) -> Self {
        Self(ch, 0)
    }
}

impl Debug for Pipe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let tmp = format!("{}({})", self.0, self.1);
        f.write_str(&tmp)?;
        Ok(())
    }
}

impl Pipe {
    /// returns a list of the pipes connections. Index by
    ///  `Direction as u8`
    /// to get the connection as a `boolean`
    fn adjacency_map(ch: Pipe) -> Result<[bool; 4], ()> {
        match ch.0 {
            '|' => Ok([true, false, true, false]),
            '-' => Ok([false, true, false, true]),
            'L' => Ok([true, true, false, false]),
            'J' => Ok([true, false, false, true]),
            'F' => Ok([false, true, true, true]),
            _ => Err(()),
        }
    }

    fn from(ch: char) -> Self {
        Self(ch, 0)
    }
    // /// returns a function that takes a direction and returns a new direction based on the pipes shape
    fn to_direction(&self) -> Result<Box<dyn Fn(Direction) -> Direction>, ()> {
        match self.0 {
            '|' => Ok(Box::new(|d: Direction| match d {
                Direction::U => Direction::U,
                Direction::D => Direction::D,
                _ => panic!("{d:?}"),
            })),
            '-' => Ok(Box::new(|d: Direction| match d {
                Direction::L => Direction::L,
                Direction::R => Direction::R,
                _ => panic!("{d:?}"),
            })),

            'L' => Ok(Box::new(|d| match d {
                Direction::L => Direction::U,
                Direction::D => Direction::R,
                _ => panic!("{d:?}"),
            })),
            'F' => Ok(Box::new(|d| match d {
                Direction::L => Direction::D,
                Direction::U => Direction::R,
                _ => panic!("{d:?}"),
            })),

            'J' => Ok(Box::new(|d| match d {
                Direction::R => Direction::U,
                Direction::D => Direction::L,
                _ => panic!("{d:?}"),
            })),
            '7' => Ok(Box::new(|d| match d {
                Direction::U => Direction::L,
                Direction::R => Direction::D,
                _ => panic!("{d:?}"),
            })),

            ch => Err(()),
        }
    }
}
