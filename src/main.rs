use std::{
    collections::HashMap,
    env::args,
    fs::read_to_string,
    io::{self, Write},
    path::Path,
    process::exit,
};

fn main() {
    let args: Vec<String> = args().collect();
    let debug = args.contains(&"--debug".to_string()) || args.contains(&"-d".to_string());
    let args = if debug {
        let mut args = args;
        if let Some(x) = args.iter().position(|x| x == "--debug") {
            args.remove(x);
        }
        if let Some(x) = args.iter().position(|x| x == "-d") {
            args.remove(x);
        }
        args
    } else {
        args
    };
    let (source, wordend): (String, String) = if args.len() > 2 {
        (
            read_to_string(args[1].clone())
                .expect(&format!("ファイルが開けなかった{}", args[2].clone())),
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

    noze(source, wordend.clone(), debug)
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
    None,
}

impl Type {
    fn parse(s: String, memory: &mut HashMap<String, Type>) -> Type {
        let mut s = s.trim().to_string();
        if let Some(value) = memory.get(&s) {
            value.clone()
        } else if let Ok(i) = s.parse::<f64>() {
            Type::Number(i)
        } else if s == "真" {
            Type::Bool(true)
        } else if s == "偽" {
            Type::Bool(false)
        } else if s == "無し" {
            Type::None
        } else if s.starts_with("「") && s.starts_with("「") {
            Type::String({
                s.remove(s.find("「").unwrap_or_default());
                s.remove(s.rfind("」").unwrap_or_default());
                s.to_string()
            })
        } else {
            Type::String(s.to_string())
        }
    }

    fn get_string(&self) -> String {
        match self {
            Type::Number(i) => i.to_string(),
            Type::String(s) => s.to_string(),
            Type::Bool(b) => if *b { "真" } else { "偽" }.to_string(),
            Type::None => "無し".to_string(),
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
            Type::None => 0.0,
        }
    }
    fn get_bool(&self) -> bool {
        match self {
            Type::Number(i) => *i != 0.0,
            Type::String(s) => s == "真",
            Type::Bool(b) => *b,
            Type::None => false,
        }
    }
}

fn noze(source: String, wordend: String, debug: bool) {
    let memory: &mut HashMap<String, Type> = &mut HashMap::new();
    let lines = split_multiple(source, ['。', '！'].to_vec());
    let mut call_stack: Vec<usize> = Vec::new();
    let mut pc: usize = 0;
    let mut output = String::new();

    while pc < lines.len() {
        let code = lines[pc].trim();
        if code.is_empty() {
            continue;
        }
        if debug {
            eprintln!(
                "
プログラムカウンタ：{:?}
命令　　　　　　　：{:?}
呼び出しスタック　：{:?}
記憶領域　　　　　：{:?}
出力：　　　　　　：{:?}",
                pc.clone(),
                code,
                call_stack.clone(),
                memory.clone(),
                output
            );
            input("Enterキーを押してデバック実行継続");
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
                            .map(|s| Type::parse(s.to_string(), memory))
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
                        "余剰" => {
                            let args: Vec<f64> = args.iter().map(|i| i.get_number()).collect();
                            let mut result: f64 =
                                *args.get(0).expect(&format!("引数が必要{}", wordend));
                            for i in args[1..args.len()].to_vec().iter() {
                                result %= i;
                            }
                            Type::Number(result)
                        }
                        "結合" => {
                            let args: Vec<String> = args.iter().map(|i| i.get_string()).collect();
                            Type::String(args.join(""))
                        }
                        "検索" => {
                            let args: Vec<String> = args.iter().map(|i| i.get_string()).collect();
                            Type::Bool(args[0].contains(&args[1]))
                        }
                        "ファイル読み込み" => Type::String(
                            read_to_string(Path::new(&args[0].get_string())).unwrap_or_default(),
                        ),
                        "置換" => {
                            let args: Vec<String> = args.iter().map(|i| i.get_string()).collect();
                            Type::String(args[0].replace(&args[1], &args[2]))
                        }
                        "等価演算" => {
                            let args: Vec<String> = args.iter().map(|i| i.get_string()).collect();
                            Type::Bool(match args.first() {
                                Some(first) => args.iter().all(|x| x == first),
                                None => true,
                            })
                        }
                        "論理否定" => Type::Bool(!args[0].get_bool()),
                        "論理積" => {
                            let args: Vec<bool> = args.iter().map(|i| i.get_bool()).collect();
                            Type::Bool(args.iter().all(|&x| x))
                        }
                        "論理和" => {
                            let args: Vec<bool> = args.iter().map(|i| i.get_bool()).collect();
                            Type::Bool(args.iter().any(|&x| x))
                        }
                        "表示" => {
                            let text = args
                                .iter()
                                .map(|i| i.get_string())
                                .collect::<Vec<String>>()
                                .join(" ");
                            if debug {
                                output += &format!("{text} ");
                            } else {
                                println!("{text}")
                            };
                            Type::None
                        }
                        "移動" => {
                            pc = args[0].get_number() as usize - 1;
                            Type::None
                        }
                        "条件付きで移動" => {
                            if args[1].get_bool() {
                                pc = args[0].get_number() as usize - 1
                            }
                            Type::None
                        }
                        "呼び出し" => {
                            call_stack.push(pc);
                            pc = args[0].get_number() as usize - 1;
                            Type::None
                        }
                        "評価" => Type::parse(args[0].get_string(), memory),
                        "削除" => {
                            memory.remove(&args[0].get_string());
                            Type::None
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
                            Type::None
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
                    let value =
                        Type::parse(code[1].replace("である", "").trim().to_string(), memory);
                    memory.insert(code[0].to_string(), value);
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

        pc += 1;
    }
}
