#[derive(Copy, Clone, Debug, PartialEq)]
enum Player {
    Alice,
    Bob
}

type Position = Option<Player>;

#[derive(Debug)]
enum MoveError {
    NonEmptyPosition
}

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

        if let Ok(_) = board.make_move(3, 3) {};
        if let Ok(_) = board.make_move(3, 4) {};
        if let Ok(_) = board.make_move(4, 4) {};
        if let Ok(_) = board.make_move(4, 3) {};

        board
    }
}

impl Board {
    fn change_player(self: &mut Self) {
      match self.current_player {
          Player::Alice => self.current_player = Player::Bob,
          Player::Bob => self.current_player = Player::Alice
      };
    }

    pub fn make_move(self: &mut Self, x: usize, y: usize) -> Result<Player,MoveError> {
        let result = match self.board[x][y] {
            None => {
                self.board[x][y] = Some(self.current_player);
                Ok(self.current_player)
            },
            _ => Err(MoveError::NonEmptyPosition)
        };

        self.change_player();

        result
    }

    // immutable one
    pub fn position(self: &Self, x: usize, y: usize) -> Position {
        self.board[x][y]
    }
}

#[test]
fn it_works() {
    assert!(true);
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

    assert!(board.position(3, 3).unwrap() == Player::Alice);
    assert!(board.position(4, 4).unwrap() == Player::Alice);
    assert!(board.position(3, 4).unwrap() == Player::Bob);
    assert!(board.position(4, 3).unwrap() == Player::Bob);
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
fn it_allows_playing_on_empty_square() {
    let mut board = Board::default();

    match board.make_move(0,0) {
        Ok(_) => { assert!(true) }
        Err(_) => { assert!(false) }
    }
}
