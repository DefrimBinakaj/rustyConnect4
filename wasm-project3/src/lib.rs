#[macro_use]
extern crate lazy_static;
extern crate mut_static;

mod utils;

use wasm_bindgen::prelude::*;
use mut_static::MutStatic;
use std::ffi::CString;
use std::os::raw::c_char;
use js_sys::Array;
use rand::prelude::*;
use rand::Rng;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

static C4_NUM_ROWS: usize = 6;
static C4_NUM_COLS: usize = 7;
static WIN_LEN_C4: usize = 4;
static TO_NUM_ROWS: usize = 4;
static TO_NUM_COLS: usize = 6;

lazy_static! {
    static ref BOARD: MutStatic<Board> = MutStatic::new();
    static ref GAME: MutStatic<String> = MutStatic::new();
}

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[derive(Debug, Clone)]
pub struct Player {
    id: usize,
}

#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct Space {
    row: usize,
    col: usize,
    player: Option<Player>,
    letter: Option<String>
}

impl Space {
    pub fn new(row: usize, col:usize, player: Option<Player>, letter: Option<String>) -> Space {
        Space{row: row, col:col, player:player, letter:letter}
    }
}

#[derive(Debug)]
pub struct Board {
    pieces: Vec<Vec<Space>>, // Find R1C3, pieces[2][0]
    num_rows: usize,
    num_cols: usize
}

impl Board {
    pub fn new() -> Board {
        let game = GAME.read().unwrap();
        let num_rows; let num_cols;
        if game.as_str() == "Connect4" {
            num_rows = C4_NUM_ROWS;
            num_cols = C4_NUM_COLS;
        } else if game.as_str() == "Toot-Otto" {
            num_rows = TO_NUM_ROWS;
            num_cols = TO_NUM_COLS;
        } else {
            panic!("Game not defined before Board created!");
        }
        // pieces = Vec of columns, where columns = Vec of rows
        let mut pieces = vec![];
        for i in 0..num_cols {
            let mut col = vec![];
            for j in 0..num_rows {
                col.push(Space::new(j + 1, i + 1, None, None));
            }
            pieces.push(col);
        }
        Board{pieces: pieces, num_rows, num_cols}
    }

