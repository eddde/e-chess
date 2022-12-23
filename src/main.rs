mod cpu;
mod evalfunc;
mod moves;
mod types;
use ansi_term::Colour::{Red, White, RGB};
use cpu::get_all_moves;
use moves::*;
use std::fmt::format;
use std::fs;
use std::io::{stdin, Write};
use types::*;

fn main() {
    clear_screen();
    let board = Board::new();
    arrow_print("Welcome to E-Chess!", true);
    arrow_print("Input 'exit' to exit the application at anytime.", false);
    arrow_print(
        "What do you want to play?\n\n(1) Local Multiplayer\n(2) Computer vs Computer\n(3) Singleplayer vs Computer\n",
        false,
    );

    loop {
        print!("{} ", White.bold().paint(">>>"));
        std::io::stdout().flush().unwrap();
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        match input.trim().parse::<usize>().unwrap() {
            1 => mp_game_loop(board),
            2 => pc_game_loop(board),
            3 => sp_game_loop(board),
            _ => {
                arrow_print("Invalid input!", true);
                continue;
            }
        }
    }
}

fn mp_game_loop(mut board: Board) {
    clear_draw(board, true);
    loop {
        new_turn(&mut board, true, true, None);
        if let Some(winner) = check_for_mates(board) {
            clear_draw(board, true);
            arrow_print(&format!("{} Wins!", winner.ctos()), true);
            break;
        }

        new_turn(&mut board, false, true, None);
        if let Some(winner) = check_for_mates(board) {
            clear_draw(board, false);
            arrow_print(&format!("{} Wins!", winner.ctos()), true);
            break;
        }
    }

    arrow_print("Hit Enter to exit . . .", true);
    stdin().read_line(&mut String::new()).unwrap();
}

