pub const LEGAL_VALUES: [char; 9] = [ '1', '2', '3', '4', '5', '6', '7', '8', '9' ];

pub const EMPTY_VALUE: char = '.';

pub struct Board {
    pub values: [[char; 9]; 9],
    is_given: [[bool; 9]; 9]
}

impl Board {
    pub fn new() -> Self {
        Self {
            values: [[EMPTY_VALUE; 9]; 9],
            is_given: [[false; 9]; 9]
        }
    }
}
