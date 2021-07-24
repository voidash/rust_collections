//this is tic tac toe game implemented in rust
use std::io;
use input_stream::InputStream; 

struct Board {
   set :[[char; 3]; 3],
   variant_a : char,
   variant_b: char,
   turn: char
       
}

    fn check_straight(board:&Board) -> bool{
        for x in 0..3{
            //vertical
            let (a,b,c) = (board.set[0][x], board.set[1][x], board.set[2][x]);
            if a == b && b == c && a != '-'{
                return true;
             }
            //horizonal
            let (r,p,l) = (board.set[x][0], board.set[x][1], board.set[x][2]);
            if r == p && p == l && p != '-'{
                return true;
            }
       }
        false
    }


    fn check_diagonal(board: &Board) -> bool {
     let (r,p,l) = (board.set[0][0], board.set[1][1], board.set[2][2]);
            if r == p && p == l && p != '-'{
                return true;
            }
        
     let (x,u,z) = (board.set[2][0], board.set[1][1], board.set[0][2]);
            if x == u && u == z && x != '-'{
                return true;
            }
          false  
        }


fn check_win(board: &Board) -> bool{

     return  if check_diagonal(&board) || check_straight(&board) {true} else {false}
}

fn main() {
    let mut board = Board {
        set : [['-'; 3]; 3],
        variant_a: '0',
        variant_b: 'X',
        turn: '0' 
};
    println!("TIC TAC TOE ! Welcome to the game");
    println!("You  are {} and your opponent is {}",board.variant_a, board.variant_b);
    
    // board.set[0][0] = '0';
    // board.set[1][1] = '0';
    // board.set[2][2] = '0';


    loop {
         
        println!("its {} turn",board.turn);
        
    for x in board.set.iter() {
        for y in x.iter() {
            print!(" {} |",y);
        }
        println!("");
    }

    println!("Where do you want to place , in coordinates form");

    let stdin =io::stdin();
    let mut input = InputStream::new(stdin.lock());


    let u : usize= input.scan().expect("a char"); 
    let y : usize= input.scan().expect("a char");
    
    if board.set[u][y] != 'X' || board.set[u][y] != '0' {
        board.set[u][y] = board.turn;
    }else{
        println!("can't place there");
    }

        if check_win(&board) == true {
            println!("the game is won by {}",board.turn);
            break; 
        }

        board.turn = if board.turn == board.variant_a {board.variant_b} else {board.variant_a};

    }
}