fn pc_game_loop(mut board: Board) {
    arrow_print("What difficulty do you want to play at?", true);
    arrow_print(
        "The higher the difficulty, the longer the computer will think",
        true,
    );
    arrow_print(
        "A difficulty higher than 4 will result in long turns!",
        true,
    );
    arrow_print("1. Easy", false);
    arrow_print("2. Medium", false);
    arrow_print("3. Hard", false);
    arrow_print("4. Impossible", false);
    arrow_print("5. Unstoppable", false);

    let mut difficulty;
    loop {
        let mut input = String::new();
        print!(">>> ");
        std::io::stdout().flush().unwrap();
        stdin().read_line(&mut input).unwrap();
        difficulty = match input.trim().parse::<usize>() {
            Ok(n) => n,
            Err(_) => {
                arrow_print("Invalid input!", true);
                continue;
            }
        };

        if difficulty > 5 {
            arrow_print("Invalid input!", true);
            continue;
        }

        break;
    }

    let mut last_white_move = (0, 0);
    let mut white_stalemate = 0;
    let mut last_black_move = (0, 0);
    let mut black_stalemate = 0;

    // make 5 random moves to make the game more interesting
    for _ in 0..5 {
        // use fastrand crate for randomness
        let mut all_moves = get_all_moves(board, true);
        all_moves.retain(|x| x.1.len() != 0);
        let move_ = &all_moves[fastrand::usize(0..all_moves.len())];

        match move_piece(
            &mut board,
            move_.0,
            move_.1[fastrand::usize(..move_.1.len())],
            true,
        ) {
            Err(e) => {
                clear_draw(board, true);
                input_error(e);
            }

            Ok(_) => {
                clear_draw(board, true);
                arrow_print("Doing 10 random moves...", true)
            }
        }

        std::thread::sleep(std::time::Duration::from_millis(500));

        let mut all_moves = get_all_moves(board, false);
        all_moves.retain(|x| x.1.len() != 0);
        let move_ = &all_moves[fastrand::usize(0..all_moves.len())];

        match move_piece(
            &mut board,
            move_.0,
            move_.1[fastrand::usize(..move_.1.len())],
            true,
        ) {
            Err(e) => {
                clear_draw(board, true);
                input_error(e);
            }

            Ok(_) => {
                clear_draw(board, true);
                arrow_print("Doing 10 random moves...", true)
            }
        }

        std::thread::sleep(std::time::Duration::from_millis(500));
    }

    loop {
        let white_start = std::time::Instant::now();
        let move_ = cpu::get_best_move(board, difficulty as i32, true);
        match move_piece(&mut board, move_.0, move_.1, true) {
            Err(e) => {
                clear_draw(board, true);
                input_error(e);
            }

            Ok(_) => {}
        }
        if move_.1 == last_white_move {
            white_stalemate += 1;
        } else {
            white_stalemate = 0;
        }
        last_white_move = move_.0;

        if let Some(winner) = check_for_mates(board) {
            clear_draw(board, true);
            arrow_print(&format!("{} Wins!", winner.ctos()), true);
            break;
        }
        let to_piece = board.tiles[move_.1 .0][move_.1 .1].piece.piece_type;
        clear_draw(board, true);
        print!(
            "{} White moved: {} to {} after {:?}\n",
            Red.bold().paint(">>>"),
            Red.bold().paint(to_piece.ttos()),
            Red.bold().paint(reverse_match_input(move_.1)),
            white_start.elapsed()
        );
        println!("{} Black is thinking...", Red.bold().paint(">>>"));
        if white_stalemate >= 3 && black_stalemate >= 3 {
            clear_draw(board, true);
            arrow_print("Stalemate!", true);
            break;
        }
        let black_start = std::time::Instant::now();
        let move_ = cpu::get_best_move(board, difficulty as i32, false);
        match move_piece(&mut board, move_.0, move_.1, false) {
            Err(e) => {
                clear_draw(board, false);
                input_error(e);
            }

            Ok(_) => {}
        }
        if move_.1 == last_black_move {
            black_stalemate += 1;
        } else {
            black_stalemate = 0;
        }
        last_black_move = move_.0;
        if let Some(winner) = check_for_mates(board) {
            clear_draw(board, true);
            arrow_print(&format!("{} Wins!", winner.ctos()), true);
            break;
        }
        let to_piece = board.tiles[move_.1 .0][move_.1 .1].piece.piece_type;
        clear_draw(board, true);
        print!(
            "{} Black moved: {} to {} after {:?}\n",
            Red.bold().paint(">>>"),
            Red.bold().paint(to_piece.ttos()),
            Red.bold().paint(reverse_match_input(move_.1)),
            black_start.elapsed()
        );
        println!("{} White is thinking...", Red.bold().paint(">>>"));

        if white_stalemate >= 3 && black_stalemate >= 3 {
            clear_draw(board, true);
            arrow_print("Stalemate!", true);
            break;
        }
    }
}

