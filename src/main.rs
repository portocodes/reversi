extern crate reversi;

use std::io;
use std::io::Write;
use reversi::{Game};

fn main() {
    let mut stdin = io::stdin();
    let input = &mut String::new();
    let mut board = Game::default();


    while !board.finished() {
        input.clear();
        println!("{}", board);
        print!("Player 1> ");
        io::stdout().flush();
        stdin.read_line(input);
        println!("");

        if input == "" {
            println!("Exiting...");
            std::process::exit(0);
        }
        if input == "sext\n" {
            break
        };

        let coordinates: Vec<&str> = input.trim().split(",").collect();

        let uint2coord = |x| usize::from_str_radix(x, 10).ok().and_then(|x| Some(x - 1));

        let blurp = coordinates.first().and_then(|x| uint2coord(x))
            .and_then(|x|
                coordinates.last()
                    .and_then(|y| uint2coord(y))
                     .and_then(|y| Some((x,y))))
                       .and_then(|p| board
                        .make_move(p.0, p.1).ok());

        match blurp {
            Some(c) => println!("{:?}", c),
            _ => {}
        };
    }

    let score = board.score();

    println!("Game over!");
    println!("Score was:\n\tPlayer 1 - {}\n\tPlayer 2 - {}", score[0], score[1]);
}
