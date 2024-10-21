use std::io::stdin;

fn main() {
    let mut memories: Vec<f64> = vec![0.0; 10];
    let mut prev_reuslt: f64 = 0.0;
    for line in stdin().lines() {
        let line = line.unwrap();
        if line.is_empty() {
            break;
        }

        let tokens: Vec<&str> = line.split_whitespace().collect();
        
        if tokens[0] == "mem+" {
            memory += prev_reuslt;
            print_value(memory);
            continue;
        } else if tokens[0] == "mem-" {
            memory -= prev_reuslt;
            print_value(memory);
            continue;
        }

        let left: f64 = eval_token(tokens[0], memory);
        let right: f64 = eval_token(tokens[2], memory);
        let result: f64 = eval_expression(left, tokens[1], right);

        
        print_value(result);
        prev_reuslt = result;
    }
}

fn print_value(value: f64) {
    println!("{}", value);
}

fn add(left: f64, right: f64) -> f64 {
    left + right
}

fn subtract(left: f64, right: f64) -> f64 {
    left - right
}

fn multiply(left: f64, right: f64) -> f64 {
    left * right
}

fn divide(left: f64, right: f64) -> f64 {
    left / right
}

fn eval_token(token: &str, memory: f64) -> f64 {
    if token == "mem" {
        memory
    } else {
        token.parse().unwrap()
    }
}

fn eval_expression(left: f64, operator: &str, right: f64) -> f64 {
    match operator {
        "+" => add(left, right),
        "-" => subtract(left, right),
        "*" => multiply(left, right),
        "/" => divide(left, right),
        _ => unreachable!(),
    }
}