fn sp_game_loop(mut board: Board) {
    arrow_print("What difficulty do you want to play at?", true);
    arrow_print(
        "The higher the difficulty, the longer the computer will think",
        true,
    );
    arrow_print(
        "A difficulty higher than 4 will result in long turns!",
        true,
    );
    arrow_print("1. Easy", false);
    arrow_print("2. Medium", false);
    arrow_print("3. Hard", false);
    arrow_print("4. Impossible", false);
    arrow_print("5. Unstoppable", false);

    let mut difficulty;
    loop {
        let mut input = String::new();
        print!(">>> ");
        std::io::stdout().flush().unwrap();
        stdin().read_line(&mut input).unwrap();
        difficulty = match input.trim().parse::<usize>() {
            Ok(n) => n,
            Err(_) => {
                arrow_print("Invalid input!", true);
                continue;
            }
        };

        if difficulty > 5 {
            arrow_print("Invalid input!", true);
            continue;
        }

        break;
    }

    clear_screen();
    arrow_print("Should game be recorded? y/n", true);

    let should_record: bool;
    loop {
        let mut input = String::new();
        print!(">>> ");
        std::io::stdout().flush().unwrap();
        stdin().read_line(&mut input).unwrap();
        let input = input.trim().to_lowercase();
        match input.as_str() {
            "y" => should_record = true,
            "n" => should_record = false,
            _ => {println!("Invalid input!"); continue;}
        }

        break;
    }

    let mut file: Option<std::fs::File>;

    if should_record {
        clear_screen();

        let problematic_chars = ['<', '>', ':', '"', '/', '\\', '|', '?', '*', '\u{1F}'];
        let file_name: String;
        loop {
            arrow_print("What should the filename be?", true);
            arrow_print("Notice that files with the same name as previous ones will be truncated", false);
            let mut input = String::new();
            print!(">>> ");
            std::io::stdout().flush().unwrap();
            stdin().read_line(&mut input).unwrap();
            let input = input.trim().to_lowercase();

            if problematic_chars.iter().any(|&v| input.contains(v)) {
                clear_screen();
                println!("Cannot use characters: ");
                for c in problematic_chars.iter() {
                    print!("{}, ", c);
                } 
                println!(" in filename");
                continue;
            } else {
                file_name = input;
            }

            break;
        }
        let file_path = (&file_name as &str).to_owned() + ".edvard";
        fs::File::create(&file_path);
        file = Some(fs::OpenOptions::new()
        .write(true)
        .append(true)
        .open(&file_path)
        .unwrap());

    } else {
        file = None;
    }

    for _ in 0..2 {
        if let Some(winner) = check_for_mates(board) {
            clear_draw(board, false);
            arrow_print(&format!("{} Wins!", winner.ctos()), true);
            break;
        }

        let mut all_moves = get_all_moves(board, true);
        all_moves.retain(|x| x.1.len() != 0);
        let move_ = &all_moves[fastrand::usize(0..all_moves.len())];
        let move_to = move_.1[fastrand::usize(..move_.1.len())];

        match move_piece(
            &mut board,
            move_.0,
            move_to,
            true,
        ) {
            Err(e) => {
                clear_draw(board, true);
                input_error(e);
            }

            Ok(_) => {
                match file {
                    Some(ref mut file) => writeln!(file, "{:?}:{:?}", move_.0, move_to).unwrap(),
                    None => {},
                }
                clear_draw(board, false);
                arrow_print("Computer is doing one of it's 2 random moves...", true)
            }
        }

        if let Some(winner) = check_for_mates(board) {
            clear_draw(board, false);
            arrow_print(&format!("{} Wins!", winner.ctos()), true);
            break;
        }

        new_turn(&mut board, false, true, file.as_mut());

        std::thread::sleep(std::time::Duration::from_millis(1000));
    }

    // actual game loop
    loop {
        if let Some(winner) = check_for_mates(board) {
            clear_draw(board, false);
            arrow_print(&format!("{} Wins!", winner.ctos()), true);
            break;
        }

        println!("{} Computer is thinking...", Red.bold().paint(">>>"));
        let black_start = std::time::Instant::now();
        let move_ = cpu::get_best_move(board, difficulty as i32, true);
        match move_piece(&mut board, move_.0, move_.1, true) {
            Err(e) => {
                clear_draw(board, false);
                input_error(e);
            }

            Ok(_) => {
                match file {
                    Some(ref mut file) => writeln!(file, "{:?}:{:?}", move_.0, move_.1).unwrap(),
                    None => {},
                }
            }
        }

        if let Some(winner) = check_for_mates(board) {
            clear_draw(board, false);
            arrow_print(&format!("{} Wins!", winner.ctos()), true);
            break;
        }

        clear_draw(board, false);
        let to_piece = board.tiles[move_.1 .0][move_.1 .1].piece.piece_type;
        print!(
            "{} Computer moved: {} to {} after {:?}\n",
            Red.bold().paint(">>>"),
            Red.bold().paint(to_piece.ttos()),
            Red.bold().paint(reverse_match_input(move_.1)),
            black_start.elapsed()
        );

        new_turn(&mut board, false, false, file.as_mut());
    }
}

