pub fn satisfies_conditions(grid: Vec<Vec<i32>>) -> bool {
    let n = grid[0].len();
    for x in 1..n {
        if grid[0][x - 1] == grid[0][x] {
            return false;
        }
    }

    let m = grid.len();
    for x in 0..n {
        for y in 1..m {
            if grid[0][x] != grid[y][x] {
                return false;
            }
        }
    }
    true
}