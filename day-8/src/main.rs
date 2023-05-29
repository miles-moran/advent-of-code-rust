use std::fs;
fn main() {
    // load arrow with parsed integers
    for x in (0..4).rev() {
        println!("{}", x);
    }
    let mut grid: Vec<Vec<u32>> = vec![];
    fs::read_to_string("input.txt")
        .expect("error reading file")
        .split("\n")
        .filter(|&r| r != "")
        .for_each(|r| {
            const RADIX: u32 = 10;
            grid.push(
                r.chars()
                    .map(|f| f.to_digit(RADIX).unwrap())
                    .collect::<Vec<_>>(),
            );
        });

    println!("{:?}", grid);
    let mut vis_count = 0;
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            let cur_height = grid[y][x];
            let mut is_visible = false;
            println!("{}, {}", x, y);
            // left
            if x != 0 {
                for l in (0..x - 1).rev() {
                    let comp_height = grid[y][l];
                    if comp_height < cur_height {
                        is_visible = true;
                    }
                }
            }
            // right
            if x != grid[y].len() {
                for r in (x + 1..grid[y].len()) {
                    let comp_height = grid[y][r];
                    if comp_height < cur_height {
                        is_visible = true;
                    }
                }
            }
            if y != 0 {
                for u in (0..y - 1).rev() {

                    let comp_height = grid[u][x];
                    if comp_height < cur_height {
                        is_visible = true;
                    }
                }
            }
            if y != grid.len() {
                for d in (y + 1..grid.len()) {
                    let comp_height = grid[d][x];
                    if comp_height < cur_height {
                        is_visible = true;
                    }
                }
            }
            if is_visible == true {
                vis_count += 1;
            }
        }
    }
    println!("-- {}", vis_count);
}
