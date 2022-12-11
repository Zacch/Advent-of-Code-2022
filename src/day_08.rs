use std::cmp::max;
use std::fs;

pub fn run() {
    println!("Day 8");
    let contents = fs::read_to_string("input/day_08.txt")
        .expect("Couldn't read the file");
    let lines:Vec<&str> = Vec::from_iter(contents.lines());

    let mut grid:Vec<Vec<u32>> =vec![];

    for line in lines.iter() {
        let mut row: Vec<u32> = vec![];
        for c in line.chars() {
            row.push(c.to_digit(10).unwrap());
        }
        grid.push(row);
    }

    let mut part1 = 0;
    let mut part2 = 0;
    for (r, row) in grid.iter().enumerate() {
        for (c, _) in row.iter().enumerate() {
            if is_visible(r, c, &grid) {
                part1 += 1;
            }
            part2 = max(part2, scenic_score(r, c, &grid));
        }
    }
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);

}

fn is_visible(r: usize, c: usize, grid:&Vec<Vec<u32>>) -> bool {
    if r == 0 || r == grid.len() - 1 { return true }
    if c == 0 || c == &grid[0].len() - 1 { return true }

    let tree = grid[r][c];
    let (mut vis_left, mut vis_right, mut vis_top, mut vis_bottom) = (true, true, true, true);
    for (col_index, grid_tree) in grid[r].iter().enumerate() {
        if col_index < c && *grid_tree >= tree {
            vis_left = false
        }
        if col_index > c && *grid_tree >= tree {
            vis_right = false
        }
    }
    for (row_index, row) in grid.iter().enumerate() {
        let grid_tree = row[c];
        if row_index < r && grid_tree >= tree {
            vis_top = false
        }
        if row_index > r && grid_tree >= tree {
            vis_bottom = false
        }
    }
        vis_top || vis_left || vis_right || vis_bottom
}

fn scenic_score(r: usize, c: usize, grid:&Vec<Vec<u32>>) -> i32 {
    if r == 0 || r == grid.len() - 1 { return 0 }
    if c == 0 || c == &grid[0].len() - 1 { return 0 }

    let row = &grid[r];
    let tree = row[c];

    let (mut left, mut top, mut right, mut bottom) = (0, 0, 0, 0);
    for i in (0..c).rev() {
        left += 1;
        if row[i] >= tree { break; }
    }

    for i in (c + 1)..row.len() {
        right += 1;
        if row[i] >= tree { break; }
    }

    for i in (0..r).rev() {
        top += 1;
        if grid[i][c] >= tree { break; }
    }

    for i in (r + 1)..grid.len() {
        bottom += 1;
        if grid[i][c] >= tree { break; }
    }

    left * top * right * bottom
}
