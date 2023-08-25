use std::io;

fn Win(player: &str){
    println!("Player {} won!", player);
} 

fn main()
{
    let mut playing = true;
    
    println!("Initializing the program...");

    let array_width = 3;
    let array_height = 3;

    let mut played_width = 0;
    let mut played_height = 0;
    let mut input = String::new();
    
    let mut array = vec![vec![0; array_width]; array_height];

    //            0   1   2
    //          0 0 , 0 , 0
    //          1 0 , 0 , 0
    //          2 0 , 0 , 0

    //          0 means empty
    //          1 means player 1
    //          2 means player 2

    while playing == true {
        // player 1 moves first

        for i in 0..array_width{
            println!("{:?}", array[i]);
        }
        while played_width > 3 || played_width < 1 {
            println!("Player 1: Please input the row of your play: ");
            input.clear();
            io::stdin().read_line(&mut input).expect("Failed to read!");
            played_width = input.trim().parse().expect("Not a valid number.");
        }
        while played_height > 3 || played_height < 1 { 
            println!("Player 1: Please input the column of your play: ");
            input.clear();
            io::stdin().read_line(&mut input).expect("Failed to read!");
            played_height = input.trim().parse().expect("Not a valid number.");
        }
        array[played_width-1][played_height-1] = 1;

        if array[0][0] == 1 && array[1][1] == 1 && array[2][2] == 1 {Win("player1"); break;}
        else if array[0][0] == 1 && array[0][1] == 1 && array[0][2] == 1 {Win("player1"); break;}
        else if array[1][1] == 1 && array[1][1] == 1 && array[1][2] == 1 {Win("player1"); break;}
        else if array[2][0] == 1 && array[2][1] == 1 && array[2][2] == 1 {Win("player1"); break;}
        else if array[2][0] == 1 && array[1][1] == 1 && array[0][2] == 1 {Win("player1"); break;}
        else if array[0][0] == 1 && array[1][0] == 1 && array[2][0] == 1 {Win("player1"); break;}
        else if array[0][1] == 1 && array[1][1] == 1 && array[2][1] == 1 {Win("player1"); break;}
        else if array[0][2] == 1 && array[1][2] == 1 && array[2][2] == 1 {Win("player1"); break;}

        played_width = 0;
        played_height = 0;

        //player 2 goes second

        for i in 0..array_width{
            println!("{:?}", array[i]);
        }
        while played_width > 3 || played_width < 1 {
            println!("Player 2: Please input the row of your play: ");
            input.clear();
            io::stdin().read_line(&mut input).expect("Failed to read!");
            played_width = input.trim().parse().expect("Not a valid number.");
        }
        while played_height > 3 || played_height < 1 {
            println!("Player 2: Please input the column of your play: ");
            input.clear();
            io::stdin().read_line(&mut input).expect("Failed to read!");
            played_height = input.trim().parse().expect("Not a valid number.");
        }
        array[played_width-1][played_height-1] = 2;

        if array[0][0] == 2 && array[1][1] == 2 && array[2][2] == 2 {Win("player2"); break;}
        else if array[0][0] == 2 && array[0][1] == 2 && array[0][2] == 2 {Win("player2"); break;}
        else if array[1][0] == 2 && array[1][1] == 2 && array[1][2] == 2 {Win("player2"); break;}
        else if array[2][0] == 2 && array[2][1] == 2 && array[2][2] == 2 {Win("player2"); break;}
        else if array[2][0] == 2 && array[1][1] == 2 && array[0][2] == 2 {Win("player2"); break;}
        else if array[0][0] == 2 && array[1][0] == 2 && array[2][0] == 2 {Win("player2"); break;}
        else if array[0][1] == 2 && array[1][1] == 2 && array[2][1] == 2 {Win("player2"); break;}
        else if array[0][2] == 2 && array[1][2] == 2 && array[2][2] == 2 {Win("player2"); break;}

        played_width = 0;
        played_height = 0;

    }
}