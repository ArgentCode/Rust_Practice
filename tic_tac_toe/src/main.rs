fn main() {
    println!("Hello, world!");
    println!("");
    println!("");
    println!("Please enter the number that you would like place your marker on. X goes first!");
    let mut board: [[&str; 3]; 3] = [["0","1","2"],["3","4","5"],["6","7","8"]];
    let mut options: Vec<String> = vec!["0".to_string(),"1".to_string(),"2".to_string(),
                                    "3".to_string(),"4".to_string(),"5".to_string(),
                                    "6".to_string(),"7".to_string(),"8".to_string()];
    print_board(board);
    let mut player = true;
    loop {
        let winner: &str = check_win(board);
        if winner == "N" {
            let mut line = String::new();
            let _ = std::io::stdin().read_line(&mut line);
            if !options.contains(&line.trim().to_string()) {
                println!("Please choose one of the options on the board");
                continue;
            };
            options.retain(|x| x != line.trim());
            let input: i32 = line.trim().parse::<i32>().unwrap();
            let col: usize = (input%3).try_into().unwrap();
            let row: usize = (input/3).try_into().unwrap();
            if player {
                board[row][col] = "X";
            } else {
                board[row][col] = "O";
            };
            player = !player;
            println!("");
            print_board(board);
        } else {
            println!("{} wins! ", winner);
            break
        };
    }

}

fn print_board(board: [[&str; 3]; 3]) {
    for item in board {
        println!("{} | {} | {} ", item[0].to_string(), item[1].to_string(), item[2].to_string());
        }
}

fn check_win(board: [[&str; 3]; 3]) -> &str {
    let s: &str = "N";
    for item in board {
        if item[0] == item[1] && item[1] == item[2] {
            return item[0]
        }
    };
    if board[0][0] == board[1][1] && board[1][1] == board[2][2] {
        return board[1][1]
    };
    if board[0][2] == board[1][1] && board[1][1] == board[2][0] {
        return board[1][1]
    };
    let mut i = 0;
    while i < 3 {
        if board[0][i] == board[1][i] && board[1][i] == board[2][i] {
            return board[0][i];
        }
        i = i+1;
    }

    return s;
}