fn new_turn(board: &mut Board, is_white: bool, clear: bool, write_file: Option<&mut std::fs::File>) {
    loop {
        if clear {
            clear_draw(*board, is_white)
        }

        if is_white {
            println!("White's Turn");
        } else {
            println!("Black's Turn");
        }

        let (from, to) = handle_input(*board, is_white);
        let white_moves = legal_moves(*board, from, is_white);
        if white_moves.contains(&to) {
            match move_piece(board, from, to, is_white) {
                Err(e) => {
                    clear_draw(*board, is_white);
                    input_error(e);
                }

                Ok(_) => {
                    match write_file {
                        Some(write_file) => writeln!(write_file, "{:?}:{:?}", from, to).unwrap(),
                        None => {},
                    }
                    //write_file.write_all(from + "," + to + "\n");
                    clear_draw(*board, is_white);
                    return;
                }
            }
        } else {
            clear_draw(*board, is_white);
            input_error(Error::IllegalMove);
        }
    }
}

fn handle_input(board: Board, is_white: bool) -> ((usize, usize), (usize, usize)) {
    let colour = if is_white {
        Colour::White
    } else {
        Colour::Black
    };

    loop {
        print!("{} ", White.bold().paint(">>>"));
        std::io::stdout().flush().unwrap();
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        input = input.trim().to_lowercase().to_string();

        if input == "exit" {
            std::process::exit(0);
        }

        if input.len() != 4 {
            clear_draw(board, is_white);
            input_error(Error::Length);
            continue;
        }

        let (from, to) = match_input(input);

        if from.0 == 99 || from.1 == 99 || to.0 == 99 || to.1 == 99 {
            clear_draw(board, is_white);
            input_error(Error::OutOfBounds);
            continue;
        }

        if board.tiles[from.0][from.1].piece.piece_type == Type::Empty {
            clear_draw(board, is_white);
            input_error(Error::Empty);
            continue;
        }

        if board.tiles[from.0][from.1].piece.colour != colour
            && board.tiles[from.0][from.1].piece.piece_type != Type::Empty
        {
            clear_draw(board, is_white);
            input_error(Error::EnemyMove);
            continue;
        }

        if board.tiles[to.0][to.1].piece.colour == colour
            && board.tiles[to.0][to.1].piece.piece_type != Type::Empty
        {
            clear_draw(board, is_white);
            input_error(Error::TeamDmg);
            continue;
        }

        return (from, to);
    }
}

