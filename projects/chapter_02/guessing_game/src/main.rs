// 標準入出力を行うための io というライブラリ
use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    /*
        let apples = 5; // 不変
        let mut bananas = 5; // 可変
    */

    let mut guess = String::new(); // String型の新しいインスタンスを作成


    io::stdin()
        .read_line(&mut guess) // 入力をguessに格納
        .expect("Failed to read line"); // 入力処理に対するエラー処理

    /*
        let x = 5;
        let y = 10;

        println!("x = {} and y = {}", x, y); // x = 5 and y = 10
    */

    println!("You guessed: {}", guess); // {}に、第２引数のguessが入る
}