use crossterm::{
    style::{Color, Colors, Print, ResetColor, SetColors},
    ExecutableCommand,
};
use std::env;
use std::io::stdout;

const QUEEN: &str = "<W>";
const EMPTY: &str = "   ";

/// Adapted from reading "Algorithms" by Jeff Erickson
/// freely available on http://jeffe.cs.illinois.edu/teaching/algorithms/
pub fn main() {
    let args: Vec<String> = env::args().collect();

    // nothing fancy at all, just brute-force it
    if args.len() < 2 || args.len() > 3 {
        println!("Usage: {} n [show]", args[0]);
        println!("  n = size of chessboard");
        println!("  [show] to actually see each chessboard solution");
        return;
    }

    let boardsize = args[1].parse::<usize>().unwrap();
    let chessboard = vec![-1i8; boardsize];
    let showboard = args.len() == 3; // don't even care what it is

    place_queens(chessboard.clone(), 0, showboard);
}

fn show_chessboard(board: Vec<i8>, showboard: bool) {
    let mut light: bool = true;

    if showboard {
        for pos in board.to_vec() {
            for cell in 0..board.len() {
                match (pos == cell as i8, light) {
                    (true, true) => draw_square(Colors::new(Color::Black, Color::Grey), QUEEN),
                    (true, false) => draw_square(Colors::new(Color::Grey, Color::Black), QUEEN),
                    (false, true) => draw_square(Colors::new(Color::Black, Color::Grey), EMPTY),
                    (false, false) => draw_square(Colors::new(Color::Grey, Color::Black), EMPTY),
                };
                light = !light;
            }
            println!();
            if board.len() % 2 == 0 {
                // to checkerboard even-sized boards
                light = !light;
            }
        }
        println!("\n");
    } else {
        let adjusted: Vec<i8> = board.iter().map(|&p| p + 1).collect(); // lets remove the 0-based confusion
        println!("{:?}", adjusted);
    }
}

fn draw_square(color: Colors, chesspiece: &str) {
    let mut stdout = stdout();

    stdout.execute(SetColors(color)).unwrap();
    stdout.execute(Print(chesspiece)).unwrap();
    stdout.execute(ResetColor).unwrap();
}

/// n Queens Problem
/// Section2.1, page 69-71
fn place_queens(mut chessboard: Vec<i8>, row: usize, showboard: bool) {
    if row == chessboard.len() {
        show_chessboard(chessboard.to_vec(), showboard);
        return;
    }
    for column in 0..chessboard.len() {
        let mut legal = true;
        for cell in 0..row {
            let pos = cell as usize;
            if chessboard[pos] == (column as i8)
                || (column + row >= cell && chessboard[pos] == ((column + row - cell) as i8))
                || (column + cell >= row && chessboard[pos] == ((column + cell - row) as i8))
            {
                legal = false;
            }
        }
        if legal {
            chessboard[row] = column as i8;
            place_queens(chessboard.clone(), row + 1, showboard);
        }
    }
}
