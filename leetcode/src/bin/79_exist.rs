fn main() {
    let board = vec![
        vec!['A', 'B', 'C', 'E'],
        vec!['S', 'F', 'C', 'S'],
        vec!['A', 'D', 'E', 'E'],
    ];
    let word = "ABCCED".to_string();

    println!("result: {:?}", exist(board, word));
}

pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
    let mut s = S::new(board, word);
    let (r, c) = s.get_size();

    for i in 0..r {
        for j in 0..c {
            if s.check(i, j, 0) {
                return true;
            }
        }
    }

    false
}

pub struct S {
    pub board: Vec<Vec<char>>,
    pub word: String,
    size: (i32, i32),
    directions: Vec<(i32, i32)>,
    visited: Vec<Vec<bool>>,
}

impl S {
    pub fn new(board: Vec<Vec<char>>, word: String) -> Self {
        let (r, c) = (board.len(), board[0].len());
        S {
            board,
            word,
            size: (r as i32, c as i32),
            directions: vec![(0, 1), (0, -1), (1, 0), (-1, 0)],
            visited: vec![vec![false; c]; r],
        }
    }

    pub fn get_size(&self) -> (i32, i32) {
        self.size
    }

    pub fn check(&mut self, i: i32, j: i32, k: i32) -> bool {
        if self.board[i as usize][j as usize] != self.word.chars().nth(k as usize).unwrap() {
            return false;
        }
        if k == self.word.chars().count() as i32 - 1 {
            return true;
        }

        self.visited[i as usize][j as usize] = true;
        let mut result = false;

        for (di, dj) in self.directions.clone().iter() {
            let (new_i, new_j) = (i + di, j + dj);
            if 0 <= new_i && new_i < self.size.0 && 0 <= new_j && new_j < self.size.1 {
                if !self.visited[new_i as usize][new_j as usize] {
                    if self.check(new_i, new_j, k + 1) {
                        result = true;
                        break;
                    }
                }
            }
        }

        self.visited[i as usize][j as usize] = false;

        result
    }
}
