use std::fmt;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Player {
    Alice,
    Bob
}

type Position = Option<Player>;

#[derive(Debug)]
pub enum MoveError {
    NonEmptyPosition,
    InvalidPosition
}

#[derive(Copy, Clone, Debug)]
struct Coordinate {
    x: usize,
    y: usize
}

impl Coordinate {
    // this underflows on -1, but it will never overflow on 8.
    /// Returns none if you try to go under -1 for either x or y
    pub fn forward(self: &Self, direction: &Direction) -> Option<Coordinate> {
        match *direction {
            Direction::South | Direction::Southwest | Direction::Southeast if self.y == 0 => { return None; },
            Direction::West | Direction::Northwest | Direction::Southwest if self.x == 0 => { return None; },
            _ => {}
        }

        let coordinates = match *direction {
            Direction::North => { Coordinate { x: self.x, y: self.y + 1 } },
            Direction::South => { Coordinate { x: self.x, y: self.y - 1 } },
            Direction::East => { Coordinate { x: self.x + 1, y: self.y } },
            Direction::West => { Coordinate { x: self.x - 1, y: self.y } },
            Direction::Northwest => { Coordinate { x: self.x - 1, y: self.y + 1 } },
            Direction::Northeast => { Coordinate { x: self.x + 1, y: self.y + 1 } },
            Direction::Southwest => { Coordinate { x: self.x - 1, y: self.y - 1 } },
            Direction::Southeast => { Coordinate { x: self.x + 1, y: self.y - 1 } }
        };

        Some(coordinates)
    }
}

enum Direction {
    North,
    South,
    East,
    West,
    Northwest,
    Northeast,
    Southwest,
    Southeast
}

static ALL_DIRECTIONS: [Direction; 8] = [Direction::North, Direction::South, Direction::East, Direction::West,
         Direction::Northwest, Direction::Northeast, Direction::Southwest, Direction::Southeast];

#[derive(Clone, Debug)]
pub struct Game {
    current_player: Player,
    board: [[Position; 8]; 8]
}

impl Default for Game {
    fn default() -> Self {
        let mut board = Game {
            current_player: Player::Alice,
            board: [[None; 8]; 8]
        };

        board.set_position(&Coordinate { x: 3, y: 4 }, Player::Alice);
        board.set_position(&Coordinate { x: 4, y: 4 }, Player::Bob);
        board.set_position(&Coordinate { x: 3, y: 3 }, Player::Bob);
        board.set_position(&Coordinate { x: 4, y: 3 }, Player::Alice);

        board
    }
}

impl fmt::Display for Game {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "x -> first player; o -> second player\n\n");

        for y in 0..8 {
            for x in 0..8 {
                match self.position(&Coordinate{x:x,y:y}) {
                    None => write!(formatter, " {},{} ", x+1, y+1),
                    Some(Player::Alice) => write!(formatter, "  {}  ", "x"),
                    Some(Player::Bob) => write!(formatter, "  {}  ", "o")
                };
            }
            write!(formatter, "\n");
        }

        write!(formatter, "")
    }
}

impl Game {
    // TODO: create a Move object, instead of passing x,y everywhere
    //       or rename Position to CellState and use Position to represent x,y

    fn other(self: &Self, p : Player) -> Player {
        match p {
            Player::Alice => Player::Bob,
            Player::Bob => Player::Alice
        }
    }

    fn change_player(self: &mut Self) {
        self.current_player = self.other(self.current_player);
    }

    fn valid_moves(self: &Self, p: Player) -> Vec<Coordinate> {
        let mut moves: Vec<Coordinate> = Vec::new();

        for x in 0..8 {
            for y in 0..8 {
                let c = Coordinate{x:x, y:y};
                if let Ok(_) = self.is_valid_move(&c, p) {
                    moves.push(c)
                }
            }
        }

        moves
    }

    fn is_valid_move(self: &Self, c: &Coordinate, p: Player) -> Result<Vec<Coordinate>,MoveError> {
        if !self.within_bounds(c) {
            return Err(MoveError::InvalidPosition)
        }

        match self.board[c.x][c.y] {
            None => {
                let cs: Vec<Coordinate> = ALL_DIRECTIONS.iter()
                    .flat_map(|ref d| self.raytrace(c, &d, p))
                    .collect();

                if cs.is_empty() {
                    Err(MoveError::InvalidPosition)
                } else {
                    Ok(cs)
                }

            },
            _ => Err(MoveError::NonEmptyPosition)
        }
    }

    fn within_bounds(self: &Self, coordinates: &Coordinate) -> bool {
        coordinates.x < 8 && coordinates.y < 8
    }

    fn check_position_against_player(self: &Self, c: Option<Coordinate>, player: Player) -> bool {
        c.and_then(|c| Some(self.within_bounds(&c) && self.position(&c).and_then(|p| Some(p == player)).unwrap_or(false))).unwrap_or(false)
    }

