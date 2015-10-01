#[derive(Copy, Clone, Debug, PartialEq)]
enum Player {
    Alice,
    Bob
}

type Position = Option<Player>;

#[derive(Debug)]
enum MoveError {
    NonEmptyPosition,
    InvalidPosition
}

#[derive(Copy, Clone, Debug)]
struct Coordinates {
    x: usize,
    y: usize
}

impl Coordinates {
    pub fn forward(self: &Self, direction: &Direction) -> Coordinates {
        match *direction {
            Direction::North => { Coordinates { x: self.x, y: self.y + 1 } },
            Direction::South => { Coordinates { x: self.x, y: self.y - 1 } },
            Direction::East => { Coordinates { x: self.x + 1, y: self.y } },
            Direction::West => { Coordinates { x: self.x - 1, y: self.y } },
            Direction::Northwest => { Coordinates { x: self.x - 1, y: self.y + 1 } },
            Direction::Northeast => { Coordinates { x: self.x + 1, y: self.y + 1 } },
            Direction::Southwest => { Coordinates { x: self.x - 1, y: self.y - 1 } },
            Direction::Southeast => { Coordinates { x: self.x + 1, y: self.y - 1 } }
        }
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
struct Board {
    current_player: Player,
    board: [[Position; 8]; 8]
}

impl Default for Board {
    fn default() -> Self {
        let mut board = Board {
            current_player: Player::Alice,
            board: [[None; 8]; 8]
        };

        board.set_position(&Coordinates { x: 3, y: 4 }, Player::Alice);
        board.set_position(&Coordinates { x: 4, y: 4 }, Player::Bob);
        board.set_position(&Coordinates { x: 3, y: 3 }, Player::Bob);
        board.set_position(&Coordinates { x: 4, y: 3 }, Player::Alice);

        board
    }
}

impl Board {
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

    pub fn is_valid_move(self: &Self, cs: &mut Vec<Coordinates>, c: &Coordinates) -> Result<Player,MoveError> {
        match self.board[c.x][c.y] {
            None => {
                let yips = ALL_DIRECTIONS.iter()
                    .map(|ref d| self.raytrace(cs, c, &d, self.current_player))
                    .fold(false, |acc, item| acc || item);

                if yips {
                    Ok(self.current_player)
                } else {
                    Err(MoveError::InvalidPosition)
                }

            },
            _ => Err(MoveError::NonEmptyPosition)
        }
    }

    fn within_bounds(self: &Self, coordinates: &Coordinates) -> bool {
        coordinates.x < 8 && coordinates.y < 8
    }

    fn check_position_against_player(self: &Self, c: &Coordinates, player: Player) -> bool {
        self.within_bounds(c) && self.position(c).and_then(|p| Some(p == player)).unwrap_or(false)
    }

    fn raytrace(self: &Self, cs: &mut Vec<Coordinates>, c: &Coordinates, direction: &Direction, player: Player) -> bool {
        let mut found = 0;
        let mut c = c.forward(&direction); // burrito monad

        while self.check_position_against_player(&c, self.other(player)) {
            cs.push(c);
            found += 1;
            c = c.forward(&direction);
        }

        found > 0 && self.check_position_against_player(&c, player)
    }

    pub fn make_move(self: &mut Self, x: usize, y: usize) -> Result<Player,MoveError> {
        let mut cs: Vec<Coordinates> = Vec::new();
        let c = Coordinates { x: x, y: y };

        let validity = self.is_valid_move(&mut cs, &c);

        if let Ok(_) = validity {
            let p = self.current_player;
            self.set_position(&c, p);
            self.change_player();

            for c in cs {
                self.set_position(&c, p);
            }
        }

        validity
    }

    pub fn position(self: &Self, c: &Coordinates) -> Position {
        self.board[c.x][c.y]
    }

    pub fn set_position(self: &mut Self, c: &Coordinates, p: Player) {
        self.board[c.x][c.y] = Some(p);
    }
}

#[test]
fn it_initializes_an_empty_board() {
    let board = Board::default();

    assert!(board.board.len() == 8, "Board should be 8x8.");

    for row in board.board.iter() {
        assert!(row.len() == 8, "Board should be 8x8.");
    }
}

#[test]
fn it_is_alice_to_move() {
    let board = Board::default();

    match board.current_player {
        Player::Alice => { assert!(true) },
        _ => { panic!("oh god") }
    }
}

#[test]
fn it_initializes_the_center() {
    let board = Board::default();

    assert!(board.position(&Coordinates { x: 3, y: 4 }).unwrap() == Player::Alice);
    assert!(board.position(&Coordinates { x: 4, y: 3 }).unwrap() == Player::Alice);
    assert!(board.position(&Coordinates { x: 3, y: 3 }).unwrap() == Player::Bob);
    assert!(board.position(&Coordinates { x: 4, y: 4 }).unwrap() == Player::Bob);
}

#[test]
fn it_does_not_allow_playing_on_an_occuppied_square() {
    let mut board = Board::default();

    match board.make_move(3,3) {
        Err(MoveError::NonEmptyPosition) => { assert!(true) }
        _ => { assert!(false) }
    }
}

#[test]
fn it_does_not_allow_playing_unless_it_is_a_valid_move() {
    let mut board = Board::default();

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
    let mut board = Board::default();

    if let Ok(_) = board.make_move(5,4) {};
    assert!(Player::Bob == board.current_player);
}

#[test]
fn it_does_not_change_player_on_invalid_move() {
    let mut board = Board::default();

    if let Ok(_) = board.make_move(4,4) {};
    assert!(Player::Alice == board.current_player);
}

#[test]
fn it_returns_other_player() {
    let board = Board::default();

    assert!(Player::Bob == board.other(Player::Alice));
    assert!(Player::Alice == board.other(Player::Bob));
}

#[test]
fn it_raytraces() {
    let board = Board::default();
    let mut cs = Vec::new();

    assert!(board.raytrace(&mut cs, &Coordinates { x: 5, y: 4 }, &Direction::West, Player::Alice));

    match cs.first() {
        Some(c) => {
            assert!(4 == c.x);
            assert!(4 == c.y);
        },
        None => assert!(false)
    }
}

fn it_flips_a_single_piece() {
    let mut board = Board::default();
    let flipped_piece = Coordinates { x: 4, y: 4 };

    if let Ok(_) = board.make_move(5,4) {};

    match board.position(&flipped_piece).unwrap() {
        Player::Alice => assert!(true),
        _ => assert!(false, "Piece should be flipped.")
    }
}