use std::fmt;


#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum Digit { D1, D2, D3, D4, D5, D6, D7, D8, D9 }

pub const LEGAL_VALUES: [Digit; 9] = [ Digit::D1, Digit::D2, Digit::D3, Digit::D4, Digit::D5, Digit::D6, Digit::D7, Digit::D8, Digit::D9 ];

impl fmt::Display for Digit {
   fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
       match self {
           Digit::D1 => write!(f, "1"),
           Digit::D2 => write!(f, "2"),
           Digit::D3 => write!(f, "3"),
           Digit::D4 => write!(f, "4"),
           Digit::D5 => write!(f, "5"),
           Digit::D6 => write!(f, "6"),
           Digit::D7 => write!(f, "7"),
           Digit::D8 => write!(f, "8"),
           Digit::D9 => write!(f, "9"),
       }
   }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Cell {
    Given(Digit),
    NonGiven(Digit),
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
