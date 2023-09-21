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

pub struct Decision {
    White,
    Black,
    Tie,
}

/*
pub struct MoveType { 
    normal, 
    // promotion,
    // en_passant,
    // castling,
}
*/

pub struct Move {
    start_x: usize,
    start_y: usize,
    end_x: usize,
    end_y: usize,
}


pub struct MoveType {
    capture_or_pawn, 
    other, 
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
    move_history: Vec<MoveType>, /* will be needed to check whether draw can be claimed */
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
            move_history: Vec::new(),
        }
    }
    /* should perform a move if possible */
    pub fn do_move(&mut self, mv: Move) -> result::Result<Option<Decision>, MoveError> {

        // if found enemy pieces, means king is still checked, and must undo move

        /*
        if !legal_movement(mv.clone()) {
           // return some error  
        }
        */
        match legal_movement(mv) { 
            Some(x) => {
                return Err(x);
            }
            None => {
                // just a normal move
            }
        }

        let saved_start: Option<Piece> = self.board[mv.start_y][mv.start_x].clone();
        let saved_end: Option<Piece> = self.board[mv.end_y][mv.end_x].clone();


        // potentially temporarily make the move
        self.board[mv.start_y][mv.start_x] = Option<Piece>::None;
        // BIG Todo! : the_piece is not in scope
        self.board[mv.end_y][mv.end_x] = the_piece;

        if inCheck(self.turn) { 
            self.board[mv.start_y][mv.start_x] = saved_start;
            self.board[mv.end_y][mv.end_x] = saved_end;
            // return some inCheck error
        }

        // else continue to mate check
        let safe_move: bool = false;

        // i'm sure there is a better way or writing this mate check. The complexity is through the
        // roof here.
        'move_gen: for org_Y in 1..8 {
            for org_X in 1..8 {
                for dest_Y in 1..8 {
                    for dest_X in 1..8 {
                        let cur_move: Move = Move{start_x: org_X, start_y: org_Y, end_x: dest_X, end_y: dest_Y};
                        if legal_movement(&mut self, cur_move) {
                            safe_move = true;
                            break 'move_gen;
                        }
                    }
                }
            }
        }
        if !safe_move {
            // self.turn has won
            // signal end of game or something
            let winner = match self.turn {
                Color::White => "White",
                Color::Black => "Black",
            };
            let loser = match self.turn {
                Color::White => "Black",
                Color::Black => "White",
            };
            let decision = match self.turn { 
                Color::White => Decision::White,
                Color::Black => Decision::Black,
            };
            println!("{} has checkmated {}, and won the game", winner, loser);
            return Ok(Some(decision));
        }
        else {
            if saved_start.unwrap().piece == PieceType::Pawn {
                self.move_history.push(MoveType::capture_or_pawn);
            }
            else {
                match saved_end {
                    None => {
                        self.move_history.push(MoveType::capture_or_pawn);
                    }, 
                    _ => {
                        self.move_history.push(MoveType::other);
                    }
                }
            }
        }
        // check for 50 move draw rule, and force draw like in chess com
        let pawn_capture_move: bool = false;
        for i in (self.move_history.len()-50..self.move_history.len()).rev() {
            if self.move_history[i as usize] == MoveType::capture_or_pawn {
                pawn_capture_move = true;
                break;
            }
        }
        // todo! check all let matches for semicolon
        if !pawn_capture_move {
            // the game is drawn
            // end game or something
            // dont know println is the best way to handle this
            println!("The game is drawn because 50 reversible moves have been played");
            return Ok(Some(Decision::Tie));
        }
        return Ok(None);
    }

    // checks for move legality
    pub fn legal_movement(brd: Board, mv: Move /* need to pass the_piece and endsquare by reference here */) -> Option<MoveError> {
        /* check possible mv errors in order */
        //todo! come back here
        let board_y = 0..8;
        let board_x = 0..8;
        // if the position doesnt change
        if mv.start_x == mv.end_x && mv.start_y == mv.end_y { 
            return Some(MoveError::movement);
        }
        // if the moves is in bounds
        if !board_x.contains(mv.start_x) || !board_x.contains(mv.end_x) || !board_y.contains(mv.start_y) || !board_y.contains(mv.end_y) {
            return Some(MoveError::outsideBoard);
        }
        let the_piece = match self.board[mv.start_y][mv.start_x] {
            None => return Some(MoveError::noPiece),
            Some(p) => if p.color != self.turn {
                return Some(MoveError::wrongColorPiece)
            }
            else {
                Some(p),
            }, 
        };
        
        let capture: bool = 0;

        let end_square = match self.board[mv.start_y][mv.start_x] {
            None => Option::<Piece>::None, 
            Some(p) => if p.color == self.turn {
                return Some(MoveError::wrongColorPiece)
            }
            else {
                capture = 1;
                Some(p),
            }, 
        };
        // todo! the_piece and end_square are of type Option<Piece>
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
                    return Some(MoveError::movement);
                }
                let yDif = mv.end_y - mv.start_y;
                if mv.start_y == 0 || mv.start_y == 7 {
                    if yDif > { if self.turn == White {2} else {-2} } {
                        return Some(MoveError::movement);
                    }
                }
                else {
                    if yDif > { if self.turn == White {1} else {-1} } {
                        return Some(MoveError::movement);
                    }
                }
            }
            PieceType::Knight => {
                // should always have legal movement at this stage
            }
            PieceType::Bishop => {
                if abs(mv.end_y - mv.start_y) != abs(mv.end_x - mv.start_x) {
                    return Some(MoveError::movement);
                }

                let curX = mv.start_x;
                let curY = mv.start_y;
                while (curX != mv.end_x) || (curY != mv.end_y) {
                    if  (curX != mv.start_x) || (curY != mv.start_y) {
                        if self.board[curY][curX].is_some() {
                            return Some(MoveError::blockedPath);
                        }
                    }
                    curX += dx;
                    curX += dy;
                }

            }
            PieceType::Rook => {
                if mv.end_y - mv.start_y != 0 && mv.end_x - mv.end_x != 0 { 
                    return Some(MoveError::movement);
                }

                let curX = mv.start_x;
                let curY = mv.start_y;
                while (curX != mv.end_x) || (curY != mv.end_y) {
                    if  (curX != mv.start_x) || (curY != mv.start_y) {
                        if self.board[curY][curX].is_some() {
                            return Some(MoveError::blockedPath);
                        }
                    }
                    curX += dx;
                    curX += dy;
                }
            }
            PieceType::Queen => {
                let Y = abs(mv.end_y - mv.start_y);
                let X = abs(mv.end_x - mv.start_x);
                if (cmp::max(X, Y) != cmp::min(X, Y) && cmp::min(X, Y) != 0) {
                    return Some(MoveError::movement);
                }
                let curX = mv.start_x;
                let curY = mv.start_y;
                while (curX != mv.end_x) || (curY != mv.end_y) {
                    if  (curX != mv.start_x) || (curY != mv.start_y) {
                        if self.board[curY][curX].is_some() {
                            return Some(MoveError::blockedPath);
                        }
                    }
                    curX += dx;
                    curX += dy;
                }
            }
            PieceType::King => {
                // check if the attempted move is dx = 2 (potential attempt to castle)
                // otherwise there should be no problem
            }
        }
        
        return None;
    }

    pub fn inCheck(&mut self, C: Color) -> bool {
        /* FOR checking if a move is legal */ 
        // make a copy of the board 
        // find king
        let kingX: usize = 0;
        let kingY: usize = 0;
        'outer: for i in 1..8 {
            for j in 1..8 {
               match self.board[i as usize][j as usize] {
                   None => continue,
                   Some(p) => if p.color == self.turn && p.piece == King {
                        KingY = i;
                        kingX = j;
                        break 'outer;
                   },
               }
            }
        }
        // cast a ray from the king in 8 directions 
        let dir: [[usize; 2]; 8] = [[-1, -1], [-1, 0], [0, -1], [0,1], [1, 0], [-1, 1], [1, -1], [1, 1]];
        let checked: bool = false;
        'outer: for i in 1..8 {
            let curX = KingX;
            let curY = KingY;
            let DX = dir[i as usize][0 as usize];
            let DY = dir[i as usize][1 as usize];
            while (0 < curX+DX && curX+DX < 8 && 0 < curY+DY && curY+DY < 8) {
                curX += DX;
                curY += DY;
                if self.board[curY][curX] != None && self.board[curY][curX] != self.turn {
                    checked = true;
                    break 'outer;
                }
            }
        }
        // check 1 knight move away
        let knight_dir: [[usize; 2]; 8] = [[2, -1], [2, 1], [-2, -1], [-2, 1], [1, 2], [1, -2], [-1, 2], [-1, -2]];
        'outer: for i in 1..8 {
            let curX = KingX + dir[i as usize][0];
            let curY = KingY + dir[i as usize][1];
            if self.board[curY][curX] != None && self.board[curY][curX].color != self.turn {
                checked = true;
                break 'outer;
            }
        }
        return checked;

}