fn match_input(input: String) -> ((usize, usize), (usize, usize)) {
    let mut chars = input.chars();
    let mut from = String::new();
    let mut to = String::new();

    for _ in 0..2 {
        from.push(chars.next().unwrap());
    }

    for _ in 0..2 {
        to.push(chars.next().unwrap());
    }

    let from: (usize, usize) = match from.as_str() {
        "a1" => (7, 0),
        "a2" => (6, 0),
        "a3" => (5, 0),
        "a4" => (4, 0),
        "a5" => (3, 0),
        "a6" => (2, 0),
        "a7" => (1, 0),
        "a8" => (0, 0),
        "b1" => (7, 1),
        "b2" => (6, 1),
        "b3" => (5, 1),
        "b4" => (4, 1),
        "b5" => (3, 1),
        "b6" => (2, 1),
        "b7" => (1, 1),
        "b8" => (0, 1),
        "c1" => (7, 2),
        "c2" => (6, 2),
        "c3" => (5, 2),
        "c4" => (4, 2),
        "c5" => (3, 2),
        "c6" => (2, 2),
        "c7" => (1, 2),
        "c8" => (0, 2),
        "d1" => (7, 3),
        "d2" => (6, 3),
        "d3" => (5, 3),
        "d4" => (4, 3),
        "d5" => (3, 3),
        "d6" => (2, 3),
        "d7" => (1, 3),
        "d8" => (0, 3),
        "e1" => (7, 4),
        "e2" => (6, 4),
        "e3" => (5, 4),
        "e4" => (4, 4),
        "e5" => (3, 4),
        "e6" => (2, 4),
        "e7" => (1, 4),
        "e8" => (0, 4),
        "f1" => (7, 5),
        "f2" => (6, 5),
        "f3" => (5, 5),
        "f4" => (4, 5),
        "f5" => (3, 5),
        "f6" => (2, 5),
        "f7" => (1, 5),
        "f8" => (0, 5),
        "g1" => (7, 6),
        "g2" => (6, 6),
        "g3" => (5, 6),
        "g4" => (4, 6),
        "g5" => (3, 6),
        "g6" => (2, 6),
        "g7" => (1, 6),
        "g8" => (0, 6),
        "h1" => (7, 7),
        "h2" => (6, 7),
        "h3" => (5, 7),
        "h4" => (4, 7),
        "h5" => (3, 7),
        "h6" => (2, 7),
        "h7" => (1, 7),
        "h8" => (0, 7),
        _ => (99, 99),
    };

    let to: (usize, usize) = match to.as_str() {
        "a1" => (7, 0),
        "a2" => (6, 0),
        "a3" => (5, 0),
        "a4" => (4, 0),
        "a5" => (3, 0),
        "a6" => (2, 0),
        "a7" => (1, 0),
        "a8" => (0, 0),
        "b1" => (7, 1),
        "b2" => (6, 1),
        "b3" => (5, 1),
        "b4" => (4, 1),
        "b5" => (3, 1),
        "b6" => (2, 1),
        "b7" => (1, 1),
        "b8" => (0, 1),
        "c1" => (7, 2),
        "c2" => (6, 2),
        "c3" => (5, 2),
        "c4" => (4, 2),
        "c5" => (3, 2),
        "c6" => (2, 2),
        "c7" => (1, 2),
        "c8" => (0, 2),
        "d1" => (7, 3),
        "d2" => (6, 3),
        "d3" => (5, 3),
        "d4" => (4, 3),
        "d5" => (3, 3),
        "d6" => (2, 3),
        "d7" => (1, 3),
        "d8" => (0, 3),
        "e1" => (7, 4),
        "e2" => (6, 4),
        "e3" => (5, 4),
        "e4" => (4, 4),
        "e5" => (3, 4),
        "e6" => (2, 4),
        "e7" => (1, 4),
        "e8" => (0, 4),
        "f1" => (7, 5),
        "f2" => (6, 5),
        "f3" => (5, 5),
        "f4" => (4, 5),
        "f5" => (3, 5),
        "f6" => (2, 5),
        "f7" => (1, 5),
        "f8" => (0, 5),
        "g1" => (7, 6),
        "g2" => (6, 6),
        "g3" => (5, 6),
        "g4" => (4, 6),
        "g5" => (3, 6),
        "g6" => (2, 6),
        "g7" => (1, 6),
        "g8" => (0, 6),
        "h1" => (7, 7),
        "h2" => (6, 7),
        "h3" => (5, 7),
        "h4" => (4, 7),
        "h5" => (3, 7),
        "h6" => (2, 7),
        "h7" => (1, 7),
        "h8" => (0, 7),
        _ => (99, 99),
    };

    return (from, to);
}

