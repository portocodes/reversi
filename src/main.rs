extern crate reversi;

use std::io;
use std::io::Write;
use reversi::{Game};

fn main() {
    let mut stdin = io::stdin();
    let input = &mut String::new();
    let mut board = Game::default();
    

    //while !board.finished() {
    loop {
        input.clear();
        println!("{}", board);
        print!("Player 1> ");
        io::stdout().flush();
        stdin.read_line(input);
        
        if input == "sext\n" {
            std::process::exit(0)
        };
        
        let coordinates: Vec<&str> = input.trim().split(",").collect();
        let y = usize::from_str_radix(coordinates.first().unwrap(), 10).unwrap() - 1;
        let x = usize::from_str_radix(coordinates.last().unwrap(), 10).unwrap() - 1;
        println!("-> {},{} <-", x,y);
        
        let blurp = board.make_move(x, y);
        
        match blurp {
            Ok(c) => println!("{:?}", c),
            _ => {}
        };

        // read input
        // make move
        // print result
    }
}
