pub struct Part2;
use super::*;
impl Part2 {
    pub fn solve(&self, map: &str) {
        let (start_pos, mut map) = parse(map);
        let mut walker1 = Position::new(start_pos, Direction::R);
        // replace Start position with actual pipe
        replace_start_with_pipe(&mut map, start_pos);
        // mark the loop by walking
        while { walker1.walk(&mut map).is_ok() && (walker1.x, walker1.y) != start_pos } {}

        // scanline algorithm
        dbg!(scanline(map));
    }
}
/// use the even odd rule to count the area inside the loop
///
/// optimised using the scanline algorithm
fn scanline(map: Vec<Vec<Pipe>>) -> u32 {
    map.into_iter().map(walk_line).sum()
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

            // always skip horizontal pipes
            // they cant change the `inside_loop` state
            '-' => continue,
            // all pipes that are part of the loop
            ch if pipe.1 > 0 => match ch {
                '|' => inside_loop = !inside_loop,

                // match the pipe. if the pipe can close a loop, look at the previous corner
                // if the pipe forms a U, dont change ̀`inside_loop`

                // opening pipes that are corners
                'F' | 'L' => last_corner = Some(pipe.0),
                // F and J dont form a U turn, so they change `inside_loop`
                'J' => {
                    if last_corner == Some('F') {
                        inside_loop = !inside_loop
                    } else {
                        // println!("last_corner: {last_corner:?}, pipe: {pipe:?}, inside_loop: {inside_loop}",);
                    }
                }
                '7' => {
                    if last_corner == Some('L') {
                        inside_loop = !inside_loop
                    } else {
                        // println!("last_corner: {last_corner:?}, pipe: {pipe:?}, inside_loop: {inside_loop}",);
                    }
                }

                ch => println!("unknown pipe: {ch:?}"),
            },
            _ => area.increase(),
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
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.";

        Part2.solve(TEST_DATA)
    }

    #[test]
    fn with_test2() {
        let res = scanline(vec![".|.|.L---J..F---7|.|L7.F-J...|..|"
            .chars()
            .map(|ch| {
                let x = if ch != '.' { 1 } else { 0 };
                Pipe(ch, x)
            })
            .collect()]);
        assert_eq!(res, 5);
    }
}
