pub struct Part2;
use super::*;
impl Part2 {
    pub fn solve(&self, map: &str) {
        let (pos, mut map) = parse(map);
        let mut walker1 = Position::new(pos, Direction::R);
        // mark the loop

        while {
            walker1.walk(&mut map).is_ok()
        } {}

        // scanline algorithm
        scanline(map);
    }
}

fn scanline(map: Vec<Vec<Pipe>>) -> u32 {
    map.into_iter().map(walk_line).sum()
}

fn walk_line(line: Vec<Pipe>) -> u32 {
	let mut inside_loop = false;
	let mut area = Counter::new();
    dbg!(&line);

	for pipe in line {
		match pipe.0 {
			'.' if inside_loop => {},
			'.' if inside_loop => area.incr(),
		}
	}

    panic!();
    0
}

/// struct that is an iterator that counts up until [`u64::MAX`] is reached
struct Counter {
	step_size: i64,
	count: i64,
}

impl Counter {
	/// initializes with 0 and steps by 1
	fn new() -> Self {
		Self {
			step_size: 1,
			count: 0,
		}
	}
	
	/// starts with your value and steps by 1
	fn with_start(initial: i64) -> Self {
		Self {
			count: initial,
			step_size: 1
		}
	}

	fn step_by(&mut self, new_step: i64) {
		self.step_size = new_step;
	}

	fn incr(&mut self) -> Option<i64> {
		self.count.checked_add(self.step_size)
	}

	fn increase(&mut self) -> Option<i64> {
		self.count.checked_add(self.step_size)
	}
}







#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn with_test() {
        #[rustfmt::skip]
const TEST_DATA: &str = 
"...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";

        Part2.solve(TEST_DATA)
    }
}
