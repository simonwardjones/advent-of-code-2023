const RAW_DATA: &str = include_str!("../../input/day_13.txt");

fn main() {
    part_one();
}

fn load_data() -> Vec<Vec<Vec<char>>> {
    let grids = RAW_DATA
        .trim()
        .split("\n\n")
        .map(|raw_grid| {
            raw_grid
                .split("\n")
                .map(|row| row.chars().collect())
                .collect()
        })
        .collect();
    grids
}

fn transpose(grid: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut new_grid = vec![vec!['.'; grid.len()]; grid[0].len()];
    for (i, row) in grid.iter().enumerate() {
        for (j, col) in row.iter().enumerate() {
            new_grid[j][i] = *col;
        }
    }
    new_grid
}

#[allow(dead_code, unused_variables)]
fn part_one() {
    println!("Part 1");
    let data = load_data();

    let mut total = 0;

    data.iter().for_each(|grid| {
        // println!("{:?}",grid);
        for i in 1..grid.len() {
            let first_rows: Vec<Vec<char>> = grid[..i].to_vec().into_iter().rev().collect();
            let end_rows: Vec<Vec<char>> = grid[i..].to_vec();
            let min_rows = first_rows.len().min(end_rows.len());
            if first_rows[..min_rows] == end_rows[..min_rows] {
                total += i * 100
            }
        }

        let column_grid = transpose(grid);

        for i in 1..column_grid.len() {
            let first_rows: Vec<Vec<char>> = column_grid[..i].to_vec().into_iter().rev().collect();
            let end_rows: Vec<Vec<char>> = column_grid[i..].to_vec();
            let min_rows = first_rows.len().min(end_rows.len());
            if first_rows[..min_rows] == end_rows[..min_rows] {
                total += i
            }
        }
    });
    println!("{total:?}");
}