    // get the Space at given row and column
    pub fn get_piece(&self, row: usize, col: usize) -> Space {
        self.pieces[col - 1][row - 1].clone()
    }
    // set the player at the given row and column (for Connect4)
    pub fn set_piece_player(&mut self, row: usize, col: usize, player: Player) {
        self.pieces[col - 1][row - 1].player = Some(player);
    }
    // set the letter at the given row and column (for Toot-Otto)
    pub fn set_piece_letter(&mut self, row: usize, col: usize, letter: String) {
        self.pieces[col - 1][row - 1].letter = Some(letter);
    }
    // return vector of player ids in given row
    pub fn get_ids_in_row(&self, row: usize) -> Vec<usize> {
        let mut row_vec = vec![];
        for col in 1..=self.num_cols {
            match self.pieces[col - 1][row - 1].player.clone() {
                None => row_vec.push(0),
                Some(player) => row_vec.push(player.id),
            }
        }
        return row_vec;
    }
    // return vector of player ids in given col
    pub fn get_ids_in_col(&self, col: usize) -> Vec<usize> {
        let mut col_vec = vec![];
        for row in 1..=self.num_rows {
            match self.pieces[col - 1][row - 1].player.clone() {
                None => col_vec.push(0),
                Some(player) => col_vec.push(player.id),
            }
        }
        return col_vec;
    }
    // return vector of player ids in right (/) diagonal from given row and col
    pub fn get_ids_in_right_diagonal(&self, mut row: usize, mut col: usize) -> Vec<usize> {
        // In right diagonal, up/right means R dec, C inc and down/left means R inc, C dec
        // First, go to bottom left corner of diagonal, where row = 6 or col = 1
        while row < self.num_rows && col > 1 { row += 1; col -= 1; }
        // Now at leftmost of diagonal, iterate to rightmost of diagonal
        let mut dia_vec = vec![];
        while row >= 1 && col <= self.num_cols {
            match self.pieces[col - 1][row - 1].player.clone() {
                None => dia_vec.push(0),
                Some(player) => dia_vec.push(player.id),
            }
            row -= 1; col += 1;
        }
        return dia_vec;
    }
    // return vector of player ids in left (\) diagonal from given row and col
    pub fn get_ids_in_left_diagonal(&self, mut row: usize, mut col: usize) -> Vec<usize> {
        // In left diagonal, up/left means R dec, C dec and down/right means R inc, C inc
        // First, go to top left corner of diagonal, where row = 1 or col = 1
        while row < 1 && col > 1 { row -= 1; col -= 1; }
        // Now at leftmost of diagonal, iterate to rightmost of diagonal
        let mut dia_vec = vec![];
        while row <= self.num_rows && col <= self.num_cols {
            match self.pieces[col - 1][row - 1].player.clone() {
                None => dia_vec.push(0),
                Some(player) => dia_vec.push(player.id),
            }
            row += 1; col += 1;
        }
        return dia_vec;
    }
    // return string of letters in given row
    pub fn get_letters_in_row(&self, row: usize) -> String {
        let mut row_str = String::new();
        for col in 1..=self.num_cols {
            match self.pieces[col - 1][row - 1].letter.clone() {
                None => row_str.push_str("-"),  // for blank spaces
                Some(letter) => row_str.push_str(&letter),
            }
        }
        return row_str;
    }
    // return string of letters in given col
    pub fn get_letters_in_col(&self, col: usize) -> String {
        let mut col_str = String::new();
        for row in 1..=self.num_rows {
            match self.pieces[col - 1][row - 1].letter.clone() {
                None => col_str.push_str("-"),  // for blank spaces
                Some(letter) => col_str.push_str(&letter),
            }
        }
        return col_str;
    }
    // return string of letters in right (/) diagonal from given row and col
    pub fn get_letters_in_right_diagonal(&self, mut row: usize, mut col: usize) -> String {
        // In right diagonal, up/right means R dec, C inc and down/left means R inc, C dec
        // First, go to bottom left corner of diagonal, where row = 6 or col = 1
        while row < self.num_rows && col > 1 { row += 1; col -= 1; }
        // Now at leftmost of diagonal, iterate to rightmost of diagonal
        let mut dia_str = String::new();
        while row >= 1 && col <= self.num_cols {
            match self.pieces[col - 1][row - 1].letter.clone() {
                None => dia_str.push_str("-"),  // for blank spaces
                Some(letter) => dia_str.push_str(&letter),
            }
            row -= 1; col += 1;
        }
        return dia_str;
    }
    // return string of letters in left (\) diagonal from given row and col
    pub fn get_letters_in_left_diagonal(&self, mut row: usize, mut col: usize) -> String {
        // In left diagonal, up/left means R dec, C dec and down/right means R inc, C inc
        // First, go to top left corner of diagonal, where row = 1 or col = 1
        while row < 1 && col > 1 { row -= 1; col -= 1; }
        // Now at leftmost of diagonal, iterate to rightmost of diagonal
        let mut dia_str = String::new();
        while row <= self.num_rows && col <= self.num_cols {
            match self.pieces[col - 1][row - 1].letter.clone() {
                None => dia_str.push_str("-"),  // for blank spaces
                Some(letter) => dia_str.push_str(&letter),
            }
            row += 1; col += 1;
        }
        return dia_str;
    }
}

#[wasm_bindgen]
// alert the given message
pub fn notify(msg: String) {
    alert(msg.as_str());
}

#[wasm_bindgen]
// set the game mode to "Toot-Otto" or "Connect4"
pub fn set_game(game: String) {
    GAME.set(game).unwrap();
}

#[wasm_bindgen]
// create a new board, must set the game mode first
pub fn new_board() {
    BOARD.set(Board::new()).unwrap();
}

#[wasm_bindgen]
// For Connect4 game
// Insert a piece into the selected column. Returns the row if successful, or 0 if the col is full
pub fn insert_piece_C4(col: usize, player_id: usize) -> usize {
    let mut board = BOARD.write().unwrap();
    for row in (1..=board.num_rows).rev() { // starting from bottom
        match board.get_piece(row, col).player {
            Some(_) => {},
            None => {
                board.set_piece_player(row, col, Player{id: player_id});
                println!("Changed piece R{}C{} to {:?}", row, col, board.get_piece(row, col));
                return row
            },
        }
    }
    return 0
}

