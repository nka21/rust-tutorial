// 標準入出力を行うための io ライブラリ
use std::io;
// 乱数を生成するための Rng ライブラリ
use rand::Rng;

fn main() {
    println!("Guess the number!");

    /*
        gen_range(1..101) // 下限値は含むが、上限値は含まない
     */

    // 乱数を、 1 ~ 100 までの間で生成する
    let secret_number = rand::thread_rng().gen_range(1..101);

    // 開発用のために、回答の数字を出力させる
    println!("The secret number is: {}", secret_number);

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