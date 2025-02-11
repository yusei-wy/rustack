use core::panic;

#[derive(Debug, PartialEq, Eq)]
enum Value<'src> {
    Num(i32),
    Op(&'src str),
    Block(Vec<Value<'src>>),
}

impl<'src> Value<'src> {
    fn as_num(&self) -> i32 {
        match self {
            Self::Num(val) => *val,
            _ => panic!("Value is not a number"),
        }
    }
}

fn main() {
    for line in std::io::stdin().lines().flatten() {
        parse(&line);
    }
}

fn parse<'a>(line: &'a str) -> Vec<Value> {
    let mut stack = vec![];
    let input = line.split_whitespace().collect::<Vec<_>>();
    let mut words = &input[..];

    while let Some((&word, mut rest)) = words.split_first() {
        if word.is_empty() {
            break;
        }

        if word == "{" {
            let value;
            (value, rest) = parse_block(rest);
            stack.push(value);
        } else if let Ok(parsed) = word.parse::<i32>() {
            stack.push(Value::Num(parsed));
        } else {
            match word {
                "+" => add(&mut stack),
                "-" => sub(&mut stack),
                "*" => mul(&mut stack),
                "/" => div(&mut stack),
                _ => panic!("{word:?} could not be parsed"),
            }
        }

        words = rest;
    }

    println!("stack: {stack:?}");

    stack
}

fn parse_block<'src, 'a>(input: &'a [&'src str]) -> (Value<'src>, &'a [&'src str]) {
    let mut tokens = vec![];
    let mut words = input;

    while let Some((&word, mut rest)) = words.split_first() {
        if word.is_empty() {
            break;
        }

        if word == "{" {
            let value;
            (value, rest) = parse_block(rest);
            tokens.push(value);
        } else if word == "}" {
            return (Value::Block(tokens), rest);
        } else if let Ok(value) = word.parse::<i32>() {
            tokens.push(Value::Num(value));
        } else {
            tokens.push(Value::Op(word));
        }

        words = rest;
    }

    (Value::Block(tokens), words)
}

fn add(stack: &mut Vec<Value>) {
    let hls = stack.pop().unwrap().as_num();
    let lls = stack.pop().unwrap().as_num();
    stack.push(Value::Num(hls + lls));
}

fn sub(stack: &mut Vec<Value>) {
    let hls = stack.pop().unwrap().as_num();
    let lls = stack.pop().unwrap().as_num();
    stack.push(Value::Num(hls - lls));
}

fn mul(stack: &mut Vec<Value>) {
    let hls = stack.pop().unwrap().as_num();
    let lls = stack.pop().unwrap().as_num();
    stack.push(Value::Num(hls * lls));
}

fn div(stack: &mut Vec<Value>) {
    let hls = stack.pop().unwrap().as_num();
    let lls = stack.pop().unwrap().as_num();
    stack.push(Value::Num(hls / lls));
}

mod tests {
    use super::{parse, Value::*};

    #[test]
    fn test_add() {
        assert_eq!(
            parse("1 2 + { 3 4 }"),
            vec![Num(3), Block(vec![Num(3), Num(4)])]
        );
    }

    #[test]
    fn test_sub() {
        assert_eq!(
            parse("1 2 - { 3 4 }"),
            vec![Num(-1), Block(vec![Num(3), Num(4)])]
        );
    }

    #[test]
    fn test_mul() {
        assert_eq!(
            parse("1 2 * { 3 4 }"),
            vec![Num(2), Block(vec![Num(3), Num(4)])]
        );
    }

    #[test]
    fn test_div() {
        assert_eq!(
            parse("1 2 / { 3 4 }"),
            vec![Num(0), Block(vec![Num(3), Num(4)])]
        );
    }
}