#[wasm_bindgen]
// For Toot-Otto game
// Insert a piece into the selected column. Returns the row if successful, or 0 if the col is full
pub fn insert_piece_TO(col: usize, letter: String) -> usize {
    let mut board = BOARD.write().unwrap();
    for row in (1..=board.num_rows).rev() { // starting from bottom
        match board.get_piece(row, col).letter {
            Some(_) => {},
            None => {
                board.set_piece_letter(row, col, letter);
                println!("Changed piece R{}C{} to {:?}", row, col, board.get_piece(row, col));
                return row
            },
        }
    }
    return 0
}

#[wasm_bindgen]
// For Connect4 game
// Check around a piece for a win, return true if the game has been won
pub fn check_for_win_C4(row: usize, col: usize, player_id: usize) -> bool {
    let board = BOARD.read().unwrap();

    // Check for 4 in a row horizontally
    let ids_row = board.get_ids_in_row(row);
    if ids_row.iter().filter(|&x| *x == player_id).count() >= WIN_LEN_C4 {
        let mut consec_cnt = 0;
        for x in ids_row {
            if x == player_id {
                consec_cnt += 1;
                if consec_cnt == WIN_LEN_C4 {
                    return true;  // the player has won
                }
            } else {
                consec_cnt = 0;
            }
        }
    }
    // Check for 4 in a row vertically
    let ids_col = board.get_ids_in_col(col);
    if ids_col.iter().filter(|&x| *x == player_id).count() >= WIN_LEN_C4 {
        let mut consec_cnt = 0;
        for x in ids_col {
            if x == player_id {
                consec_cnt += 1;
                if consec_cnt == WIN_LEN_C4 {
                    return true;  // the player has won
                }
            } else {
                consec_cnt = 0;
            }
        }
    }

    // Check for 4 in a row diagonally /
    let ids_r_dia = board.get_ids_in_right_diagonal(row, col);
    if ids_r_dia.iter().filter(|&x| *x == player_id).count() >= WIN_LEN_C4 {
        let mut consec_cnt = 0;
        for x in ids_r_dia {
            if x == player_id {
                consec_cnt += 1;
                if consec_cnt == WIN_LEN_C4 {
                    return true;  // the player has won
                }
            } else {
                consec_cnt = 0;
            }
        }
    }

    // Check for 4 in a row diagonally \
    let ids_l_dia = board.get_ids_in_left_diagonal(row, col);
    if ids_l_dia.iter().filter(|&x| *x == player_id).count() >= WIN_LEN_C4 {
        let mut consec_cnt = 0;
        for x in ids_l_dia {
            if x == player_id {
                consec_cnt += 1;
                if consec_cnt == WIN_LEN_C4 {
                    return true;  // the player has won
                }
            } else {
                consec_cnt = 0;
            }
        }
    }

    return false;
}

#[wasm_bindgen]
// For Toot-Otto game
// Check around a piece for a win, return 1 for TOOT, 2 for OTTO, 0 if no win, 3 if tie
pub fn check_for_win_TO(row: usize, col: usize) -> usize {
    let board = BOARD.read().unwrap();
    let mut winner = 0;

    // Check for win horizontally
    let letters_row = board.get_letters_in_row(row);
    if letters_row.chars().filter(|x| *x != '-').count() >= WIN_LEN_C4 {
        if letters_row.contains("TOOT") {
            winner = 1;
        } 
        if letters_row.contains("OTTO") {
            if winner != 1 {
                winner = 2;
            } else {
                return 3;  // there was a tie
            } 
        }
    }
    // Check for win vertically
    let letters_col = board.get_letters_in_col(col);
    if letters_col.chars().filter(|x| *x != '-').count() >= WIN_LEN_C4 {
        if letters_col.contains("TOOT") {
            if winner != 2 {
                winner = 1;
            } else {
                return 3;  // there was a tie
            }
        } else if letters_col.contains("OTTO") {
            if winner != 1 {
                winner = 2;
            } else {
                return 3;  // there was a tie
            }
        }
    }

    // Check for 4 in a row diagonally /
    let letters_r_dia = board.get_letters_in_right_diagonal(row, col);
    if letters_r_dia.chars().filter(|x| *x != '-').count() >= WIN_LEN_C4 {
        if letters_r_dia.contains("TOOT") {
            if winner != 2 {
                winner = 1;
            } else {
                return 3;  // there was a tie
            } 
        } else if letters_r_dia.contains("OTTO") {
            if winner != 1 {
                winner = 2;
            } else {
                return 3;  // there was a tie
            }
        }
    }

    // Check for 4 in a row diagonally \
    let letters_l_dia = board.get_letters_in_left_diagonal(row, col);
    if letters_l_dia.chars().filter(|x| *x != '-').count() >= WIN_LEN_C4 {
        if letters_l_dia.contains("TOOT") {
            if winner != 2 {
                winner = 1;
            } else {
                return 3;  // there was a tie
            } 
        } else if letters_l_dia.contains("OTTO") {
            if winner != 1 {
                winner = 2;
            } else {
                return 3;  // there was a tie
            } 
        }
    }

    return winner;
}


