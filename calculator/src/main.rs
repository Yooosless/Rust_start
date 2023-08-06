use std::env::args;

fn main() {
    println!("Hello, world!");

    let mut args = args();

    args.next();


    let first: i64 = args.next().unwrap().parse().unwrap();

    let operator = args.next().unwrap();

    let operator_char=operator.chars().next().unwrap();

   
    let second: i64 = args.next().unwrap().parse().unwrap();


    let result = match operator_char {
        '+' => first + second,
        '-' => first - second,
        '*' => first * second,
        '/' => first / second,
        _ => {
            println!("Invalid operator");
            return;
        }
    };

    println!("{:?}",format_to_string(first, operator_char, second, result) );
}

fn format_to_string(first:i64,operator_char:char,second:i64,result:i64)->String{
    format!("{}{}{}= {}",first,operator_char,second,result)
}