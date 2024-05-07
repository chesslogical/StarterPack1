use std::{thread, time};

fn main() {
    let mut board = vec![
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 1, 1, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 1, 0, 0, 0, 0, 0],
        vec![0, 0, 1, 0, 1, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    ];

    loop {
        print_board(&board);
        board = next_generation(&board);
        thread::sleep(time::Duration::from_millis(500));
    }
}

fn print_board(board: &Vec<Vec<u8>>) {
    print!("\x1B[2J\x1B[1;1H");
    for row in board {
        for &cell in row {
            print!("{}", if cell == 0 { ' ' } else { '◼' });
        }
        println!();
    }
}

fn next_generation(board: &Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let mut new_board = board.clone();
    for y in 0..board.len() {
        for x in 0..board[y].len() {
            let alive_neighbors = count_alive_neighbors(board, x, y);
            if board[y][x] == 1 {
                if alive_neighbors < 2 || alive_neighbors > 3 {
                    new_board[y][x] = 0;
                }
            } else {
                if alive_neighbors == 3 {
                    new_board[y][x] = 1;
                }
            }
        }
    }
    new_board
}

fn count_alive_neighbors(board: &Vec<Vec<u8>>, x: usize, y: usize) -> u8 {
    let mut count = 0;
    for i in -1..=1 {
        for j in -1..=1 {
            if i == 0 && j == 0 {
                continue;
            }
            let new_x = x as isize + i;
            let new_y = y as isize + j;
            if new_x >= 0
                && new_x < board[0].len() as isize
                && new_y >= 0
                && new_y < board.len() as isize
                && board[new_y as usize][new_x as usize] == 1
            {
                count += 1;
            }
        }
    }
    count
}

