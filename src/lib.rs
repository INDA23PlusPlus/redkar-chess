use std::result;
use std::cmp::min;
use std::cmp::max;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum PieceType {
    Pawn, 
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}


#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Color { 
    Black,
    White,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Piece {
    piece: PieceType, 
    color: Color, 
}

#[derive(Clone, Copy, PartialEq, Debug)]
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

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Move {
    pub start_x: usize,
    pub start_y: usize,
    pub end_x: usize,
    pub end_y: usize,
}


#[derive(PartialEq, Eq, Clone, Debug)]
pub enum MoveType {
    CaptureOrPawn, 
    Other, 
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum MoveError {
    OutsideBoard,
    WrongColorPiece,
    FriendlyFire,
    NoPiece, 
    BlockedPath,
    SelfCheck,
    Movement,
    Mated,
    // pinnedPiece,
}

#[derive(Clone, PartialEq, Debug)]
pub struct Game {
    pub board: [[Option<Piece>; 8]; 8],
    pub turn: Color, 
    finished: bool, 
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
                    Some(Piece{piece: PieceType::Rook, color: Color::Black}),  
                    Some(Piece{piece: PieceType::Knight, color: Color::Black}), 
                    Some(Piece{piece: PieceType::Bishop, color: Color::Black}), 
                    Some(Piece{piece: PieceType::King, color: Color::Black}), 
                    Some(Piece{piece: PieceType::Queen, color: Color::Black}), 
                    Some(Piece{piece: PieceType::Bishop, color: Color::Black}), 
                    Some(Piece{piece: PieceType::Knight, color: Color::Black}), 
                    Some(Piece{piece: PieceType::Rook, color: Color::Black}) 
                ]]
            }, 

            turn: Color::White,
            move_history: Vec::new(),
            finished: false,
        }
    }
    pub fn empty_game() -> Game {
        Game {
            board: {
                [[None; 8]; 8]
            },
            turn: Color::White,
            finished: false,
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
        if self.finished {
            return Err(MoveError::Mated);
        }
        // dbg!(mv);
        let the_piece = match self.board[mv.start_y][mv.start_x] {
            None => return Err(MoveError::NoPiece),
            Some(p) => {
                if p.color != self.turn {
                    return Err(MoveError::WrongColorPiece)
                }
                else {
                    Some(p)
                }
            },
        };
        
        let mut capture: bool = false;

        let end_square = match self.board[mv.end_y][mv.end_x] {
            None => Option::<Piece>::None, 
            Some(p) => if p.color == self.turn {
                return Err(MoveError::FriendlyFire)
            }
            else {
                capture = true;
                Some(p)
            }, 
        };
        // dbg!(the_piece);
        // dbg!(end_square);

        match self.legal_movement(&mv, &the_piece, &end_square, capture) { 
            Some(x) => {
                return Err(x);
            }
            None => {
                // just a normal move
            }
        }
        // dbg!(self.legal_movement(&mv, &the_piece, &end_square, capture));

        let saved_start: Option<Piece> = self.board[mv.start_y][mv.start_x].clone();
        let saved_end: Option<Piece> = self.board[mv.end_y][mv.end_x].clone();


        // dbg!(self.board[mv.start_y][mv.start_x]);
        // dbg!(self.board[mv.end_y][mv.end_x]);
        // potentially temporarily make the move
        self.board[mv.start_y][mv.start_x] = None;
        self.board[mv.end_y][mv.end_x] = the_piece;
        
        if self.in_check() { 
            // dbg!("thinks in check");
            self.board[mv.start_y][mv.start_x] = saved_start;
            self.board[mv.end_y][mv.end_x] = saved_end;
            return Err(MoveError::SelfCheck);
            // return some in_check error
        }
        // dbg!("after check check");
        // dbg!(self.board[mv.start_y][mv.start_x]);
        // dbg!(self.board[mv.end_y][mv.end_x]);

        // else continue to mate check
        let mut safe_move: bool = false;

        // i'm sure there is a better way or writing this mate check. The complexity is through the
        // roof here.
        // temporarily change color 
        match self.turn {
            Color::White => self.turn = Color::Black,
            Color::Black => self.turn = Color::White,
        }
        // dbg!(self.turn);
        'move_gen: for org_y in 0..8 {
            for org_x in 0..8 {
                for dest_y in 0..8 {
                    for dest_x in 0..8 {
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
                        if cur_move == (Move {start_x: 5, start_y: 1, end_x: 5, end_y: 2}) {
                            // dbg!(cur_piece);
                            // dbg!(cur_end_square);
                        }

                        if self.legal_movement(&cur_move, &cur_piece, &cur_end_square, cur_capture).is_none() {
                            let saved_start: Option<Piece> = self.board[cur_move.start_y][cur_move.start_x].clone();
                            let saved_end: Option<Piece> = self.board[cur_move.end_y][cur_move.end_x].clone();
                            if cur_move == (Move {start_x: 5, start_y: 1, end_x: 5, end_y: 2}) {
                                // dbg!("IN");
                                // dbg!(saved_start);
                                // dbg!(saved_end);
                            }
                            self.board[cur_move.start_y][cur_move.start_x] = None; 
                            self.board[cur_move.end_y][cur_move.end_x] = saved_start.clone(); 
                            if !self.in_check() {
                                safe_move = true; 
                                self.board[cur_move.start_y][cur_move.start_x] = saved_start; 
                                self.board[cur_move.end_y][cur_move.end_x] = saved_end; 
                                break 'move_gen;
                            }
                            self.board[cur_move.start_y][cur_move.start_x] = saved_start; 
                            self.board[cur_move.end_y][cur_move.end_x] = saved_end; 
                        }
                    }
                }
            }
        }
        // switch color back
        match self.turn {
            Color::White => self.turn = Color::Black,
            Color::Black => self.turn = Color::White,
        }
        // dbg!(safe_move);
        if !safe_move {
            // self.turn has won
            // signal end of game or something
            // dbg!("got here");
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
        let LEN: isize = self.move_history.len() as isize;
        for i in (LEN - min(50, LEN)..LEN).rev() {
            if self.move_history[i as usize] == MoveType::CaptureOrPawn {
                pawn_capture_move = true;
                break;
            }
        }
        if !pawn_capture_move {
            // the game is drawn
            // end game or something
            // dont know println is the best way to handle this
            println!("The game is drawn because 50 reversible moves have been played");
            self.finished = true;
            return Ok(Some(Decision::Tie));
        }
        // changing turn
        match self.turn {
            Color::White => {self.turn = Color::Black;}, 
            Color::Black => {self.turn = Color::White;}, 
        }
        return Ok(None);
    }

    // checks for move legality
    pub fn legal_movement(&mut self, mv: &Move, the_piece: &Option<Piece>, end_square: &Option<Piece>, capture: bool) -> Option<MoveError> {
        /* check possible mv errors in order */
        let board_y = 0..8;
        let board_x = 0..8;
        // if the position doesnt change
        if mv.start_x == mv.end_x && mv.start_y == mv.end_y { 
            // dbg!("thinks its same");
            return Some(MoveError::Movement);
        }
        // if the moves is in bounds
        if !board_x.contains(&mv.start_x) || !board_x.contains(&mv.end_x) || !board_y.contains(&mv.start_y) || !board_y.contains(&mv.end_y) {
            // dbg!("thinks its out of bounds");
            return Some(MoveError::OutsideBoard);
        }
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
                // dbg!("thinks its pawn");
                if (mv.end_x as isize - mv.start_x as isize).abs() != { if capture {1} else {0} } {
                    // dbg!("thinks pawn is not moving to the side");
                    return Some(MoveError::Movement);
                }
                let y_dif = mv.end_y as isize - mv.start_y as isize;
                // dbg!(y_dif);
                // todo! : make sure the pawn doesnt move back
                if mv.start_y == 1 || mv.start_y == 6 {
                    if ((y_dif > 2 || y_dif < 1) && self.turn == Color::White) || ((y_dif < -2 || y_dif > -1) && self.turn == Color::Black) {
                        // dbg!("thinks its moving more than 2 steps in y");
                        return Some(MoveError::Movement);
                    }
                }
                else {
                    // todo make the rest of the movement statements like this vvv
                    if (y_dif != 1 && self.turn == Color::White) || (y_dif != -1 && self.turn == Color::Black) {
                        // dbg!("thinks its moving more than 1 steps in y");
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

                let mut cur_x: isize =  mv.start_x as isize;
                let mut cur_y: isize = mv.start_y as isize;
                while (cur_x != mv.end_x as isize) || (cur_y != mv.end_y as isize) {
                    if  (cur_x != mv.start_x as isize) || (cur_y != mv.start_y as isize) {
                        if self.board[cur_y as usize][cur_x as usize].is_some() {
                            return Some(MoveError::BlockedPath);
                        }
                    }
                    cur_x += dx as isize;
                    cur_y += dy as isize;
                }

            }
            PieceType::Rook => {
                if mv.end_y as isize - mv.start_y as isize != 0 && mv.end_x as isize - mv.end_x as isize != 0 { 
                    return Some(MoveError::Movement);
                }

                let mut cur_x: isize = mv.start_x as isize;
                let mut cur_y: isize = mv.start_y as isize;
                while (cur_x != mv.end_x as isize) || (cur_y != mv.end_y as isize) {
                    if  (cur_x != mv.start_x as isize) || (cur_y != mv.start_y as isize) {
                        if self.board[cur_y as usize][cur_x as usize].is_some() {
                            return Some(MoveError::BlockedPath);
                        }
                    }
                    cur_x += dx as isize;
                    cur_y += dy as isize;
                }
            }
            PieceType::Queen => {
                let Y = (mv.end_y as isize - mv.start_y as isize).abs();
                let X = (mv.end_x as isize - mv.start_x as isize).abs();
                if max(X, Y) != min(X, Y) && min(X, Y) != 0 {
                    return Some(MoveError::Movement);
                }
                let mut cur_x = mv.start_x as isize;
                let mut cur_y = mv.start_y as isize;
                while (cur_x != mv.end_x as isize) || (cur_y != mv.end_y as isize) {
                    if  (cur_x != mv.start_x as isize) || (cur_y != mv.start_y as isize) {
                        if self.board[cur_y as usize][cur_x as usize].is_some() {
                            return Some(MoveError::BlockedPath);
                        }
                    }
                    cur_x += dx as isize;
                    cur_y += dy as isize;
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
        'outer: for i in 0..8 {
            for j in 0..8 {
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
        'outer: for i in 0..8 {
            let mut cur_x = king_x;
            let mut cur_y = king_y;
            let DX = dir[i as usize][0 as usize];
            let DY = dir[i as usize][1 as usize];
            while 0 <= cur_x+DX && cur_x+DX < 8 && 0 <= cur_y+DY && cur_y+DY < 8 {
                cur_x += DX;
                cur_y += DY;
                match self.board[cur_y as usize][cur_x as usize] {
                    Some(p) => if p.color != self.turn {
                        match p.piece {
                            PieceType::Rook => if min(DX.abs(), DY.abs()) == 0 {
                                // dbg!(p.piece);
                                checked = true;
                                break;
                            },
                            PieceType::Bishop => if min(DX.abs(), DY.abs()) != 0 {
                                // dbg!(p.piece);
                                checked = true;
                                break;
                            },
                            PieceType::Queen => {
                                // dbg!(p.piece);
                                checked = true;
                                break;
                            },
                            PieceType::Pawn => {
                                match self.turn {
                                    Color::White => if (cur_x - king_x).abs() == 1 && cur_y - king_y == 1 {
                                        // dbg!(p.piece);
                                        checked = true;
                                        break;
                                    }
                                    Color::Black => if (cur_x - king_x).abs() == 1 && cur_y - king_y == -1 {
                                        // dbg!(p.piece);
                                        checked = true;
                                        break;
                                    }
                                }
                            }
                            PieceType::King => if max((cur_x-king_x).abs(), (cur_y-king_y).abs()) == 1 {
                                checked = true;
                            },
                            _ => {},
                        }
                    }
                    else {
                        break;
                    },
                    None => {},
                }
            }
        }
        // check 1 knight move away
        let knight_dir: [[isize; 2]; 8] = [[2, -1], [2, 1], [-2, -1], [-2, 1], [1, 2], [1, -2], [-1, 2], [-1, -2]];
        'outer: for i in 0..8 {
            let cur_x = king_x + knight_dir[i as usize][0];
            let cur_y = king_y + knight_dir[i as usize][1];
            if 0 > cur_x || cur_x > 7 || 0 > cur_y || cur_y > 7 {
                continue;
            }
            if self.board[cur_y as usize][cur_x as usize] != None && self.board[cur_y as usize][cur_x as usize].unwrap().color != self.turn {
                checked = true;
                break 'outer;
            }
        }
        return checked;
    }

    pub fn game_from_fen(s: &str) -> Game {
        let mut g = Game::empty_game();
        let mut row: isize = 7;
        let mut col: isize = 7; 
        let mut space_found = false;
        for x in s.chars() {
            if space_found {
                match x {
                    'w' => {
                        g.turn = Color::White;
                    }
                    'b' => {
                        g.turn = Color::Black;
                    }
                    _ => {

                    }
                }
                break;
            }
            match x {
                ' ' => {
                    // dbg!(1);
                    space_found = true;
                }
                '/' => {
                    // dbg!(2);
                    row -= 1;
                    col = 7;
                    continue;
                }
                'P' => {
                    // dbg!(3);
                    g.board[row as usize][col as usize] = Some(Piece{piece: PieceType::Pawn, color: Color::White});
                }
                'p' => {
                    // dbg!(4);
                    g.board[row as usize][col as usize] = Some(Piece{piece: PieceType::Pawn, color: Color::Black});
                }
                'N' => {
                    // dbg!(5);
                    g.board[row as usize][col as usize] = Some(Piece{piece: PieceType::Knight, color: Color::White});
                }
                'n' => {
                    // dbg!(6);
                    g.board[row as usize][col as usize] = Some(Piece{piece: PieceType::Knight, color: Color::Black});
                }
                'B' => {
                    // dbg!(7);
                    g.board[row as usize][col as usize] = Some(Piece{piece: PieceType::Bishop, color: Color::White});
                }
                'b' => {
                    // dbg!(8);
                    g.board[row as usize][col as usize] = Some(Piece{piece: PieceType::Bishop, color: Color::Black});
                }
                'R' => {
                    // dbg!(9);
                    g.board[row as usize][col as usize] = Some(Piece{piece: PieceType::Rook, color: Color::White});
                }
                'r' => {
                    // dbg!(1);
                    g.board[row as usize][col as usize] = Some(Piece{piece: PieceType::Rook, color: Color::Black});
                }
                'Q' => {
                    // dbg!(12);
                    g.board[row as usize][col as usize] = Some(Piece{piece: PieceType::Queen, color: Color::White});
                }
                'q' => {
                    // dbg!(13);
                    g.board[row as usize][col as usize] = Some(Piece{piece: PieceType::Queen, color: Color::Black});
                }
                'K' => {
                    // dbg!(14);
                    g.board[row as usize][col as usize] = Some(Piece{piece: PieceType::King, color: Color::White});
                }
                'k' => {
                    // dbg!(15);
                    g.board[row as usize][col as usize] = Some(Piece{piece: PieceType::King, color: Color::Black});
                }
                '1' => {
                    col -= 1;
                    continue;
                }
                '2' => {
                    col -= 2;
                    continue;
                }
                '3' => {
                    col -= 3;
                    continue;
                }
                '4' => {
                    col -= 4;
                    continue;
                }
                '5' => {
                    col -= 5;
                    continue;
                }
                '6' => {
                    col -= 6;
                    continue;
                }
                '7' => {
                    col -= 7;
                    continue;
                }
                '8' => {
                    col -= 8;
                    continue;

                }
                _ => {}
            }
            col -= 1;
        }
        // i guess move history can be ignored for this
        return g;
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn check_new_game() {
        let base_new_game = Game::new_game(); 
        let fen_game = Game::game_from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
        assert_eq!(base_new_game, fen_game);
    }

    // #[test]
    pub fn check_new_game_wrong() {
        let base_new_game = Game::new_game(); 
        let fen_game = Game::game_from_fen("Rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
        assert_eq!(base_new_game, fen_game);
    }

    #[test]
    pub fn pawn_move() {
        let mut pawn_move = Game::new_game();
        pawn_move.do_move(Move{start_x: 3, start_y: 1, end_x: 3, end_y: 3});
        let fen_game = Game::game_from_fen("rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1");
        assert_eq!(pawn_move.board, fen_game.board);
    }
    
    // #[test]
    pub fn doesnt_move() {
        let mut pawn_move = Game::new_game();
        pawn_move.do_move(Move{start_x: 3, start_y: 1, end_x: 3, end_y: 3});
        let base_game = Game::game_from_fen("Rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
        assert_eq!(pawn_move.board, base_game.board);
    }

    #[test]
    pub fn knight_move() {
        let mut knight_move = Game::new_game();
        knight_move.do_move(Move { start_x: 1, start_y: 0, end_x: 2, end_y: 2});
        let fen_game = Game::game_from_fen("rnbqkbnr/pppppppp/8/8/8/5N2/PPPPPPPP/RNBQKB1R b KQkq - 1 1");
        assert_eq!(knight_move.board, fen_game.board);
    }

    #[test]
    pub fn italian_game() {
        let mut italian_game = Game::new_game();
        italian_game.do_move(Move{start_x: 3, start_y: 1, end_x: 3, end_y: 3});
        italian_game.do_move(Move { start_x: 3, start_y: 6, end_x: 3, end_y: 4});
        italian_game.do_move(Move{start_x: 1, start_y: 0, end_x: 2, end_y: 2});
        italian_game.do_move(Move{start_x: 6, start_y: 7, end_x: 5, end_y: 5});
        italian_game.do_move(Move{start_x: 2, start_y: 0, end_x: 5, end_y: 3});
        // let fen_game = Game::game_from_fen("r1bqkbnr/pppp1ppp/2n5/4p3/2B1P3/5N2/PPPP1PPP/RNBQK2R b KQkq - 3 3");
        let fen_game = Game::game_from_fen("r1bqkbnr/pppp1ppp/2n5/4p3/2B1P3/5N2/PPPP1PPP/RNBQK2R b KQkq - 3 3");
        assert_eq!(italian_game.board, fen_game.board);
    }

    #[test]
    pub fn pin_ruy_lopez() {
        let mut pin_ruy_lopez = Game::new_game();
        pin_ruy_lopez.do_move(Move{start_x: 3, start_y: 1, end_x: 3, end_y: 3});
        pin_ruy_lopez.do_move(Move { start_x: 3, start_y: 6, end_x: 3, end_y: 4});
        pin_ruy_lopez.do_move(Move{start_x: 1, start_y: 0, end_x: 2, end_y: 2});
        pin_ruy_lopez.do_move(Move{start_x: 6, start_y: 7, end_x: 5, end_y: 5});
        pin_ruy_lopez.do_move(Move{start_x: 2, start_y: 0, end_x: 6, end_y: 4});
        pin_ruy_lopez.do_move(Move{start_x: 4, start_y: 6, end_x: 4, end_y: 5});
        pin_ruy_lopez.do_move(Move{start_x: 6, start_y: 0, end_x: 5, end_y: 2});
        pin_ruy_lopez.do_move(Move{start_x: 5, start_y: 5, end_x: 4, end_y: 3});
        pin_ruy_lopez.do_move(Move{start_x: 7, start_y: 6, end_x: 7, end_y: 5});
        let fen_game = Game::game_from_fen("r1bqkbnr/1pp2ppp/p1np4/1B2p3/4P3/2N2N2/PPPP1PPP/R1BQK2R w KQkq - 0 5");
        assert_eq!(pin_ruy_lopez.board, fen_game.board);
    }
    #[test]
    pub fn fools_mate() {
        let mut fools_mate = Game::new_game();
        fools_mate.do_move(Move{start_x: 3, start_y: 1, end_x: 3, end_y: 3});
        fools_mate.do_move(Move{start_x: 1, start_y: 6, end_x: 1, end_y: 4});
        fools_mate.do_move(Move{start_x: 4, start_y: 1, end_x: 4, end_y: 3});
        fools_mate.do_move(Move{start_x: 2, start_y: 6, end_x: 2, end_y: 5});
        fools_mate.do_move(Move{start_x: 4, start_y: 0, end_x: 0, end_y: 4});
        let fen_game = Game::game_from_fen("rnbqkbnr/ppppp2p/5p2/6pQ/3PP3/8/PPP2PPP/RNB1KBNR b KQkq - 1 3");
        assert_eq!(fools_mate.board, fen_game.board);
    }
    
    #[test]
    pub fn move_after_mate() {
        let mut move_after_mate = Game::new_game();
        move_after_mate.do_move(Move{start_x: 3, start_y: 1, end_x: 3, end_y: 3});
        move_after_mate.do_move(Move{start_x: 1, start_y: 6, end_x: 1, end_y: 4});
        move_after_mate.do_move(Move{start_x: 4, start_y: 1, end_x: 4, end_y: 3});
        move_after_mate.do_move(Move{start_x: 2, start_y: 6, end_x: 2, end_y: 5});
        move_after_mate.do_move(Move{start_x: 4, start_y: 0, end_x: 0, end_y: 4});
        move_after_mate.do_move(Move{start_x: 4, start_y: 0, end_x: 0, end_y: 4});
        move_after_mate.do_move(Move{start_x: 4, start_y: 0, end_x: 0, end_y: 4});
        let fen_game = Game::game_from_fen("rnbqkbnr/ppppp2p/5p2/6pQ/3PP3/8/PPP2PPP/RNB1KBNR b KQkq - 1 3");
        assert_eq!(move_after_mate.board, fen_game.board);
    }

    #[test]
    pub fn sic_queen() {
        let mut sic_queen = Game::new_game();
        sic_queen.do_move(Move{start_x: 3, start_y: 1, end_x: 3, end_y: 3});
        sic_queen.do_move(Move{start_x: 5, start_y: 6, end_x: 5, end_y: 4});
        sic_queen.do_move(Move{start_x: 4, start_y: 1, end_x: 4, end_y: 2});
        sic_queen.do_move(Move{start_x: 4, start_y: 7, end_x: 7, end_y: 4});
        sic_queen.do_move(Move{start_x: 5, start_y: 1, end_x: 5, end_y: 2});
        let fen_game = Game::game_from_fen("rnb1kbnr/pp1ppppp/8/q1p5/4P3/2PP4/PP3PPP/RNBQKBNR b KQkq - 0 3");
        assert_eq!(sic_queen.board, fen_game.board);
    }
}
