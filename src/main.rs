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
            let enter = input("> ").trim().to_string();
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

#[derive(Clone)]
enum Type {
    Number(f64),
    String(String),
}

impl Type {
    fn get_string(&self) -> String {
        match self {
            Type::Number(i) => i.to_string(),
            Type::String(s) => s.to_string(),
        }
    }
    fn get_number(&self) -> f64 {
        match self {
            Type::Number(i) => *i,
            Type::String(s) => s.parse().unwrap_or_default(),
        }
    }
}

fn noze(source: String, memory: &mut HashMap<String, Type>, wordend: String) {
    let lines = split_multiple(source, ['。', '！'].to_vec());
    let mut pc:  usize = 0;
    while pc > lines.len() {
        let code = lines[pc].trim();
        if !code.is_empty() {
            if code.ends_with(&wordend) {
                let code = code.replace(&wordend, "");
                if code.ends_with("する") {
                    let code = code.replace("する", "");
                    let mut eval = |code: String| -> Type {
                        let code: Vec<&str> = code.split("を").collect();
                        if code.len() > 1 {
                            let (order, args): (String, Vec<Type>) = (
                                code[1].to_string(),
                                code[0]
                                    .split("と")
                                    .into_iter()
                                    .map(|s| {
                                        let s = s.trim();
                                        if let Some(value) = memory.get(s) {
                                            value.clone()
                                        } else if let Ok(i) = s.parse::<f64>() {
                                            Type::Number(i)
                                        } else {
                                            Type::String(s.to_string())
                                        }
                                    })
                                    .collect(),
                            );

                            match order.as_str() {
                                "足し算" => {
                                    let args: Vec<f64> =
                                        args.iter().map(|i| i.get_number()).collect();
                                    let mut result: f64 =
                                        *args.get(0).expect(&format!("引数が必要{}", wordend));
                                    for i in args[1..args.len()].to_vec().iter() {
                                        result += i;
                                    }
                                    Type::Number(result)
                                }
                                "引き算" => {
                                    let args: Vec<f64> =
                                        args.iter().map(|i| i.get_number()).collect();
                                    let mut result: f64 =
                                        *args.get(0).expect(&format!("引数が必要{}", wordend));
                                    for i in args[1..args.len()].to_vec().iter() {
                                        result -= i;
                                    }
                                    Type::Number(result)
                                }
                                "掛け算" => {
                                    let args: Vec<f64> =
                                        args.iter().map(|i| i.get_number()).collect();
                                    let mut result: f64 =
                                        *args.get(0).expect(&format!("引数が必要{}", wordend));
                                    for i in args[1..args.len()].to_vec().iter() {
                                        result *= i;
                                    }
                                    Type::Number(result)
                                }
                                "割り算" => {
                                    let args: Vec<f64> =
                                        args.iter().map(|i| i.get_number()).collect();
                                    let mut result: f64 =
                                        *args.get(0).expect(&format!("引数が必要{}", wordend));
                                    for i in args[1..args.len()].to_vec().iter() {
                                        result /= i;
                                    }
                                    Type::Number(result)
                                }
                                "表示" => {
                                    let output = args
                                        .iter()
                                        .map(|i| i.get_string())
                                        .collect::<Vec<String>>()
                                        .join(" ");
                                    println!("[出力]: {output}",);
                                    Type::String(output)
                                }
                                "移動" => {
                                    pc = memory.get(&args[0].get_string()).expect(&format!(
                                        "指定したラベルが定義されてない{}",
                                        wordend
                                    )).get_number() as usize - 1;
                                    Type::Number(pc as f64)
                                }
                                "入力待ち" => {
                                    Type::String(input(&format!("{}", args[0].get_string())))
                                }
                                other => panic!("定義されてない命令{}：{}", wordend, other),
                            }
                        } else {
                            match code[0] {
                                "終了" => exit(0),
                                other => panic!("定義されてない命令{}：{}", wordend, other),
                            }
                        }
                    };
                    if code.contains("は") {
                        let code: Vec<&str> = code.split("は").collect();
                        let result = eval(code[1].to_string());
                        memory.insert(code[0].to_string(), result);
                    } else {
                        eval(code.to_string());
                    }
                } else if code.ends_with("である") {
                    if code.contains("は") {
                        let code: Vec<&str> = code.split("は").collect();
                        memory.insert(
                            code[0].to_string(),
                            if let Some(value) = memory.get(code[1]) {
                                value.clone()
                            } else if let Ok(i) = code[1].parse::<f64>() {
                                Type::Number(i)
                            } else {
                                Type::String(code[1].to_string())
                            },
                        );
                    } else {
                        memory.insert(code.replace("である", ""), Type::Number(pc as f64));
                    }
                }
            } else {
                panic!("文の終端には「{}」を付ける必要がある{}", wordend, wordend);
            }
        }
        pc += 0;
    }
}
