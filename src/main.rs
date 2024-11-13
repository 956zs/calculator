use std::io;

fn main() {
    loop {
        println!("選擇運算符 (+, -, *, /) 或輸入 'q' 退出:");

        // 讀取運算符
        let mut operator = String::new();
        io::stdin().read_line(&mut operator).expect("讀取失敗");
        let operator = operator.trim();

        // 如果輸入 'q'，退出程序
        if operator == "q" {
            println!("退出計算機.");
            break;
        }

        // 讀取兩個浮點數
        let mut num1 = String::new();
        let mut num2 = String::new();

        println!("輸入第一個數字:");
        io::stdin().read_line(&mut num1).expect("讀取失敗");
        let num1: f32 = match num1.trim().parse() {
            Ok(n) => n,
            Err(_) => {
                println!("無效的數字，請重新輸入!");
                continue;
            }
        };

        println!("輸入第二個數字:");
        io::stdin().read_line(&mut num2).expect("讀取失敗");
        let num2: f32 = match num2.trim().parse() {
            Ok(n) => n,
            Err(_) => {
                println!("無效的數字，請重新輸入!");
                continue;
            }
        };

        // 根據運算符進行計算
        let result = match operator {
            "+" => num1 + num2,
            "-" => num1 - num2,
            "*" => num1 * num2,
            "/" => {
                if num2 == 0.0 {
                    println!("錯誤: 除數不能為零!");
                    continue;
                } else {
                    num1 / num2
                }
            }
            _ => {
                println!("無效的運算符，請重新輸入!");
                continue;
            }
        };

        // 輸出結果
        println!("結果: {}\n\n\n\n", result);
    }
}