    fn raytrace(self: &Self, c: &Coordinate, direction: &Direction, player: Player) -> Vec<Coordinate> {
        let mut found = 0;
        let mut c = c.forward(&direction); // burrito monad
        let mut csgo: Vec<Coordinate> = Vec::new();

        while self.check_position_against_player(c, self.other(player)) {
            csgo.push(c.unwrap());
            found += 1;
            c = c.unwrap().forward(&direction);
        }

        if found > 0 && self.check_position_against_player(c, player) {
            csgo
        } else {
            vec![]
        }
    }

    pub fn finished(self: &Self) -> bool {
        self.valid_moves(Player::Alice).is_empty() &&
          self.valid_moves(Player::Bob).is_empty()
    }

    pub fn make_move(self: &mut Self, x: usize, y: usize) -> Result<Player,MoveError> {
        let c = Coordinate { x: x, y: y };

        let validity = self.is_valid_move(&c, self.current_player);

        match validity {
          Ok(cs) => {
            let p = self.current_player;
            self.set_position(&c, p);
            self.change_player();

            for c in cs {
                self.set_position(&c, p);
            }

            Ok(self.current_player)
          },
          Err(x) => Err(x)
        }
    }

    fn position(self: &Self, c: &Coordinate) -> Position {
        self.board[c.x][c.y]
    }

    fn set_position(self: &mut Self, c: &Coordinate, p: Player) {
        self.board[c.x][c.y] = Some(p);
    }

    pub fn score(self: &Self) -> [i32; 2] {
        let mut result: [i32; 2] = [0, 0];

        for row in self.board.iter() {
            for cell in row.iter() {
                match *cell {
                    None => {},
                    Some(Player::Alice) => { result[0] = result[0] + 1 },
                    Some(Player::Bob)   => { result[1] = result[1] + 1 }
                }
            }
        }

        // over-the-board rule
        if result[0] == 0 { result[1] == 64; }
        if result[1] == 0 { result[0] == 64; }

        result
    }
}

#[test]
fn it_initializes_an_empty_board() {
    let board = Game::default();

    assert!(board.board.len() == 8, "Board should be 8x8.");

    for row in board.board.iter() {
        assert!(row.len() == 8, "Board should be 8x8.");
    }
}

#[test]
fn it_is_alice_to_move() {
    let board = Game::default();

    match board.current_player {
        Player::Alice => { assert!(true) },
        _ => { panic!("oh god") }
    }
}

#[test]
fn it_initializes_the_center() {
    let board = Game::default();

    assert!(board.position(&Coordinate { x: 3, y: 4 }).unwrap() == Player::Alice);
    assert!(board.position(&Coordinate { x: 4, y: 3 }).unwrap() == Player::Alice);
    assert!(board.position(&Coordinate { x: 3, y: 3 }).unwrap() == Player::Bob);
    assert!(board.position(&Coordinate { x: 4, y: 4 }).unwrap() == Player::Bob);
}

#[test]
fn it_does_not_allow_playing_on_an_occuppied_square() {
    let mut board = Game::default();

    match board.make_move(3,3) {
        Err(MoveError::NonEmptyPosition) => { assert!(true) }
        _ => { assert!(false) }
    }
}

#[test]
fn it_does_not_allow_playing_unless_it_is_a_valid_move() {
    let mut board = Game::default();

    match board.make_move(5,4) {
        Ok(_) => { assert!(true) }
        _ => { assert!(false) }
    }

    match board.make_move(4,1) {
        Err(MoveError::InvalidPosition) => { assert!(true) }
        _ => { assert!(false) }
    }
}


#[test]
fn it_changes_player_on_valid_move() {
    let mut board = Game::default();

    if let Ok(_) = board.make_move(5,4) {};
    assert!(Player::Bob == board.current_player);
}

#[test]
fn it_does_not_change_player_on_invalid_move() {
    let mut board = Game::default();

    if let Ok(_) = board.make_move(4,4) {};
    assert!(Player::Alice == board.current_player);
}

#[test]
fn it_detects_out_of_bounds() {
    let mut board = Game::default();

    if let Ok(_) = board.make_move(1004,1004) { assert!(false) };
}

#[test]
fn it_returns_other_player() {
    let board = Game::default();

    assert!(Player::Bob == board.other(Player::Alice));
    assert!(Player::Alice == board.other(Player::Bob));
}

#[test]
fn it_raytraces() {
    let board = Game::default();
    let cs = board.raytrace(&Coordinate { x: 5, y: 4 }, &Direction::West, Player::Alice);

    assert!(!cs.is_empty());

    match cs.first() {
        Some(c) => {
            assert!(4 == c.x);
            assert!(4 == c.y);
        },
        None => assert!(false)
    }
}

#[test]
fn it_flips_a_single_piece() {
    let mut board = Game::default();
    let flipped_piece = Coordinate { x: 4, y: 4 };

    if let Ok(_) = board.make_move(5,4) {};
    println!("{}", board);

    match board.position(&flipped_piece).unwrap() {
        Player::Alice => assert!(true),
        _ => assert!(false, "Piece should be flipped.")
    }

    if let Ok(_) = board.make_move(5,5) {};
    println!("{}", board);
}

#[test]
fn it_calculates_score_tie() {
    let mut board = Game::default();
    let score: [i32; 2] = board.score();

    assert!(2 == score[0]);
    assert!(2 == score[1]);
}