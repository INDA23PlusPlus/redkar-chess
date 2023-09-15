pub enum PieceType {
    Pawn, 
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

pub enum Color { 
    Black,
    White,
}

pub struct Piece {
    piece: PieceType, 
    color: Color, 
}

pub struct MoveType { 
    normal, 
    // promotion,
    // en_passant,
    // castling,
}

pub struct Move {
    start_x: usize,
    start_y: usize,
    end_x: usize,
    end_y: usize,
    capture: bool, 
    movetype: MoveType
}

pub enum MoveError {
    outsideBoard,
    wrongColorPiece,
    friendlyFire,
    noPiece, 
    blockedPath,
    selfCheck,
    movement,
    // pinnedPiece,
}

pub struct Game {
    board: [[Option<Piece>8]; 8],
    turn: Color, 
    inCheck: bool, 
    move_history: Vec<Move>, /* will be needed to check whether draw can be claimed */
}

impl Game {
    pub fn new_game() -> Game {
        Game {
            board: {
                [[
                    Some(Piece{piece: PieceType::Rook, color: Color::White}),  
                    Some(Piece{piece: PieceType::Knight, color: Color::White})
                    Some(Piece{piece: PieceType::Bishop, color: Color::White})
                    Some(Piece{piece: PieceType::King, color: Color::White})
                    Some(Piece{piece: PieceType::Queen, color: Color::White})
                    Some(Piece{piece: PieceType::Bishop, color: Color::White})
                    Some(Piece{piece: PieceType::Knight, color: Color::White})
                    Some(Piece{piece: PieceType::Rook, color: Color::White}) 
                ],
                [Some(Piece{piece: PieceType::Pawn, color: Color::White}); 8],
                [None; 8],
                [None; 8],
                [None; 8],
                [None; 8],
                [Some(Piece{piece: PieceType::Pawn, color: Color::Black}); 8],
                [
                    Some(Piece{piece: PieceType::Rook, color: Color::White}),  
                    Some(Piece{piece: PieceType::Knight, color: Color::White})
                    Some(Piece{piece: PieceType::Bishop, color: Color::White})
                    Some(Piece{piece: PieceType::King, color: Color::White})
                    Some(Piece{piece: PieceType::Queen, color: Color::White})
                    Some(Piece{piece: PieceType::Bishop, color: Color::White})
                    Some(Piece{piece: PieceType::Knight, color: Color::White})
                    Some(Piece{piece: PieceType::Rook, color: Color::White}) 
                ]],
            }

            turn: White,
            inCheck: false,
            move_history: Vec::new(),
        }
    }
    /* should perform a move if possible */
    pub fn move(&mut self, move: Move) -> {
        /* check possible move errors in order */
        let board_y = 0..8;
        let board_x = 0..8;
        if (!board_x.contains(move.start_x) || !board_x.contains(move.end_x) || !board_y.contains(move.start_y) || !board_y.contains(move.end_y)) {
            return Err(MoveError::outsideBoard);
        }
        let the_piece = match self.board[move.start_y][move.start_x] {
            None => return Err(moveError::noPiece),
            Some(p) => if p.color != self.turn {
                return Err(MoveError::wrongColorPiece)
            }
            else {
                p
            }, 
        };
        
        let end_square = match self.board[move.start_y][move.start_x] {
            None => Option::<Piece>::None;
            Some(p) => if p.color == self.turn {
                return Err(MoveError::wrongColorPiece)
            }
            else {
                p
            }, 
        };
        match the_piece.piece {
            PieceType::Pawn => {
                if move.capture {
                                     
                }
                else {

                }
            }
            PieceType::Knight => {

            }
            PieceType::Bishop => {

            }
            PieceType::Rook => {

            }
            PieceType::Queen => {

            }
            PieceType::King => {

            }
        }
    }

}
