const RAW_DATA: &str = include_str!("../../input/day_13.txt");

fn main() {
    part_two();
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
fn part_two() {
    println!("Part 1");
    let data = load_data();

    let totals: Vec<_> = data
        .iter()
        .filter_map(|grid| {
            // println!("{:?}",grid);
            for i in 1..grid.len() {
                let first_rows: Vec<Vec<char>> = grid[..i].to_vec().into_iter().rev().collect();
                let end_rows: Vec<Vec<char>> = grid[i..].to_vec();
                let min_rows = first_rows.len().min(end_rows.len());
                let first_rows = &first_rows[..min_rows];
                let end_rows = &end_rows[..min_rows];
                let mut simon = 0;
                for (i, row) in first_rows.iter().enumerate() {
                    for (j, col) in row.iter().enumerate() {
                        if col == &'#' && end_rows[i][j] == '.'
                            || col == &'.' && end_rows[i][j] == '#'
                        {
                            simon += 1;
                        }
                    }
                }
                if simon == 1 {
                    return Some(i * 100);
                }
            }

            let column_grid = transpose(grid);

            for i in 1..column_grid.len() {
                let first_rows: Vec<Vec<char>> =
                    column_grid[..i].to_vec().into_iter().rev().collect();
                let end_rows: Vec<Vec<char>> = column_grid[i..].to_vec();
                let min_rows = first_rows.len().min(end_rows.len());
                let first_rows = &first_rows[..min_rows];
                let end_rows = &end_rows[..min_rows];
                let mut simon = 0;
                for (i, row) in first_rows.iter().enumerate() {
                    for (j, col) in row.iter().enumerate() {
                        if col == &'#' && end_rows[i][j] == '.'
                            || col == &'.' && end_rows[i][j] == '#'
                        {
                            simon += 1;
                        }
                    }
                }
                if simon == 1 {
                    return Some(i);
                }
            }
            None
        })
        .collect();
    println!("{totals:?}");
    println!("{:?}", totals.iter().sum::<usize>());
}
