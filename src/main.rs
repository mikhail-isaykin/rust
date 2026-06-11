fn snake_matrix(n: usize) -> Vec<Vec<usize>> {
    let mut matrix = vec![vec![0; n]; n];
    let mut value = 1;

    for row in 0..n {
        if row % 2 == 0 {
            for col in 0..n {
                matrix[row][col] = value;
                value += 1;
            }
        } else {
            for col in (0..n).rev() {
                matrix[row][col] = value;
                value += 1;
            }
        }
    }

    matrix
}

fn main() {
    let matrix = snake_matrix(4);

    for row in matrix {
        println!("{:?}", row);
    }
}