// leaderboards stuff

// leaderboards stuff

// #[wasm_bindgen]
// pub fn getUsername(username: &JsValue) -> JsValue {
    
//     let mut doc_name_t: String = username.as_string().unwrap();

//     let mut doc_name = doc_name_t.as_str();

//     if doc_name == "" {
//         return JsValue::from_str("[please enter a valid username]");
//     } else {
//         return JsValue::from_str(doc_name);
//     }
    
// }

// #[wasm_bindgen]
// pub fn getPassword(password: &JsValue) -> JsValue {
    
//     let mut doc_password_t: String = password.as_string().unwrap();

//     let mut doc_password = doc_password_t.as_str();

//     if doc_password == "" {
//         return JsValue::from_str("[please enter a valid password]");
//     } else {
//         return JsValue::from_str(doc_password);
//     }
    
// }



// https://stackoverflow.com/questions/47529643/how-to-return-a-string-or-similar-from-rust-in-webassembly#:~:text=You%20cannot%20directly%20return%20a,string%20on%20the%20JavaScript%20side.
#[wasm_bindgen]
pub fn showC4Leaderboard() -> JsValue {
    let mut c4List = "".to_owned();
    for i in 0..5 {
        let tempStr: String = ". [USER]\n".to_owned();
        c4List.push_str(&(i+1).to_string());
        c4List.push_str(&tempStr);
        // c4List += i as String;
    }
    let c4ListStr = c4List.as_str();

    return JsValue::from_str(c4ListStr);
}

#[wasm_bindgen]
//Easy bot for otto game
pub fn easy_otto()-> Array{
    loop{
        let mut rng = rand::thread_rng();
        let letter_choice = if rng.gen_bool(0.5) { 'T' } else { 'O' };
        let column = rng.gen_range(1..6);
        
       
        let insert_obj = insert_piece_TO(column, letter_choice.to_string());

         if insert_obj>0 && insert_obj>0 {
            let arr = Array::new();
    
        arr.push(&JsValue::from(insert_obj));
        arr.push(&JsValue::from(column));
        arr.push(&JsValue::from(letter_choice.to_string()));
        return arr;
        }

        let win = check_for_win_TO(insert_obj, column);
        if win == 2 {
            alert(format!("Player 2 has won!!!", ).as_str());

        }

    }
}

