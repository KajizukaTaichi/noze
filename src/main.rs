use regex::Regex;
use std::{
    collections::HashMap,
    env::{self, args},
    fs::{self, read_to_string},
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
            if let Ok(file) = read_to_string(args[1].clone()) {
                file
            } else {
                repl(args[2].to_string(), debug);
                return;
            },
            args[2].to_string(),
        )
    } else if args.len() > 1 {
        (
            if let Ok(file) = read_to_string(args[1].clone()) {
                file
            } else {
                repl("のぜ".to_string(), debug);
                return;
            },
            "のぜ".to_string(),
        )
    } else {
        repl("のぜ".to_string(), debug);
        return;
    };

    noze(source, wordend.clone(), debug)
}

fn repl(wordend: String, debug: bool) {
    println!("Noze：日本語プログラミング言語なのぜ！！！ \n(c) 2024 梶塚太智. All rights reserved");
    loop {
        let mut code = String::new();
        loop {
            let enter = input("> ").trim().to_string();
            code += &enter;
            if enter.is_empty() {
                break;
            }
        }

        noze(code, wordend.clone(), debug);
    }
}

fn input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut result = String::new();
    io::stdin().read_line(&mut result).ok();
    result.trim().to_string()
}

fn convert_to_usize(text: &str) -> Option<usize> {
    let re = Regex::new(r"^[０１２３４５６７８９]+$").unwrap();
    if !re.is_match(text) {
        return None;
    }

    let map: HashMap<char, usize> = vec![
        ('０', 0),
        ('１', 1),
        ('２', 2),
        ('３', 3),
        ('４', 4),
        ('５', 5),
        ('６', 6),
        ('７', 7),
        ('８', 8),
        ('９', 9),
    ]
    .into_iter()
    .collect();

    let mut result = String::new();
    for c in text.chars() {
        if let Some(&value) = map.get(&c) {
            result += &value.to_string()
        } else {
            return None;
        }
    }

    Some(result.parse().unwrap_or_default())
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
    Array(Vec<Type>),
    None,
}

impl Type {
    fn parse(s: String, memory: &mut HashMap<String, Type>) -> Type {
        let mut s = s.trim().to_string();
        if let Some(value) = memory.get(&s) {
            value.clone()
        } else if let Ok(i) = s.parse::<f64>() {
            Type::Number(i)
        } else if let Some(i) = convert_to_usize(&s) {
            Type::Number(i as f64)
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
        } else if s.starts_with("（") && s.starts_with("）") {
            Type::Array({
                s.remove(s.find("（").unwrap_or_default());
                s.remove(s.rfind("）").unwrap_or_default());
                split_multiple(s, vec!['、', ','])
                    .into_iter()
                    .map(|i| Type::parse(i.to_string(), memory))
                    .collect()
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
            Type::Array(a) => format!(
                "（ {} ）",
                a.iter()
                    .map(|i| i.get_string())
                    .collect::<Vec<String>>()
                    .join("、")
            ),
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
            Type::Array(a) => a.get(0).unwrap_or(&Type::None).get_number(),
            Type::None => 0.0,
        }
    }
    fn get_bool(&self) -> bool {
        match self {
            Type::Number(i) => *i != 0.0,
            Type::String(s) => s == "真",
            Type::Bool(b) => *b,
            Type::Array(a) => !a.is_empty(),
            Type::None => false,
        }
    }

    fn get_array(&self) -> Vec<Type> {
        match self {
            Type::Number(i) => vec![Type::Number(*i)],
            Type::String(s) => s
                .chars()
                .into_iter()
                .map(|i| Type::String(i.to_string()))
                .collect(),
            Type::Bool(b) => vec![Type::Bool(*b)],
            Type::Array(a) => a.clone(),
            Type::None => vec![],
        }
    }
}

fn noze(source: String, wordend: String, debug: bool) {
    let memory: &mut HashMap<String, Type> = &mut HashMap::new();
    let lines = split_multiple(source, ['。', '.'].to_vec());
    let mut pc: usize = 0;

    // Preprocessing
    while pc < lines.len() {
        let code = lines[pc].trim();
        if code.is_empty() {
            continue;
        }
        if code.ends_with(&wordend) {
            let code = code.replace(&wordend, "");
            if code.ends_with("である") {
                if !code.contains("は") {
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
プログラムカウンタ：{}
命令　　　　　　　：{}
呼び出しスタック　：[ {} ]
記憶領域　　　　　：[ {} ]
出力：　　　　　　：{}",
                pc,
                code,
                call_stack
                    .iter()
                    .map(|i| i.to_string())
                    .collect::<Vec<String>>()
                    .join(", "),
                memory
                    .iter()
                    .map(|(k, v)| format!("{k}: {}", v.get_string()))
                    .collect::<Vec<String>>()
                    .join(", "),
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
                        "等しいか比較" => {
                            let args: Vec<String> = args.iter().map(|i| i.get_string()).collect();
                            Type::Bool(args.windows(2).all(|window| window[0] == window[1]))
                        }
                        "大きいか比較" => {
                            let args: Vec<f64> = args.iter().map(|i| i.get_number()).collect();
                            Type::Bool(args.windows(2).all(|window| window[0] > window[1]))
                        }
                        "小さいか比較" => {
                            let args: Vec<f64> = args.iter().map(|i| i.get_number()).collect();
                            Type::Bool(args.windows(2).all(|window| window[0] < window[1]))
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
                        "配列に" => Type::Array(args),
                        "配列の要素の取得" => {
                            args[0].get_array()[args[1].get_number() as usize].clone()
                        }
                        "配列の要素の削除" => {
                            let mut array = args[0].get_array();
                            array.remove(args[1].get_number() as usize);
                            Type::Array(array)
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
                        "現在ディレクトリに" => {
                            env::set_current_dir(Path::new(&args[0].get_string())).unwrap();
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
                        "アイテム一覧の取得" => Type::Array(
                            fs::read_dir(".")
                                .unwrap()
                                .filter_map(|entry| {
                                    entry
                                        .ok()
                                        .and_then(|e| e.file_name().into_string().ok())
                                        .map(Type::String)
                                })
                                .collect(),
                        ),
                        "現在ディレクトリの取得" => {
                            Type::String(env::current_dir().unwrap().to_str().unwrap().to_string())
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
                }
            }
        } else {
            panic!("文の終端には「{}」を付ける必要がある{}", wordend, wordend);
        }
        pc += 1;
    }
}
