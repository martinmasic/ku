use crate::game::{*};

pub fn print_board(board: &Board) {
    for i in 0..3 {
        print!(
            "{} {} {} | {} {} {} | {} {} {}\n",
            board.values[i][0].unwrap_or('.'), board.values[i][1].unwrap_or('.'), board.values[i][2].unwrap_or('.'),
            board.values[i][3].unwrap_or('.'), board.values[i][4].unwrap_or('.'), board.values[i][5].unwrap_or('.'),
            board.values[i][6].unwrap_or('.'), board.values[i][7].unwrap_or('.'), board.values[i][8].unwrap_or('.'),
        );
    }
    for _i in 0..21 {
        print!("-");
    }
    print!("\n");
    for i in 3..6 {
        print!(
            "{} {} {} | {} {} {} | {} {} {}\n",
            board.values[i][0].unwrap_or('.'), board.values[i][1].unwrap_or('.'), board.values[i][2].unwrap_or('.'),
            board.values[i][3].unwrap_or('.'), board.values[i][4].unwrap_or('.'), board.values[i][5].unwrap_or('.'),
            board.values[i][6].unwrap_or('.'), board.values[i][7].unwrap_or('.'), board.values[i][8].unwrap_or('.'),
        );
    }
    for _i in 0..21 {
        print!("-");
    }
    print!("\n");
    for i in 6..9 {
        print!(
            "{} {} {} | {} {} {} | {} {} {}\n",
            board.values[i][0].unwrap_or('.'), board.values[i][1].unwrap_or('.'), board.values[i][2].unwrap_or('.'),
            board.values[i][3].unwrap_or('.'), board.values[i][4].unwrap_or('.'), board.values[i][5].unwrap_or('.'),
            board.values[i][6].unwrap_or('.'), board.values[i][7].unwrap_or('.'), board.values[i][8].unwrap_or('.'),
        );
    }
}
