

pub fn square_limits_from_square(s: usize) -> (usize, usize) {
    let u = s - s % 3;
    let l = (s % 3) * 3;

    (u, l)
}

pub fn square_limits_from_cell(r: usize, c: usize) -> (usize, usize) {
    let u = r - r % 3;
    let l = c - c % 3;
    (u, l)
}

// i -> r; j -> c
//  0 1 2 3 4 5 6 7 8
// 0     |     |
// 1  0  |  1  |  2
// 2     |     |
// ------------------
// 3     |     |
// 4  3  |  4  |  5
// 5     |     |
// ------------------
// 6     |     |
// 7  6  |  7  |  8
// 8     |     |
