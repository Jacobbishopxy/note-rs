fn main() {
    let foo = vec![vec![1, 2, 3, 4], vec![5, 6, 7, 8], vec![9, 10, 11, 12]];

    println!("result: {:?}", spiral_order(foo));
}

pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
    let mut res = vec![];

    if matrix.len() == 0 || matrix[0].len() == 0 {
        return res;
    }

    let (mut top, mut left) = (0_usize, 0_usize);
    let (mut right, mut bottom) = (matrix[0].len() - 1, matrix.len() - 1);

    while left <= right && top <= bottom {
        for i in left..=right {
            res.push(matrix[top][i]);
        }

        for i in (top + 1)..=bottom {
            res.push(matrix[i][right]);
        }
        if left < right && top < bottom {
            for i in ((left + 1)..=(right - 1)).rev() {
                res.push(matrix[bottom][i]);
            }
            for i in ((top + 1)..=bottom).rev() {
                res.push(matrix[i][left]);
            }
        }

        if right == 0 || bottom == 0 {
            break;
        }
        left += 1;
        right -= 1;
        top += 1;
        bottom -= 1;
    }

    res
}
