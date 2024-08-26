use std::{
    collections::HashMap,
    env::args,
    fs::read_to_string,
    io::{self, Write},
    process::exit,
};

fn main() {
    let args: Vec<String> = args().collect();
    let (source, wordend): (String, String) = if args.len() > 2 {
        (
            read_to_string(args[1].clone())
                .expect(&format!("ファイルが存在しない{}", args[2].clone())),
            args[2].clone(),
        )
    } else if args.len() > 1 {
        (
            read_to_string(args[1].clone()).expect("ファイルが存在しないのぜ"),
            "のぜ".to_string(),
        )
    } else {
        println!(
            "Noze：日本語プログラミング言語なのぜ！！！ \n(c) 2024 梶塚太智. All rights reserved"
        );
        return;
    };

    noze(source, wordend.clone())
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

#[derive(Clone, Debug)]
enum Type {
    Number(f64),
    String(String),
    Bool(bool),
}

impl Type {
    fn get_string(&self) -> String {
        match self {
            Type::Number(i) => i.to_string(),
            Type::String(s) => s.to_string(),
            Type::Bool(b) => b.to_string(),
        }
    }
    fn get_number(&self) -> f64 {
        match self {
            Type::Number(i) => *i,
            Type::String(s) => s.parse().unwrap_or_default(),
            Type::Bool(b) => {
                if *b {
                    1.0
                } else {
                    0.0
                }
            }
        }
    }
    fn get_bool(&self) -> bool {
        match self {
            Type::Number(i) => *i != 0.0,
            Type::String(s) => s.parse().unwrap_or_default(),
            Type::Bool(b) => *b,
        }
    }
}

fn noze(source: String, wordend: String) {
    let memory: &mut HashMap<String, Type> = &mut HashMap::new();
    let lines = split_multiple(source, ['。', '！'].to_vec());
    let mut call_stack: Vec<usize> = Vec::new();
    let mut pc: usize = 0;
    while pc < lines.len() {
        let code = lines[pc].trim();
        if code.is_empty() {
            continue;
        }
        if code.ends_with(&wordend) {
            let code = code.replace(&wordend, "");
            if code.ends_with("する") {
                let code = code.replace("する", "");
                let (name, code) = if code.contains("は") {
                    let code: Vec<&str> = code.split("は").collect();
                    (
                        Some(code[0..code.len() - 1].join("は")),
                        code[code.len() - 1].to_string(),
                    )
                } else {
                    (None, code.to_string())
                };
                let code: Vec<&str> = code.split("を").collect();
                let result: Type = if code.len() > 1 {
                    let (order, args): (String, Vec<Type>) = (
                        code[code.len() - 1].to_string(),
                        code[0..code.len() - 1]
                            .join("を")
                            .split("と")
                            .into_iter()
                            .map(|s| {
                                let mut s = s.trim().to_string();
                                if let Some(value) = memory.get(&s) {
                                    value.clone()
                                } else if let Ok(i) = s.parse::<f64>() {
                                    Type::Number(i)
                                } else if let Ok(b) = s.parse::<bool>() {
                                    Type::Bool(b)
                                } else if s.starts_with("「") && s.starts_with("「") {
                                    Type::String({
                                        s.remove(s.find("「").unwrap_or_default());
                                        s.remove(s.rfind("」").unwrap_or_default());
                                        s.to_string()
                                    })
                                } else {
                                    Type::String(s.to_string())
                                }
                            })
                            .collect(),
                    );

                    match order.as_str() {
                        "足し算" => {
                            let args: Vec<f64> = args.iter().map(|i| i.get_number()).collect();
                            let mut result: f64 =
                                *args.get(0).expect(&format!("引数が必要{}", wordend));
                            for i in args[1..args.len()].to_vec().iter() {
                                result += i;
                            }
                            Type::Number(result)
                        }
                        "引き算" => {
                            let args: Vec<f64> = args.iter().map(|i| i.get_number()).collect();
                            let mut result: f64 =
                                *args.get(0).expect(&format!("引数が必要{}", wordend));
                            for i in args[1..args.len()].to_vec().iter() {
                                result -= i;
                            }
                            Type::Number(result)
                        }
                        "掛け算" => {
                            let args: Vec<f64> = args.iter().map(|i| i.get_number()).collect();
                            let mut result: f64 =
                                *args.get(0).expect(&format!("引数が必要{}", wordend));
                            for i in args[1..args.len()].to_vec().iter() {
                                result *= i;
                            }
                            Type::Number(result)
                        }
                        "割り算" => {
                            let args: Vec<f64> = args.iter().map(|i| i.get_number()).collect();
                            let mut result: f64 =
                                *args.get(0).expect(&format!("引数が必要{}", wordend));
                            for i in args[1..args.len()].to_vec().iter() {
                                result /= i;
                            }
                            Type::Number(result)
                        }
                        "結合" => {
                            let args: Vec<String> = args.iter().map(|i| i.get_string()).collect();

                            Type::String(args.join(""))
                        }
                        "等価演算" => {
                            let args: Vec<String> = args.iter().map(|i| i.get_string()).collect();
                            Type::Bool(match args.first() {
                                Some(first) => args.iter().all(|x| x == first),
                                None => true, // ベクタが空の場合はtrueとする
                            })
                        }
                        "論理否定" => Type::Bool(!args[0].get_bool()),
                        "表示" => {
                            let output = args
                                .iter()
                                .map(|i| i.get_string())
                                .collect::<Vec<String>>()
                                .join(" ");
                            println!("{output}",);
                            Type::String(output)
                        }
                        "移動" => {
                            pc = args[0].get_number() as usize - 1;
                            Type::Number(pc as f64)
                        }
                        "条件付きで移動" => {
                            if args[1].get_bool() {
                                pc = args[0].get_number() as usize - 1
                            }
                            Type::Number(pc as f64)
                        }
                        "呼び出し" => {
                            call_stack.push(pc);
                            pc = args[0].get_number() as usize - 1;
                            Type::Number(pc as f64)
                        }

                        "入力待ち" => Type::String(input(&format!("{}", args[0].get_string()))),
                        other => panic!("定義されてない命令{}：{}", wordend, other),
                    }
                } else {
                    match code[0] {
                        "終了" => exit(0),
                        "帰還" => {
                            pc = call_stack
                                .pop()
                                .expect(&format!("呼び出しスタックが空{}", wordend));
                            Type::Number(pc as f64)
                        }
                        other => panic!("定義されてない命令{}：{}", wordend, other),
                    }
                };
                if let Some(name) = name {
                    memory.insert(name, result);
                }
            } else if code.ends_with("である") {
                if code.contains("は") {
                    let code: Vec<&str> = code.split("は").collect();
                    memory.insert(code[0].to_string(), {
                        let value = code[1].replace("である", "").trim().to_string();
                        if let Some(value) = memory.get(&value) {
                            value.clone()
                        } else if let Ok(i) = value.parse::<f64>() {
                            Type::Number(i)
                        } else {
                            Type::String(value.to_string())
                        }
                    });
                } else {
                    memory.insert(
                        code.replace("である", "").trim().to_string(),
                        Type::Number(pc as f64),
                    );
                }
            }
        } else {
            panic!("文の終端には「{}」を付ける必要がある{}", wordend, wordend);
        }
        // dbg!(pc.clone(), call_stack.clone(), memory.clone());
        pc += 1;
    }
}
