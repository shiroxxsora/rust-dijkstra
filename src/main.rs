use std::collections::VecDeque;
use std::io::{self, Read};
use std::env;
use std::fs;

fn process_map(input: &str) -> String {
    let mut lines: Vec<String> = input.lines().map(|s| s.to_string()).collect();
    if lines.is_empty() {
        return input.to_string();
    }

    let height = lines.len();
    let width = lines.iter().map(|l| l.chars().count()).max().unwrap_or(0);
    if width == 0 {
        return input.to_string();
    }

    for line in &mut lines {
        let len = line.chars().count();
        if len < width {
            let mut s = String::with_capacity(width);
            s.push_str(line);
            for _ in 0..(width - len) {
                s.push(' ');
            }
            *line = s;
        }
    }

    let mut grid: Vec<Vec<char>> = lines.iter().map(|l| l.chars().collect()).collect();

    let mut start = None;
    let mut goal = None;
    for y in 0..height {
        for x in 0..width {
            match grid[y][x] {
                'i' => start = Some((x, y)),
                'O' => goal = Some((x, y)),
                _ => {}
            }
        }
    }

    let Some((sx, sy)) = start else { return lines.join("\n") };
    let Some((gx, gy)) = goal else { return lines.join("\n") };

    let mut visited = vec![vec![false; width]; height];
    let mut prev: Vec<Vec<Option<(usize, usize)>>> = vec![vec![None; width]; height];
    let mut q = VecDeque::new();
    visited[sy][sx] = true;
    q.push_back((sx, sy));

    let deltas: [(isize, isize); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];
    while let Some((x, y)) = q.pop_front() {
        if x == gx && y == gy {
            break;
        }
        for (dx, dy) in deltas {
            let nx = ((x as isize + dx).rem_euclid(width as isize)) as usize;
            let ny = ((y as isize + dy).rem_euclid(height as isize)) as usize;
            if !visited[ny][nx] && grid[ny][nx] != '#' {
                visited[ny][nx] = true;
                prev[ny][nx] = Some((x, y));
                q.push_back((nx, ny));
            }
        }
    }

    if !visited[gy][gx] {
        return lines.join("\n");
    }

    let mut cx = gx;
    let mut cy = gy;
    while !(cx == sx && cy == sy) {
        if !(cx == gx && cy == gy) && !(cx == sx && cy == sy) {
            if grid[cy][cx] != 'i' && grid[cy][cx] != 'O' && grid[cy][cx] != '#' {
                grid[cy][cx] = '.';
            }
        }
        let (px, py) = match prev[cy][cx] {
            Some(p) => p,
            None => break,
        };
        cx = px;
        cy = py;
    }

    grid.into_iter().map(|row| row.into_iter().collect()).collect::<Vec<String>>().join("\n")
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = if args.len() > 1 {
        if args[1] == "-" {
            let mut s = String::new();
            io::stdin().read_to_string(&mut s).ok();
            s
        } else {
            fs::read_to_string(&args[1]).unwrap_or_else(|_| {
                let mut s = String::new();
                io::stdin().read_to_string(&mut s).ok();
                s
            })
        }
    } else {
        let mut s = String::new();
        io::stdin().read_to_string(&mut s).ok();
        s
    };
    let output = process_map(&input);
    print!("{}", output);
}

#[cfg(test)]
mod tests {
    use super::process_map;

    #[test]
    fn finds_path_and_marks_dots() {
        let input = "##    #\n#  #i #\n#  O## \n   #   ";
        let expected = "##... #\n# .#i #\n# .O## \n   #   ";
        let out = process_map(input);
        assert_eq!(out, expected);
    }

    #[test]
    fn returns_original_if_no_path() {
        let input = "###\n#i#\n#O#";
        let out = process_map(input);
        assert_eq!(out, input);
    }

    #[test]
    fn wraps_toroidally() {
        let input = "i  #O";
        let out = process_map(input);
        if out.contains('.') {
            assert!(out.starts_with('i'));
            assert!(out.ends_with('O'));
        } else {
            assert_eq!(out, input);
        }
    }
}
