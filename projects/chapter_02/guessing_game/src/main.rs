use rand::Rng; // 乱数を生成するための Rng ライブラリ
use std::cmp::Ordering; //
use std::io; // 標準入出力を行うための io ライブラリ


fn main() {
    println!("Guess the number!");

    /*
        gen_range(1..101) // 下限値は含むが、上限値は含まない
     */

    // 乱数を、 1 ~ 100 までの間で生成する
    let secret_number = rand::thread_rng().gen_range(1..101);

    // 開発用のために、回答の数字を出力させる
    println!("The secret number is: {}", secret_number);

    // 無限ループ
    loop {
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
            前後の空白を削除
            入力直後のEnterキーによって、
            guessに改行文字(\n)が入ってしまうため
            trim()

            文字列をパース(解析)して、数値型にする
            parse()

            guessの後にコロンを付けることで、
            変数の型に型注釈をすることができる
            guess:

            parseメソッドは、
            論理的に数値に変換できる文字にしか使えないため、
            よくエラーになることがあるため記述
            変換できない文字列: (A👍%)
            .expect()
        */

        // string型からu32型へ型変換
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(e) => {
                println!("Error parsing guess: {}", e);
                continue
            },
        };

        /*
            let x = 5;
            let y = 10;

            println!("x = {} and y = {}", x, y); // x = 5 and y = 10
        */

        println!("You guessed: {}", guess); // {}に、第２引数のguessが入る

        // guess と secret_number の値の比較
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break; // 予想と回答が一致すれば終了
            }
        }
    }
}