pub struct Part2;
use super::*;
impl Part2 {
    pub fn solve(&self, map: &str) -> u32 {
        let (start_pos, mut map) = parse(map);
        // replace Start position with actual pipe
        let start_pipe = replace_start_with_pipe(&mut map, start_pos);
        let start_direction = Direction::start_direction(start_pipe.0);
        let mut walker1 = Position::new(start_pos, start_direction);

        // mark the loop by walking
        while walker1.walk(&mut map).is_ok() && (walker1.x, walker1.y) != start_pos {}
		let sum: usize = map.iter().map(|line| line.iter().filter(|p| p.1 != 0).count()).sum();
		dbg!(sum);
        // scanline algorithm
        dbg!(scanline(map))
    }
}
/// use the even odd rule to count the area inside the loop
///
/// optimised using the scanline algorithm
fn scanline(map: Vec<Vec<Pipe>>) -> u32 {
    dbg!(map.into_iter().map(walk_line).collect::<Vec<_>>()).iter().sum()
}

/// walk a line, using the even odd rule to count the area that's inside the loop
fn walk_line(line: Vec<Pipe>) -> u32 {
    let mut inside_loop = false;
    let mut area = Counter::new();
    let mut last_corner = None;
    // debug line
    // line.iter().for_each(|item| print!("{item:?}"));

    for pipe in line {
        match pipe.0 {
            '.' => {
                if inside_loop {
                    last_corner = None;
                    area.increase()
                }
            }

            // all pipes that are part of the loop
            ch if pipe.1 > 0 => match ch {
                '|' => inside_loop = !inside_loop,

                // match the pipe. if the pipe can close a loop, look at the previous corner
                // if the pipe forms a U, don't change  `inside_loop`

                // opening pipes that are corners
                'F' | 'L' => last_corner = Some(pipe.0),
                // F and J don't form a U turn, so they change `inside_loop`
                'J' if last_corner == Some('F') => inside_loop = !inside_loop,

                '7' if last_corner == Some('L') => inside_loop = !inside_loop,

                // '-' if inside_loop => area.increase(),
                _ => (),
            },

            _ => if inside_loop { area.increase() },
        }
    }

    area.count as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn with_test() {
        #[rustfmt::skip]
const TEST_DATA: &str = 
".S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7F--J|.
.|..||...|.
.L--JL---J.";

        assert_eq!(Part2.solve(TEST_DATA), 5);
    }

    #[test]
    fn new() {
        const DATA: &str = ".....
.S-7.
.|.|.
.L-J.
.....";
        let res = Part2.solve(DATA);
        assert_eq!(res, 1);
    }

    #[test]
    fn example() {
        const DATA: &str = "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L
";
        let res = Part2.solve(DATA);
        assert_eq!(res, 10);
    }

    #[test]
    fn with_test3() {
        let res = vec![".|.|.L---J..F---7|.|L7.F-J...|..|"
            .chars()
            .map(|ch| {
                let x = if ch != '.' { 1 } else { 0 };
                Pipe(ch, x)
            })
            .collect()];
        let res = scanline(res);
        assert_eq!(res, 5);
    }
}
