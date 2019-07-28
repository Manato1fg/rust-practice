use std::io;
use rand::Rng;

fn main() {
    let to_str : [String; 3] = [u("グー"), u("チョキ"), u("パー")];
    println!("じゃんけんゲーム\n連続何回勝利できるか競います");

    let mut times = 0;
    loop {
        let mut s = String::new();
        println!("0, 1, 2のうちのいずれかを入力してください");
        io::stdin().read_line(&mut s).expect("文字列を読み取れませんでした");
        let num : usize = match s.trim().parse::<usize>() {
            Ok(x) => {
                if x > 2 {
                    println!("0, 1, 2のうちのいずれかを入力してください");
                    continue;
                } else {
                    x
                }
            },
            Err(_) => {
                println!("0, 1, 2のうちのいずれかを入力してください");
                continue;
            },
        };
        println!("あなたの出した手: {}", to_str[num]);
        let rand = rand::thread_rng().gen_range(0, 2);
        println!("コンピュータの出した手: {}", to_str[rand]);

        if rand == num {
            println!("あいこでした。もう一度！");
            continue;
        } else if (num + 1) % 3 == rand {
            println!("勝ちました！");
            times += 1;
            continue;
        } else {
            println!("負けてしまいました...\n連続勝利回数{}回", times);
            break;
        }
    }
}

fn u(s: &str) -> String{
    return match String::from_utf8(s.as_bytes().to_vec()) {
        Ok(x) => x,
        Err(_) => {
            println!("An error occured");
            return String::new();
        }
    };
}
