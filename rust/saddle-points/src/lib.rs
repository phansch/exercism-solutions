pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let mut coordinates: Vec<(usize, usize)> = vec![];

    for (row_index, row) in input.iter().enumerate() {
        if row.is_empty() { continue; }

        for (column_index, _item) in row.iter().enumerate() {
            if is_saddle_point(input, row_index, column_index) {
                coordinates.push((row_index, column_index));
            }
        }
    }
    coordinates
}

fn is_saddle_point(input: &[Vec<u64>], row_index: usize, column_index: usize) -> bool {
    let row_max = row_max(&input[row_index]);
    let col_min = column_min(input, column_index);

    if row_max == col_min {
        return true
    }
    false
}

fn row_max(row: &Vec<u64>) -> u64 {
    row.iter().max().unwrap().clone()
}

pub fn column_min(input: &[Vec<u64>], index: usize) -> u64 {
    let mut column: Vec<u64> = vec![];
    for row in input {
        column.push(row[index]);
    }

    column.into_iter().min().unwrap()
}
