pub const LEGAL_VALUES: [Option<char>; 9] = [ Some('1'), Some('2'), Some('3'), Some('4'), Some('5'), Some('6'), Some('7'), Some('8'), Some('9') ];

pub struct Board {
    pub values: [[Option<char>; 9]; 9],
    pub is_given: [[bool; 9]; 9]
}

impl Board {
    pub fn new(
        values: [[Option<char>; 9]; 9],
        is_given: [[bool; 9]; 9]
    ) -> Self {
        Self {
            values: values,
            is_given: is_given
        }
    }

    // // TODO: ??
    // pub fn new_given(values: [[Option<char>; 9]; 9]) -> Self {
    //     Board::new(values, [[false; 9]; 9])
    // }

    pub fn zeroed() -> Self {
        Board::new([[None; 9]; 9], [[false; 9]; 9])
    }
}

impl Clone for Board {
    fn clone(&self) -> Self {
        Board::new(self.values.clone(), self.is_given.clone())
    }
}