//This function evaluates the board
#[wasm_bindgen]
pub fn evaluate_TO(player_id: usize) -> Array{
    let board = BOARD.read().unwrap();
     let mut horizontal_score = 0;
     let mut vertical_score = 0;
     let mut diagonal_score = 0;
     let mut rev_diagonal_score = 0;
    
   

    for row in (1..TO_NUM_ROWS+1).rev() {
        for column in 1..TO_NUM_COLS+1{
            
            for i in 0..3{
              
                if column <= TO_NUM_COLS-3{
                   
                        match board.get_piece(row, column+i).player{
                            Some(player) => {
                             
                                if player.id == player_id{
                                horizontal_score += 1;
                              
                                break;
                                }
                            }            
                            None => {
                       
                                break; 
                            }  
                        }
                }
              

        
                //Count number of consecutive vertically
                if row >= TO_NUM_ROWS-3{    
                    
                    match board.get_piece(row-i, column).player{
                        Some(player) => {
                            if player.id == player_id{
                                vertical_score += 1;
                                break;
                    
                            }
                    }
                        None => {
                            
                            break; 
                        } 
   
                    }

                    }
            
                //Diagonal count
                if row >= TO_NUM_ROWS-2 && column <= TO_NUM_COLS-3{
                   
                    match board.get_piece(row-i, column+i).player{
                        Some(player) => {
                        
                            if player.id == player_id{
                            diagonal_score += 1;
                            break;
                        }
  
                            }
                        
                        None => {
                        
                            break; 
                        } 

                    }
                }
               
                //Reverse Diagonal
                if row >= 3 && column >= TO_NUM_COLS-3{
                  
                    match board.get_piece(row-i, column-i).player{
                        Some(player) => {

                        if player.id == player_id{
                        rev_diagonal_score += 1;
                        break;
                        }
                         
                    }
                    None => {
                        
                        break; 
                    } 
                    }

                }
            }
        }
        
    }
    let mut win = 0;
    if (horizontal_score == 4){
        win = horizontal_score;
    }
    else if (vertical_score == 4){
        win = vertical_score;
    }
    else if (diagonal_score == 4){
        win = diagonal_score;
    }
    else if(rev_diagonal_score == 4){
        win = rev_diagonal_score;
    }
    let total_score = horizontal_score+vertical_score+diagonal_score+rev_diagonal_score;
    let array = Array::new_with_length(2);
    
        array.push(&JsValue::from(win));
        array.push(&JsValue::from(total_score));
        let mut column = 0;
        let mut  insert_obj = 0;
        
         
    return array;
      
       
    
   
            
}

#[wasm_bindgen]

//Added by AB
pub fn difficult_TO(player_id: usize) -> Array{   
    loop{
        let mut rng = rand::thread_rng();
        let letter_choice = if rng.gen_bool(0.5) { 'T' } else { 'O' };
        let column = rng.gen_range(1..6);
        
        
        let insert_obj = insert_piece_TO(column, letter_choice.to_string());
       

         if insert_obj>0 && insert_obj>0 {
            let arr = Array::new();
    
            arr.push(&JsValue::from(insert_obj));
            arr.push(&JsValue::from(column));
            arr.push(&JsValue::from(letter_choice.to_string()));
            return arr;
        }

    }
}
#[wasm_bindgen]

pub fn value_TO( player_id: usize, depth: i32, alpha: f64, beta: f64) -> JsValue{
   
    let mut score = evaluate_TO(player_id);
    if depth>=4{
        let mut value = 0.0; 

        let mut win = score.get(0).as_f64().unwrap();
        let mut total_score = score.get(1).as_f64().unwrap();

        value = total_score;

       
       if win == 4.0 && player_id == 2 {
        value = 999999.0;
       }
       else if (win == 4.0 && player_id == 1){
        value = 999999.0*-1.0;
       }
       
       if depth %2 == 0 {
        return minState_TO(player_id,depth+1 , alpha, beta);
       }
       return maxState_TO(player_id, depth, alpha, beta);

    }

    else{
        return maxState_TO(player_id, depth, alpha, beta);
     
    
    }


}

#[wasm_bindgen]
pub fn choose_TO(choice: Vec<i32>) -> i32 {
    let mut index = rand::thread_rng().gen_range(0..choice.len());
    return choice[index]
}

#[wasm_bindgen]
pub fn maxState_TO(player_id: usize, depth: i32, alpha: f64, beta: f64) -> JsValue {
    let mut val = -100000000007.0;
    let mut play = -1;
    let mut tempValue = Array::new();
    let mut tempState = 0;
    let mut moves = Array::new();

    let mut new_board = BOARD.write().unwrap();

    for i in (0..6) {
        // Insert those pieces in the cloned board new_board.set_piece(row, col, player)
        if tempState > 0 {
           // tempValue = value(tempState, 2, depth, alpha, beta);
            let temp = tempValue.get(0).as_f64().unwrap();
            if temp > val {
                val = temp;
                play = i;
                moves.push(&JsValue::from(i));
            } else if temp == val {
                moves.push(&JsValue::from(i));
            }

            if val > beta {
                let vec = moves.to_vec();
                let js_values = vec![JsValue::from(1), JsValue::from(2), JsValue::from(3)];
                let i32_values: Vec<i32> =
                    js_values.iter().map(|value| value.as_f64().unwrap() as i32).collect();

                play = choose_TO(i32_values);
                let result = Array::new();
                result.push(&JsValue::from(val));
                result.push(&JsValue::from(play));
                return result.into();
            }
            let mut alpha = 0.0;
            if val > alpha {
                alpha = val;
            }
        }
    }
    let js_values = vec![JsValue::from(1), JsValue::from(2), JsValue::from(3)];
    let i32_values: Vec<i32> = js_values.iter().map(|value| value.as_f64().unwrap() as i32).collect();

    play = choose_TO(i32_values).into();
    let result = Array::new();
    result.push(&JsValue::from(val));
    result.push(&JsValue::from(play));
    return result.into();
}