pub fn reverse_match_input(input: (usize, usize)) -> String {
    match input {
        (0, 0) => return String::from("a8"),
        (0, 1) => return String::from("b8"),
        (0, 2) => return String::from("c8"),
        (0, 3) => return String::from("d8"),
        (0, 4) => return String::from("e8"),
        (0, 5) => return String::from("f8"),
        (0, 6) => return String::from("g8"),
        (0, 7) => return String::from("h8"),
        (1, 0) => return String::from("a7"),
        (1, 1) => return String::from("b7"),
        (1, 2) => return String::from("c7"),
        (1, 3) => return String::from("d7"),
        (1, 4) => return String::from("e7"),
        (1, 5) => return String::from("f7"),
        (1, 6) => return String::from("g7"),
        (1, 7) => return String::from("h7"),
        (2, 0) => return String::from("a6"),
        (2, 1) => return String::from("b6"),
        (2, 2) => return String::from("c6"),
        (2, 3) => return String::from("d6"),
        (2, 4) => return String::from("e6"),
        (2, 5) => return String::from("f6"),
        (2, 6) => return String::from("g6"),
        (2, 7) => return String::from("h6"),
        (3, 0) => return String::from("a5"),
        (3, 1) => return String::from("b5"),
        (3, 2) => return String::from("c5"),
        (3, 3) => return String::from("d5"),
        (3, 4) => return String::from("e5"),
        (3, 5) => return String::from("f5"),
        (3, 6) => return String::from("g5"),
        (3, 7) => return String::from("h5"),
        (4, 0) => return String::from("a4"),
        (4, 1) => return String::from("b4"),
        (4, 2) => return String::from("c4"),
        (4, 3) => return String::from("d4"),
        (4, 4) => return String::from("e4"),
        (4, 5) => return String::from("f4"),
        (4, 6) => return String::from("g4"),
        (4, 7) => return String::from("h4"),
        (5, 0) => return String::from("a3"),
        (5, 1) => return String::from("b3"),
        (5, 2) => return String::from("c3"),
        (5, 3) => return String::from("d3"),
        (5, 4) => return String::from("e3"),
        (5, 5) => return String::from("f3"),
        (5, 6) => return String::from("g3"),
        (5, 7) => return String::from("h3"),
        (6, 0) => return String::from("a2"),
        (6, 1) => return String::from("b2"),
        (6, 2) => return String::from("c2"),
        (6, 3) => return String::from("d2"),
        (6, 4) => return String::from("e2"),
        (6, 5) => return String::from("f2"),
        (6, 6) => return String::from("g2"),
        (6, 7) => return String::from("h2"),
        (7, 0) => return String::from("a1"),
        (7, 1) => return String::from("b1"),
        (7, 2) => return String::from("c1"),
        (7, 3) => return String::from("d1"),
        (7, 4) => return String::from("e1"),
        (7, 5) => return String::from("f1"),
        (7, 6) => return String::from("g1"),
        (7, 7) => return String::from("h1"),
        _ => return String::from("ERR"),
    }
}

pub fn input_error(error: Error) {
    match error {
        Error::Empty => println!(
            "{} {}",
            Red.bold().paint(">>>"),
            "You can't move an empty tile!"
        ),
        Error::Length => println!(
            "{} {}",
            Red.bold().paint(">>>"),
            "Your input needs to be 4 chars long!"
        ),
        Error::IllegalMove => println!("{} {}", Red.bold().paint(">>>"), "Illegal move!"),
        Error::OutOfBounds => println!("{} {}", Red.bold().paint(">>>"), "Invalid choice!"),
        Error::EnemyMove => println!(
            "{} {}",
            Red.bold().paint(">>>"),
            "You can't move your opponent's piece!"
        ),
        Error::TeamDmg => println!(
            "{} {}",
            Red.bold().paint(">>>"),
            "You cannot attack your own piece!"
        ),

        Error::Check => println!(
            "{} {}",
            Red.bold().paint(">>>"),
            "You cannot move into check!"
        ),
    }
}

fn clear_draw(board: Board, is_white: bool) {
    clear_screen();
    board.draw_board(is_white);
}

fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
}

fn arrow_print(text: &str, bold: bool) {
    if bold {
        println!(
            "{} {}",
            RGB(80, 80, 80).bold().paint(">>>"),
            White.bold().paint(text)
        );
    } else {
        println!("{} {}", RGB(80, 80, 80).paint(">>>"), text);
    }
}
