use std::result
use std::cmp

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
    board: [[Option<Piece>; 8]; 8],
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
    pub fn do_move(&mut self, mv: Move) -> result::Result<Option<Color>, MoveError>> {
        /* check possible mv errors in order */
        let board_y = 0..8;
        let board_x = 0..8;
        if mv.start_x == mv.end_x && mv.start_y == mv.end_y { 
            return Err(MoveError::movement);
        }
        if !board_x.contains(mv.start_x) || !board_x.contains(mv.end_x) || !board_y.contains(mv.start_y) || !board_y.contains(mv.end_y) {
            return Err(MoveError::outsideBoard);
        }
        let the_piece = match self.board[mv.start_y][mv.start_x] {
            None => return Err(MoveError::noPiece),
            Some(p) => if p.color != self.turn {
                return Err(MoveError::wrongColorPiece)
            }
            else {
                Some(p);
            }, 
        };
        
        let capture: bool = 0;

        let end_square = match self.board[mv.start_y][mv.start_x] {
            None => Option::<Piece>::None, 
            Some(p) => if p.color == self.turn {
                return Err(MoveError::wrongColorPiece)
            }
            else {
                capture = 1;
                Some(p);
            }, 
        };

        let dx: usize = { 
            if mv.end_x - mv.start_x > 0 {1}
            else if mv.end_x - mv.start_x < 0 {-1}
            else {0}
        };
        let dy: usize = { 
            if mv.end_y - mv.start_y > 0 {1}
            else if mv.end_y - mv.start_y < 0 {-1}
            else {0}
        };

        match the_piece.piece {
            /* check if move is even legal */
            // check if it right type of move 
            PieceType::Pawn => {
                if abs(mv.end_x - mv.start_x) != { if capture {1} else {0} } {
                    return Err(MoveError::movement);
                }
                let yDif = mv.end_y - mv.start_y;
                if mv.start_y == 0 || mv.start_y == 7 {
                    if yDif > { if self.turn == White {2} else {-2} } {
                        return Err(MoveError::movement);
                    }
                }
                else {
                    if yDif > { if self.turn == White {1} else {-1} } {
                        return Err(MoveError::movement);
                    }
                }
                // else do the move;
                self.board[mv.start_y][mv.start_x] = Option<Piece>::None;
                self.board[mv.end_y][mv.end_x] = the_piece;
            }
            PieceType::Knight => {
                self.board[mv.start_y][mv.start_x] = Option<Piece>::None;
                self.board[mv.end_y][mv.end_x] = the_piece;
            }
            PieceType::Bishop => {

                if abs(mv.end_y - mv.start_y) != abs(mv.end_x - mv.start_x) {
                    return Err(MoveError::movement);
                }

                let curX = mv.start_x;
                let curY = mv.start_y;
                while (curX != mv.end_x) || (curY != mv.end_y) {
                    if  (curX != mv.start_x) || (curY != mv.start_y) {
                        if self.board[curY][curX] != Option<Piece>::None {
                            return Err(MoveError::blockedPath);
                        }
                    }
                    curX += dx;
                    curX += dy;
                }

                self.board[mv.start_y][mv.start_x] = Option<Piece>::None;
                self.board[mv.end_y][mv.end_x] = the_piece;
            }
            PieceType::Rook => {
                if mv.end_y - mv.start_y != 0 && mv.end_x - mv.end_x != 0 { 
                    return Err(MoveError::movement);
                }

                let curX = mv.start_x;
                let curY = mv.start_y;
                while (curX != mv.end_x) || (curY != mv.end_y) {
                    if  (curX != mv.start_x) || (curY != mv.start_y) {
                        if self.board[curY][curX] != Option<Piece>::None {
                            return Err(MoveError::blockedPath);
                        }
                    }
                    curX += dx;
                    curX += dy;
                }
                self.board[mv.start_y][mv.start_x] = Option<Piece>::None;
                self.board[mv.end_y][mv.end_x] = the_piece;
            }
            PieceType::Queen => {
                /* todo! : need to check if there is something in between */
                let Y = abs(mv.end_y - mv.start_y);
                let X = abs(mv.end_x - mv.start_x);
                if (cmp::max(X, Y) != cmp::min(X, Y) && cmp::min(X, Y) != 0) {
                    return Err(MoveError::movement);
                }
                let curX = mv.start_x;
                let curY = mv.start_y;
                while (curX != mv.end_x) || (curY != mv.end_y) {
                    if  (curX != mv.start_x) || (curY != mv.start_y) {
                        if self.board[curY][curX] != Option<Piece>::None {
                            return Err(MoveError::blockedPath);
                        }
                    }
                    curX += dx;
                    curX += dy;
                }
                self.board[mv.start_y][mv.start_x] = Option<Piece>::None;
                self.board[mv.end_y][mv.end_x] = the_piece;
            }
            PieceType::King => {
                // check if the attempted move is dx = 2 (potential attempt to castle)
                self.board[mv.start_y][mv.start_x] = Option<Piece>::None;
                self.board[mv.end_y][mv.end_x] = the_piece;
            }
        }

        

        /* FOR checking if a move is legal */ 
        // remember stuff so you can undo the move
        // make a copy of the board 
        let tempBoard = self.board.clone();
        // find king
        let kingX: usize = 0;
        let kingY: usize = 0;
        'outer: for i in 1..8 {
            for j in 1..8 {
               match tempBoard[i as usize][j as usize] {
                   None => continue,
                   Some(p) => if p.color == self.turn && p.piece == King {
                        KingY = i;
                        kingX = j;
                        break 'outer;
                   },
               }
            }
        }
        // cast a ray from the king in 8 directions, then check 1 knight move away for a knight 
        let dir: [[usize; 2]; 4] = [[-1, -1], [-1, 0], [0, -1], [0,1], [1, 0], [-1, 1], [1, -1], [1, 1]];
        for i in 1..8 {
            let curX = KingX;
            let curY = KingY;
            let DX = dir[i as usize][0 as usize];
            let DY = dir[i as usize][1 as usize];
            while (0 < curX+DX && curX+DX < 8 && 0 < curY+DY && curY+DY < 8) {
                curX += DX;
                curY += DY;
            }
        }
        // if found enemy pieces, means king is still checked, and must undo move
        // else continue to mate check

        // todo! : check if position is a mate
    }

}