#[wasm_bindgen]
pub fn minState_TO(player_id: usize, depth: i32, alpha: f64, beta: f64) -> JsValue{
    let mut val = -100000000007.0;
    let mut play = -1;
    let mut tempValue = Array::new();
    let mut tempState = 0.0;
    let mut moves = Array::new();

    for i in (0..7) {
        // Insert those pieces in the cloned board new_board.set_piece(row, col, player)
        if tempState > 0.0 {
          //  tempValue = value(tempState, 2, depth, alpha, beta);
            let temp = tempValue.get(0).as_f64().unwrap();
            if temp < val {
                val = temp;
                play = i;
                moves.push(&JsValue::from(i));
            } else if temp == val {
                moves.push(&JsValue::from(i));
            }
            

            if val < alpha {
                let vec = moves.to_vec();
                let js_values = vec![JsValue::from(1), JsValue::from(2), JsValue::from(3)];
                let i32_values: Vec<i32> =
                    js_values.iter().map(|value| value.as_f64().unwrap() as i32).collect();

                play = choose_TO(i32_values);
                let result = Array::new();
                result.push(&JsValue::from(val));
                result.push(&JsValue::from(play));
                return result.into();
            }
            let mut alpha = 0.0;
            if val > beta {
                alpha = val;
            }
        }
    }

    let js_values = vec![JsValue::from(1), JsValue::from(2), JsValue::from(3)];
    let i32_values: Vec<i32> = js_values.iter().map(|value| value.as_f64().unwrap() as i32).collect();

    play = choose_TO(i32_values).into();
    let result = Array::new();
    result.push(&JsValue::from(val));
    result.push(&JsValue::from(play));
    return result.into();
            



}

#[wasm_bindgen]

pub fn medium_TO(player_id: usize) -> Array{
    let mut column = 0;
    let mut  row = 0;
    let mut data = 0;
  
   // let board = BOARD.read().unwrap();

   loop{
    let mut rng = rand::thread_rng();
    let letter_choice = if rng.gen_bool(0.5) { 'T' } else { 'O' };
    let column = rng.gen_range(1..6);
    let insert_obj = insert_piece_TO(column, letter_choice.to_string());
   
     if insert_obj>0 && insert_obj>0 {
        let arr = Array::new();

        arr.push(&JsValue::from(insert_obj));
        arr.push(&JsValue::from(column));
        arr.push(&JsValue::from(letter_choice.to_string()));
        return arr;
    }

}
    
    
}


//---------------------------------------------------C4 GAME --------------------------------------

#[wasm_bindgen]

//Added by AB
pub fn easy_bot_C4(player_id: usize) -> Array{
    
    let mut column = 0;
    let mut  insert_obj = 0;
    
     loop{
        let mut rng = rand::thread_rng();
        let column = rng.gen_range(1..7);
      
        let insert_obj = insert_piece_C4(column, player_id);
      
         if insert_obj>0 && insert_obj>0 {
            let arr = Array::new();
    
            arr.push(&JsValue::from(insert_obj));
            arr.push(&JsValue::from(column));
            return arr;
         }

        let win = check_for_win_C4(insert_obj, column, player_id);
        if win == true {
            alert(format!("Player 2 has won!!!", ).as_str());

        }

    }
    
}

#[wasm_bindgen]

