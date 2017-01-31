use std::fmt;
use std::str::FromStr;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Cell {
    Empty,
    Circle,
    Cross,
}

/// Represents the index of one cell on the board.
///
/// They can be parsed from the string representation which looks like `"b1"`.
/// The letter represents the column, the digit represents the row.
#[derive(Clone, Copy, Eq, PartialEq)]
pub struct CellId {
    /// This index is row-major.
    ///
    ///     0 1 2
    ///     3 4 5
    ///     6 7 8
    index: u8,
}

impl CellId {
    /// Creates a new ID of the cell in the `row` and `col`.
    ///
    /// Panics if `row` or `col` are invalid (≥ 3).
    pub fn new(row: u8, col: u8) -> Self {
        assert!(row < 3);
        assert!(col < 3);

        CellId {
            index: row * 3 + col,
        }
    }

    pub fn row(&self) -> u8 {
        self.index / 3
    }

    pub fn col(&self) -> u8 {
        self.index % 3
    }

    /// Returns a number that can be used to index an array. For every possible
    /// cell id, this function returns a different number, starting from 0,
    /// without gaps in between.
    pub fn index(&self) -> usize {
        self.index.into()
    }
}

impl fmt::Display for CellId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "[{}{}]",
            (self.col() + 'a' as u8) as char,
            (self.row() + '1' as u8) as char,
        )
    }
}

impl fmt::Debug for CellId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "CellId {{ {}, {} }}",
            self.index,
            self,
        )
    }
}

// This might be overengineered here. A simpler solution would fit this simple
// game better, I think.
impl FromStr for CellId {
    type Err = ParseCellIdError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Check if the string has the correct length and only ascii chars.
        if s.len() > 2 {
            return Err(ParseCellIdError::TooManyChars);
        }
        if s.len() < 2 {
            return Err(ParseCellIdError::TooFewChars);
        }
        if s.chars().count() != 2 {
            return Err(ParseCellIdError::NonAscii);
        }

        // We know the string contains two ASCII-bytes
        let s = s.to_lowercase().into_bytes();
        if let 'a' ... 'c' = s[0] as char {
            if let '1' ... '3' = s[1] as char {
                let col = s[0] as u8 - 'a' as u8;
                let row = s[1] as u8 - '1' as u8;
                Ok(CellId::new(row, col))
            } else {
                Err(ParseCellIdError::InvalidRow)
            }
        } else {
            Err(ParseCellIdError::InvalidColumn)
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ParseCellIdError {
    TooFewChars,
    TooManyChars,
    NonAscii,
    InvalidColumn,
    InvalidRow,
}

impl fmt::Display for ParseCellIdError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::ParseCellIdError::*;

        match *self {
            TooFewChars   => "too few chars (2 required)",
            TooManyChars  => "too many chars (2 required)",
            NonAscii      => "both chars have to be ascii",
            InvalidColumn => "column is invalid (has to be 'a', 'b' or 'c')",
            InvalidRow    => "row is invalid (has to be '1', '2' or '3')",
        }.fmt(f)
    }
}
