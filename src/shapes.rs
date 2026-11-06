use crate::board::Cell;

pub struct Piece {
    pub kind: Cell,
    pub x: i32, // Could be partially Up from the board
    pub y: i32, // Could be negative
}

pub fn cells(kind: Cell) -> [(i32, i32); 4] {
    match kind {
        Cell::T => [(1, 0), (0, 1), (1, 1), (2, 1)],
        Cell::I => [(0, 1), (1, 1), (2, 1), (3, 1)],
        Cell::O => [(1, 0), (2, 0), (1, 1), (2, 1)],
        Cell::S => [(), (), (), ()],
        Cell::Z => [],
        Cell::J => [],
        Cell::L => [],
    }
}