//Added by AB
pub fn evaluate_C4(player_id: usize) -> Array{
    let board = BOARD.read().unwrap();
    let mut horizontal_score = 0;
    let mut vertical_score = 0;
    let mut diagonal_score = 0;
    let mut rev_diagonal_score = 0;
    let arr = easy_bot_C4(2);
   

    for row in (1..C4_NUM_ROWS+1).rev() {
        for column in 1..C4_NUM_COLS+1{
            
            for i in 0..3{
               
                if column <= C4_NUM_COLS-3{
                   
                        match board.get_piece(row, column+i).player{
                            Some(player) => {
                             
                                if player.id == player_id{
                                horizontal_score += 1;
                              
                                break;
                                }
                            }            
                            None => {
                       
                                break; 
                            }  
                        }
                }
        
                //Count number of consecutive vertically
                if row >= C4_NUM_ROWS-3{    
                    //alert(format!("Reached vertical row and column are {row}{column}", ).as_str());
                    match board.get_piece(row-i, column).player{
                        Some(player) => {
                            if player.id == player_id{
                                vertical_score += 1;
                                break;
                    
                            }
                    }
                        None => {
                            
                            break; 
                        } 
   
                    }

                    }
            
                //Diagonal count
                if row >= C4_NUM_ROWS-2 && column <= C4_NUM_COLS-3{
                   // alert(format!("Reached diagonal row and column are {row}{column}", ).as_str());
                    match board.get_piece(row-i, column+i).player{
                        Some(player) => {
                        
                            if player.id == player_id{
                            diagonal_score += 1;
                            break;
                        }
  
                            }
                        
                        None => {
                        
                            break; 
                        } 

                    }
                }
               
                //Reverse Diagonal
                if row >= 3 && column >= C4_NUM_COLS-3{
                  //  alert(format!("Reached rev diagonal row and column are {row}{column}", ).as_str());
                    match board.get_piece(row-i, column-i).player{
                        Some(player) => {

                        if player.id == player_id{
                        rev_diagonal_score += 1;
                        break;
                        }
                         
                    }
                    None => {
                        
                        break; 
                    } 
                    }

                }
            }
        }
        
    }
    let mut win = 0;
    if (horizontal_score == 4){
        win = horizontal_score;
    }
    else if (vertical_score == 4){
        win = vertical_score;
    }
    else if (diagonal_score == 4){
        win = diagonal_score;
    }
    else if(rev_diagonal_score == 4){
        win = rev_diagonal_score;
    }
    let total_score = horizontal_score+vertical_score+diagonal_score+rev_diagonal_score;
    let array = Array::new_with_length(2);
    
        array.push(&JsValue::from(win));
        array.push(&JsValue::from(total_score));
        let mut column = 0;
        let mut  insert_obj = 0;
        
         
    return array;
       
       
    
   
            
}

#[wasm_bindgen]

//Added by AB
pub fn difficult_C4(player_id: usize) -> Array{   
  
   let mut column = 0;
   let mut  insert_obj = 0;
   
    loop{
       let mut rng = rand::thread_rng();
       let column = rng.gen_range(1..7);
     
       let insert_obj = insert_piece_C4(column, player_id);

        if insert_obj>0 && insert_obj>0 {
           let arr = Array::new();
   
       arr.push(&JsValue::from(insert_obj));
       arr.push(&JsValue::from(column));
       return arr;
        }

       let win = check_for_win_C4(insert_obj, column, player_id);
       if win == true {
           alert(format!("Player 2 has won!!!", ).as_str());

       }

   }
}
#[wasm_bindgen]

pub fn value( player_id: usize, depth: i32, alpha: f64, beta: f64) -> JsValue{
   
    let mut score = evaluate_C4(player_id);
    if depth>=4{
        let mut value = 0.0; 

        let mut win = score.get(0).as_f64().unwrap();
        let mut total_score = score.get(1).as_f64().unwrap();

        value = total_score;

       if win == 4.0 && player_id == 2 {
        value = 999999.0;
       }
       else if (win == 4.0 && player_id == 1){
        value = 999999.0*-1.0;
       }
       
       if depth %2 == 0 {
        return minState_C4(player_id,depth+1 , alpha, beta);
       }
       return maxState_C4(player_id, depth, alpha, beta);

    }

    else{
        return maxState_C4(player_id, depth, alpha, beta);
     
    
    }


}

#[wasm_bindgen]

pub fn choose_C4(choice: Vec<i32>) -> i32 {
    let mut index = rand::thread_rng().gen_range(0..choice.len());
    return choice[index]
}

