fn main() {
    let foo = vec![
        vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
        vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
        vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
        vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
        vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
        vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
        vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
        vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
        vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
    ];

    println!("result: {}", is_valid_sudoku(foo));
}

pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
    if !is_valid_matrix(&board) {
        return false;
    }

    if !is_valid_matrix(&sudoku_transpose(board.clone())) {
        return false;
    }

    if !is_valid_matrix(&sudoku_turn_nine_grid(board)) {
        return false;
    }

    true
}

fn is_valid_vec(v: &Vec<char>) -> bool {
    let mut cache = vec![];

    for &i in v.iter() {
        if i != '.' {
            if cache.contains(&i) {
                return false;
            }

            cache.push(i);
        }
    }

    true
}

fn is_valid_matrix(v: &Vec<Vec<char>>) -> bool {
    for l in v.iter() {
        if is_valid_vec(l) == false {
            return false;
        }
    }

    true
}

fn sudoku_transpose(v: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut res = vec![vec!['.'; 9]; 9];

    for i in 0..9 {
        for j in 0..9 {
            res[j][i] = v[i][j];
        }
    }

    res
}

fn sudoku_turn_nine_grid(v: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut res = vec![];

    for i in 0..3 {
        for j in 0..3 {
            let (a, b, c) = (i * 3, 1 + i * 3, 2 + i * 3);
            let (x, z) = (j * 3, 2 + j * 3);

            res.push([&v[a][x..=z], &v[b][x..=z], &v[c][x..=z]].concat());
        }
    }

    res
}
