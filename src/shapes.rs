use crate::board::{Board, Cell, HEIGHT, WIDTH};

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
        Cell::S => [(1, 0), (2, 0), (0, 1), (1, 1)],
        Cell::Z => [(1, 0), (0, 0), (1, 1), (2, 1)],
        Cell::J => [(0, 0), (0, 1), (1, 1), (2, 1)],
        Cell::L => [(2, 0), (0, 1), (1, 1), (2, 1)],
    }
}

pub fn can_place(board: &Board, piece: &Piece, new_x: i32, new_y: i32) -> bool {
    for (dx, dy) in cells(piece.kind) {
        let x = new_x + dx;
        let y = new_y + dy;
        if x < 0
            || x >= WIDTH as i32
            || y >= HEIGHT as i32
            || (y >= 0 && board.cell(x as usize, y as usize).is_some())
        {
            return false;
        }
    }
    true
}

impl Piece {
    pub fn lock(&self, board: &mut Board) {
        for (dx, dy) in cells(self.kind) {
            board.set(
                (self.x + dx) as usize,
                (self.y + dy) as usize,
                Some(self.kind),
            );
        }
    }
}
