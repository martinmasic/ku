

pub const LEGAL_VALUES: [char; 9] = ['1', '2', '3', '4', '5', '6', '7', '8', '9'];

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Cell {
    Given(char),
    NonGiven(char),
    Empty
}

impl Cell {
    pub fn to_string(&self, show_non_givens: bool) -> String {
        match self {
            Cell::Given(x) => x.to_string(),
            Cell::NonGiven(x) => {
                if show_non_givens { x.to_string() }
                else { "_".to_string() }
            },
            Cell::Empty => "_".to_string()
        }
    }
}

#[derive(Copy, Debug, Clone)]
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

    pub fn non_givens_cleared(mut self) -> Board {
        for i in 0..9 {
            for j in 0..9 {
                if let Cell::NonGiven(_) = self.values[i][j] {
                    self.values[i][j] = Cell::Empty;
                }
            }
        }
        self
    }

    pub fn set(&mut self, digit: Cell, pos: (usize, usize)) {
        self.values[pos.0][pos.1] = digit;
    }

    pub fn set_non_given(&mut self, digit: Cell, pos: (usize, usize)) {
        if let Cell::Given(_) = digit { return; }
        if let Cell::NonGiven(_) = self.values[pos.0][pos.1] {
            self.set(digit, pos);
        }
        if let Cell::Empty = self.values[pos.0][pos.1] {
            self.set(digit, pos);
        }
    }
}
