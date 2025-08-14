pub const LEGAL_VALUES: [char; 9] = ['1', '2', '3', '4', '5', '6', '7', '8', '9'];


#[derive(Copy, Clone)]
#[allow(dead_code)]
pub enum Cell {
    Given(char),
    NonGiven(char),
    Empty
}

impl Cell {
    pub fn unwrap_or(self, default: char) -> char {
        match self {
            Cell::Given(x) | Cell::NonGiven(x) => x,
            Cell::Empty => default,
        }
    }
}

pub struct Board {
    pub values: [[Cell; 9]; 9],
}

impl Board {
    pub fn new(values: [[Cell; 9]; 9]) -> Self {
        Self { values }
    }

    pub fn zeroed() -> Self {
        Board::new([[Cell::Empty; 9]; 9])
    }
}

impl Clone for Board {
    fn clone(&self) -> Self {
        Board::new(self.values.clone())
    }
}