#[wasm_bindgen]
pub fn maxState_C4(player_id: usize, depth: i32, alpha: f64, beta: f64) -> JsValue {
    let mut val = -100000000007.0;
    let mut play = -1;
    let mut tempValue = Array::new();
    let mut tempState = 0;
    let mut moves = Array::new();

    let mut new_board = BOARD.write().unwrap();

    for i in (0..7) {
        // Insert those pieces in the cloned board new_board.set_piece(row, col, player)
        if tempState > 0 {
           // tempValue = value(tempState, 2, depth, alpha, beta);
            let temp = tempValue.get(0).as_f64().unwrap();
            if temp > val {
                val = temp;
                play = i;
                moves.push(&JsValue::from(i));
            } else if temp == val {
                moves.push(&JsValue::from(i));
            }

            if val > beta {
                let vec = moves.to_vec();
                let js_values = vec![JsValue::from(1), JsValue::from(2), JsValue::from(3)];
                let i32_values: Vec<i32> =
                    js_values.iter().map(|value| value.as_f64().unwrap() as i32).collect();

                play = choose_C4(i32_values);
                let result = Array::new();
                result.push(&JsValue::from(val));
                result.push(&JsValue::from(play));
                return result.into();
            }
            let mut alpha = 0.0;
            if val > alpha {
                alpha = val;
            }
        }
    }
    let js_values = vec![JsValue::from(1), JsValue::from(2), JsValue::from(3)];
    let i32_values: Vec<i32> = js_values.iter().map(|value| value.as_f64().unwrap() as i32).collect();

    play = choose_C4(i32_values).into();
    let result = Array::new();
    result.push(&JsValue::from(val));
    result.push(&JsValue::from(play));
    return result.into();
}

#[wasm_bindgen]
pub fn minState_C4(player_id: usize, depth: i32, alpha: f64, beta: f64) -> JsValue{
    let mut val = -100000000007.0;
    let mut play = -1;
    let mut tempValue = Array::new();
    let mut tempState = 0.0;
    let mut moves = Array::new();

    for i in (0..7) {
        // Insert those pieces in the cloned board new_board.set_piece(row, col, player)
        if tempState > 0.0 {
          //  tempValue = value(tempState, 2, depth, alpha, beta);
            let temp = tempValue.get(0).as_f64().unwrap();
            if temp < val {
                val = temp;
                play = i;
                moves.push(&JsValue::from(i));
            } else if temp == val {
                moves.push(&JsValue::from(i));
            }
            

            if val < alpha {
                let vec = moves.to_vec();
                let js_values = vec![JsValue::from(1), JsValue::from(2), JsValue::from(3)];
                let i32_values: Vec<i32> =
                    js_values.iter().map(|value| value.as_f64().unwrap() as i32).collect();

                play = choose_C4(i32_values);
                let result = Array::new();
                result.push(&JsValue::from(val));
                result.push(&JsValue::from(play));
                return result.into();
            }
            let mut alpha = 0.0;
            if val > beta {
                alpha = val;
            }
        }
    }

    let js_values = vec![JsValue::from(1), JsValue::from(2), JsValue::from(3)];
    let i32_values: Vec<i32> = js_values.iter().map(|value| value.as_f64().unwrap() as i32).collect();

    play = choose_C4(i32_values).into();
    let result = Array::new();
    result.push(&JsValue::from(val));
    result.push(&JsValue::from(play));
    return result.into();
            



}

#[wasm_bindgen]

pub fn medium_C4(player_id: usize) -> Array{
    let mut column = 0;
    let mut  row = 0;
    let mut data = 0;
  
   // let board = BOARD.read().unwrap();

    loop{
        let mut rng = rand::thread_rng();
        let column = rng.gen_range(1..7);
        
        let insert_obj = insert_piece_C4(column, player_id);
       

         if insert_obj>0 && insert_obj>0 {
            let arr = Array::new();
    
        arr.push(&JsValue::from(insert_obj));
        arr.push(&JsValue::from(column));
        return arr;
         }

        let win = check_for_win_C4(insert_obj, column, player_id);
        if win == true {
            alert(format!("Player 2 has won!!!", ).as_str());

        }

    }
    
    
}





  
