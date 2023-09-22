use std::result;
use std::cmp;

#[derive(Clone, Copy, PartialEq)]
pub enum PieceType {
    Pawn, 
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}


#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Color { 
    Black,
    White,
}

#[derive(Clone, Copy, PartialEq)]
pub struct Piece {
    piece: PieceType, 
    color: Color, 
}

pub enum Decision {
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


#[derive(PartialEq, Eq)]
pub enum MoveType {
    CaptureOrPawn, 
    Other, 
}

pub enum MoveError {
    OutsideBoard,
    WrongColorPiece,
    FriendlyFire,
    NoPiece, 
    BlockedPath,
    SelfCheck,
    Movement,
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
                    Some(Piece{piece: PieceType::Knight, color: Color::White}), 
                    Some(Piece{piece: PieceType::Bishop, color: Color::White}), 
                    Some(Piece{piece: PieceType::King, color: Color::White}), 
                    Some(Piece{piece: PieceType::Queen, color: Color::White}), 
                    Some(Piece{piece: PieceType::Bishop, color: Color::White}), 
                    Some(Piece{piece: PieceType::Knight, color: Color::White}), 
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
                    Some(Piece{piece: PieceType::Knight, color: Color::White}), 
                    Some(Piece{piece: PieceType::Bishop, color: Color::White}), 
                    Some(Piece{piece: PieceType::King, color: Color::White}), 
                    Some(Piece{piece: PieceType::Queen, color: Color::White}), 
                    Some(Piece{piece: PieceType::Bishop, color: Color::White}), 
                    Some(Piece{piece: PieceType::Knight, color: Color::White}), 
                    Some(Piece{piece: PieceType::Rook, color: Color::White}) 
                ]]
            }, 

            turn: Color::White,
            move_history: Vec::new(),
        }
    }
    /* should perform a move if possible */
    pub fn do_move(&mut self, mv: Move) -> result::Result<Option<Decision>, MoveError> {

        // if found enemy pieces, means king is still checked, and must undo move

        /*
        if !legal_Movement(mv.clone()) {
           // return some error  
        }
        */
        let the_piece = match self.board[mv.start_y][mv.start_x] {
            None => return Err(MoveError::NoPiece),
            Some(p) => if p.color != self.turn {
                return Err(MoveError::WrongColorPiece)
            }
            else {
                Some(p)
            },
        };
        
        let mut capture: bool = false;

        let end_square = match self.board[mv.start_y][mv.start_x] {
            None => Option::<Piece>::None, 
            Some(p) => if p.color == self.turn {
                return Err(MoveError::FriendlyFire)
            }
            else {
                capture = true;
                Some(p)
            }, 
        };

        match self.legal_movement(&mv, &the_piece, &end_square, capture) { 
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
        self.board[mv.start_y][mv.start_x] = None;
        // BIG Todo! : the_piece is not in scope
        self.board[mv.end_y][mv.end_x] = the_piece;

        if self.in_check() { 
            self.board[mv.start_y][mv.start_x] = saved_start;
            self.board[mv.end_y][mv.end_x] = saved_end;
            // return some in_check error
        }

        // else continue to mate check
        let mut safe_move: bool = false;

        // i'm sure there is a better way or writing this mate check. The complexity is through the
        // roof here.
        'move_gen: for org_y in 1..8 {
            for org_x in 1..8 {
                for dest_y in 1..8 {
                    for dest_x in 1..8 {
                        let cur_move: Move = Move{start_x: org_x, start_y: org_y, end_x: dest_x, end_y: dest_y};
                        let cur_piece = match self.board[org_y][org_x] {
                            None => continue,
                            Some(p) => if p.color != self.turn {
                                continue
                            }
                            else {
                                Some(p)
                            },
                        };
                        
                        let mut cur_capture: bool = false;

                        let cur_end_square = match self.board[dest_y][dest_x] {
                            None => Option::<Piece>::None, 
                            Some(p) => if p.color == self.turn {
                                continue 
                            }
                            else {
                                cur_capture = true;
                                Some(p)
                            }, 
                        };

                        if self.legal_movement(&cur_move, &cur_piece, &cur_end_square, cur_capture).is_none() {
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
                self.move_history.push(MoveType::CaptureOrPawn);
            }
            else {
                match saved_end {
                    None => {
                        self.move_history.push(MoveType::CaptureOrPawn);
                    }, 
                    _ => {
                        self.move_history.push(MoveType::Other);
                    }
                }
            }
        }
        // check for 50 move draw rule, and force draw like in chess com
        let mut pawn_capture_move: bool = false;
        for i in (self.move_history.len()-50..self.move_history.len()).rev() {
            if self.move_history[i as usize] == MoveType::CaptureOrPawn {
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
        // todo! : change stuff about self after a move
    }

    // checks for move legality
    pub fn legal_movement(&mut self, mv: &Move, the_piece: &Option<Piece>, end_square: &Option<Piece>, capture: bool) -> Option<MoveError> {
        /* check possible mv errors in order */
        //todo! come back here
        let board_y = 0..8;
        let board_x = 0..8;
        // if the position doesnt change
        if mv.start_x == mv.end_x && mv.start_y == mv.end_y { 
            return Some(MoveError::Movement);
        }
        // if the moves is in bounds
        if !board_x.contains(&mv.start_x) || !board_x.contains(&mv.end_x) || !board_y.contains(&mv.start_y) || !board_y.contains(&mv.end_y) {
            return Some(MoveError::OutsideBoard);
        }
        // todo! the_piece and end_square are of type Option<Piece>
        let dx: isize = { 
            if (mv.end_x as isize) - (mv.start_x as isize) > 0 {1}
            else if (mv.end_x as isize) - (mv.start_x as isize) < 0 {-1}
            else {0}
        };
        let dy: isize = { 
            if (mv.end_y as isize) - (mv.start_y as isize) > 0 {1}
            else if (mv.end_y as isize) - (mv.start_y as isize) < 0 {-1}
            else {0}
        };

        match the_piece.unwrap().piece {
            /* check if move is even legal */
            // check if it right type of move 
            PieceType::Pawn => {
                if (mv.end_x as isize - mv.start_x as isize).abs() != { if capture {1} else {0} } {
                    return Some(MoveError::Movement);
                }
                let y_dif = mv.end_y as isize - mv.start_y as isize;
                if mv.start_y == 0 || mv.start_y == 7 {
                    if y_dif > { if self.turn == Color::White {2} else {-2} } {
                        return Some(MoveError::Movement);
                    }
                }
                else {
                    if y_dif > { if self.turn == Color::White {1} else {-1} } {
                        return Some(MoveError::Movement);
                    }
                }
            }
            PieceType::Knight => {
                // should always have legal Movement at this stage
            }
            PieceType::Bishop => {
                if (mv.end_y as isize - mv.start_y as isize).abs() != (mv.end_x as isize - mv.start_x as isize).abs() {
                    return Some(MoveError::Movement);
                }

                let mut cur_x = mv.start_x;
                let mut cur_y = mv.start_y;
                while (cur_x != mv.end_x) || (cur_y != mv.end_y) {
                    if  (cur_x != mv.start_x) || (cur_y != mv.start_y) {
                        if self.board[cur_y][cur_x].is_some() {
                            return Some(MoveError::BlockedPath);
                        }
                    }
                    cur_x += dx as usize;
                    cur_y += dy as usize;
                }

            }
            PieceType::Rook => {
                if mv.end_y - mv.start_y != 0 && mv.end_x - mv.end_x != 0 { 
                    return Some(MoveError::Movement);
                }

                let mut cur_x = mv.start_x;
                let mut cur_y = mv.start_y;
                while (cur_x != mv.end_x) || (cur_y != mv.end_y) {
                    if  (cur_x != mv.start_x) || (cur_y != mv.start_y) {
                        if self.board[cur_y][cur_x].is_some() {
                            return Some(MoveError::BlockedPath);
                        }
                    }
                    cur_x += dx as usize;
                    cur_y += dy as usize;
                }
            }
            PieceType::Queen => {
                let Y = (mv.end_y as isize - mv.start_y as isize).abs();
                let X = (mv.end_x as isize - mv.start_x as isize).abs();
                if cmp::max(X, Y) != cmp::min(X, Y) && cmp::min(X, Y) != 0 {
                    return Some(MoveError::Movement);
                }
                let mut cur_x = mv.start_x;
                let mut cur_y = mv.start_y;
                while (cur_x != mv.end_x) || (cur_y != mv.end_y) {
                    if  (cur_x != mv.start_x) || (cur_y != mv.start_y) {
                        if self.board[cur_y][cur_x].is_some() {
                            return Some(MoveError::BlockedPath);
                        }
                    }
                    cur_x += dx as usize;
                    cur_y += dy as usize;
                }
            }
            PieceType::King => {
                // check if the attempted move is dx = 2 (potential attempt to castle)
                // otherwise there should be no problem
            }
        }
        
        return None;
    }

    pub fn in_check(&mut self) -> bool {
        /* FOR checking if a move is legal */ 
        // make a copy of the board 
        // find king
        let mut king_x: isize = 0;
        let mut king_y: isize = 0;
        'outer: for i in 1..8 {
            for j in 1..8 {
               match self.board[i as usize][j as usize] {
                   None => continue,
                   Some(p) => if p.color == self.turn && p.piece == PieceType::King {
                        king_y = i;
                        king_x = j;
                        break 'outer;
                   },
               }
            }
        }
        // cast a ray from the king in 8 directions 
        let dir: [[isize; 2]; 8] = [[-1, -1], [-1, 0], [0, -1], [0,1], [1, 0], [-1, 1], [1, -1], [1, 1]];
        let mut checked: bool = false;
        'outer: for i in 1..8 {
            let mut cur_x = king_x;
            let mut cur_y = king_y;
            let DX = dir[i as usize][0 as usize];
            let DY = dir[i as usize][1 as usize];
            while 0 <= cur_x+DX && cur_x+DX < 8 && 0 <= cur_y+DY && cur_y+DY < 8 {
                cur_x += DX;
                cur_y += DY;
                if self.board[cur_y as usize][cur_x as usize] != None && self.board[cur_y as usize][cur_x as usize].unwrap().color != self.turn {
                    checked = true;
                    break 'outer;
                }
            }
        }
        // check 1 knight move away
        let knight_dir: [[isize; 2]; 8] = [[2, -1], [2, 1], [-2, -1], [-2, 1], [1, 2], [1, -2], [-1, 2], [-1, -2]];
        'outer: for i in 1..8 {
            let cur_x = king_x + knight_dir[i as usize][0];
            let cur_y = king_y + knight_dir[i as usize][1];
            if 0 > cur_x || cur_x > 8 || 0 > cur_y || cur_y > 8 {
                continue;
            }
            if self.board[cur_y as usize][cur_x as usize] != None && self.board[cur_y as usize][cur_x as usize].unwrap().color != self.turn {
                checked = true;
                break 'outer;
            }
        }
        return checked;
    }
}


#[cfg(test)]
mod tests {
    //#[test]
    
}
