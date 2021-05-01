fn main() {
    let mut matrix = vec![vec![0, 1, 2, 0], vec![3, 4, 5, 2], vec![1, 3, 1, 5]];
    set_zeroes(&mut matrix);

    println!("{:?}", matrix);
}

pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
    let (m, n) = (matrix.len(), matrix[0].len());

    let mut flag_col0 = false;

    for i in 0..m {
        if matrix[i][0] == 0 {
            flag_col0 = true;
        }
        for j in 1..n {
            if matrix[i][j] == 0 {
                matrix[i][0] = 0;
                matrix[0][j] = 0;
            }
        }
    }

    for i in (0..=(m - 1)).rev() {
        for j in 1..n {
            if matrix[i][0] == 0 || matrix[0][j] == 0 {
                matrix[i][j] = 0;
            }
        }
        if flag_col0 {
            matrix[i][0] = 0;
        }
    }
}
