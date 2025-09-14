

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
    values: [[Cell; 9]; 9],
}

impl Board {
    pub fn new(values: [[Cell; 9]; 9]) -> Self {
        Self { values }
    }

    pub fn zeroed() -> Self {
        Board::new([[Cell::Empty; 9]; 9])
    }

    pub fn at(&self, row: usize, col: usize) -> Cell {
        debug_assert!(row <= 8);
        debug_assert!(col <= 8);

        self.values[row][col]
    }

    pub fn set(&mut self, row: usize, col: usize, digit: Cell) {
        debug_assert!(row <= 8);
        debug_assert!(col <= 8);

        self.values[row][col] = digit;
    }

    pub fn row_cells(&self, row: usize) -> impl Iterator<Item = (usize, &Cell)> {
        self.values[row].iter().enumerate()
    }

    pub fn col_cells(&self, col: usize) -> impl Iterator<Item = (usize, &Cell)> {
        self.values.iter().map(move |r| &r[col]).enumerate()
    }

    pub fn non_givens_cleared(mut self) -> Board {
        for r in 0..9 {
            for c in 0..9 {
                if let Cell::NonGiven(_) = self.at(r, c) {
                    self.set(r, c, Cell::Empty);
                }
            }
        }
        self
    }

    pub fn set_non_given(&mut self, row: usize, col: usize, digit: Cell) {
        match self.at(row, col) {
            Cell::Given(_) => return,
            Cell::NonGiven(_) | Cell::Empty => self.set(row, col, digit),
        }
    }
}
