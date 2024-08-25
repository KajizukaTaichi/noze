use std::{
    collections::HashMap,
    env::args,
    io::{self, Write},
    process::exit,
};

fn main() {
    println!("Noze：日本語プログラミング言語なのぜ！！！ \n(c) 2024 梶塚太智. All rights reserved");
    let args: Vec<String> = args().collect();
    let wordend: String = if args.len() > 1 {
        args[1].clone()
    } else {
        "のぜ".to_string()
    };

    let mut memory = HashMap::new();
    loop {
        let mut code = String::new();
        loop {
            let enter = input("> ");
            code += &format!("{enter}\n");
            if enter.is_empty() {
                break;
            }
        }
        noze(code, &mut memory, wordend.clone())
    }
}

fn input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut result = String::new();
    io::stdin().read_line(&mut result).ok();
    result.trim().to_string()
}

fn split_multiple(text: String, key: Vec<char>) -> Vec<String> {
    let mut result = Vec::new();
    let mut buffer = String::new();
    let mut flag = false;

    for c in text.chars() {
        if !flag {
            if key.contains(&c) {
                flag = true;
            } else {
                buffer.push(c);
            }
        } else {
            if !key.contains(&c) {
                result.push(buffer);
                flag = false;
                buffer = "".to_string();
            }
        }
    }
    if !buffer.is_empty() {
        result.push(buffer)
    }
    result
}

fn noze(source: String, memory: &mut HashMap<String, f64>, wordend: String) {
    for code in split_multiple(source, ['。', '！'].to_vec()) {
        let code = code.trim();
        if !code.is_empty() {
            if code.ends_with(&wordend) {
                let code = code.replace(&wordend, "");
                if code.contains("は") {
                    let code: Vec<&str> = code.split("は").collect();
                    let result = eval(code[1].to_string(), memory, wordend.clone());
                    memory.insert(code[0].to_string(), result);
                } else {
                    eval(code.to_string(), memory, wordend.clone());
                }
            } else {
                panic!("文の終端には「{}」を付ける必要がある{}", wordend, wordend);
            }
        }
    }
}

fn eval(code: String, memory: &mut HashMap<String, f64>, wordend: String) -> f64 {
    let code: Vec<&str> = code.split("を").collect();
    if code.len() > 1 {
        let (order, args): (String, Vec<f64>) = (
            code[1].replace("する", "").to_string(),
            code[0]
                .split("と")
                .into_iter()
                .map(|s| {
                    let s = s.trim();
                    if let Some(value) = memory.get(s) {
                        *value
                    } else {
                        s.parse::<f64>().unwrap_or_default()
                    }
                })
                .collect(),
        );

        match order.as_str() {
            "足し算" => {
                let mut result: f64 = *args.get(0).expect(&format!("引数が必要{}", wordend));
                for i in args[1..args.len()].to_vec().iter() {
                    result += i;
                }
                result
            }
            "引き算" => {
                let mut result: f64 = *args.get(0).expect(&format!("引数が必要{}", wordend));
                for i in args[1..args.len()].to_vec().iter() {
                    result -= i;
                }
                result
            }
            "掛け算" => {
                let mut result: f64 = *args.get(0).expect(&format!("引数が必要{}", wordend));
                for i in args[1..args.len()].to_vec().iter() {
                    result *= i;
                }
                result
            }
            "割り算" => {
                let mut result: f64 = *args.get(0).expect(&format!("引数が必要{}", wordend));
                for i in args[1..args.len()].to_vec().iter() {
                    result /= i;
                }
                result
            }
            "表示" => {
                println!(
                    "[出力]: {}",
                    args.iter()
                        .map(|i| i.to_string())
                        .collect::<Vec<String>>()
                        .join(" ")
                );
                0.0
            }
            other => panic!("定義されてない命令なのぜ：{}", other),
        }
    } else {
        match code[0].replace("する", "").as_str() {
            "入力待ち" => input("[入力]: ").parse::<f64>().unwrap_or_default(),
            "終了" => exit(0),
            other => panic!("定義されてない命令なのぜ：{}", other),
        }
    }
}
