use crate::game::{*};

#[allow(dead_code)]
pub fn fill_box_randomly<T: rand::Rng>(
    board: &mut Board,
    box_number: usize,
    rng: &mut T
) {
    use rand::seq::SliceRandom;

    let mut nums = crate::game::LEGAL_VALUES;
    nums.shuffle(rng);

    let mut num_i = 0;
    let l = ((box_number - 1) % 3) * 3;
    let u = (box_number - 1) / 3 * 3;

    for i in u..(u+3) {
        for j in l..(l+3) {
            board.values[i][j] = nums[num_i];
            num_i += 1;
        }
    }
}

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
