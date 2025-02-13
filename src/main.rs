use core::panic;

/// Stack の要素
#[derive(Clone, Debug, PartialEq, Eq)]
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

    fn to_block(self) -> Vec<Value<'src>> {
        match self {
            Self::Block(val) => val,
            _ => panic!("Value is not a block"),
        }
    }
}

fn main() {
    for line in std::io::stdin().lines().flatten() {
        parse(&line);
    }
}

/// 入力をパースしてスタックを返す
fn parse<'a>(line: &'a str) -> Vec<Value<'a>> {
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
        } else {
            let code = if let Ok(parsed) = word.parse::<i32>() {
                Value::Num(parsed)
            } else {
                Value::Op(word)
            };
            eval(code, &mut stack);
        }

        words = rest;
    }

    println!("stack: {stack:?}");

    stack
}

/// ブロックをパースしてスタックを返す
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

/// Value を評価する
fn eval<'src>(code: Value<'src>, stack: &mut Vec<Value<'src>>) {
    match code {
        Value::Op(op) => match op {
            "+" => add(stack),
            "-" => sub(stack),
            "*" => mul(stack),
            "/" => div(stack),
            "if" => op_if(stack),
            _ => panic!("{op:?} could not be parsed"),
        },
        _ => stack.push(code.clone()),
    }
}

/// stack から値を pop して合計した結果を stack に push する
fn add(stack: &mut Vec<Value>) {
    let hls = stack.pop().unwrap().as_num();
    let lls = stack.pop().unwrap().as_num();
    stack.push(Value::Num(hls + lls));
}

/// stack から値を pop して差を取った結果を stack に push する
fn sub(stack: &mut Vec<Value>) {
    let hls = stack.pop().unwrap().as_num();
    let lls = stack.pop().unwrap().as_num();
    stack.push(Value::Num(hls - lls));
}

/// stack から値を pop して積を取った結果を stack に push する
fn mul(stack: &mut Vec<Value>) {
    let hls = stack.pop().unwrap().as_num();
    let lls = stack.pop().unwrap().as_num();
    stack.push(Value::Num(hls * lls));
}

/// stack から値を pop して商を取った結果を stack に push する
fn div(stack: &mut Vec<Value>) {
    let hls = stack.pop().unwrap().as_num();
    let lls = stack.pop().unwrap().as_num();
    stack.push(Value::Num(hls / lls));
}

/// if 文を評価する
/// ```
/// { cond } { true_branch } { false_branch } if
/// ```
fn op_if(stack: &mut Vec<Value>) {
    let false_branch = stack.pop().unwrap().to_block();
    let true_branch = stack.pop().unwrap().to_block();
    let cond = stack.pop().unwrap().to_block();

    for code in cond {
        eval(code, stack);
    }

    let cond_result = stack.pop().unwrap().as_num();

    let branch = if cond_result == 0 {
        false_branch
    } else {
        true_branch
    };

    for code in branch {
        eval(code, stack);
    }
}

#[cfg(test)]
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

    #[test]
    fn test_if_false() {
        assert_eq!(parse("{ 1 -1 + } { 100 } { -100 } if"), vec![Num(-100)],);
    }

    #[test]
    fn test_if_true() {
        assert_eq!(parse("{ 1 1 + } { 100 } { -100 } if"), vec![Num(100)],);
    }
}
