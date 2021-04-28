fn main() {
    let mut foo = vec![
        vec![5, 1, 9, 11],
        vec![2, 4, 8, 10],
        vec![13, 3, 6, 7],
        vec![15, 14, 12, 16],
    ];

    rotate(&mut foo);

    println!("result: {:?}", foo);
}

pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
    let s = matrix.len();

    for i in 0..(s / 2) {
        for j in 0..s {
            unsafe {
                std::ptr::swap(&mut matrix[i][j], &mut matrix[s - i - 1][j]);
            }
        }
    }

    let mut count = 0;

    for i in 0..s {
        let mut j = count;
        loop {
            if j > s - 1 {
                break;
            }

            unsafe {
                std::ptr::swap(&mut matrix[i][j], &mut matrix[j][i]);
            }
            j += 1;
        }
        count += 1;
    }
}
