use core::panic;

fn main() {
    for line in std::io::stdin().lines() {
        let mut stack = vec![];

        let Ok(line) = line else {
            continue;
        };

        for word in line.split_whitespace().collect::<Vec<_>>() {
            match word.parse::<i32>() {
                Ok(parsed) => stack.push(parsed),
                _ => match word {
                    "+" => add(&mut stack),
                    "-" => sub(&mut stack),
                    "*" => mul(&mut stack),
                    "/" => div(&mut stack),
                    _ => panic!("{word:?} could not be parsed"),
                },
            }
        }

        println!("stack: {stack:?}");
    }
}

fn add(stack: &mut Vec<i32>) {
    let hls = stack.pop().unwrap();
    let lls = stack.pop().unwrap();
    stack.push(hls + lls);
}

fn sub(stack: &mut Vec<i32>) {
    let hls = stack.pop().unwrap();
    let lls = stack.pop().unwrap();
    stack.push(hls - lls);
}

fn mul(stack: &mut Vec<i32>) {
    let hls = stack.pop().unwrap();
    let lls = stack.pop().unwrap();
    stack.push(hls * lls);
}

fn div(stack: &mut Vec<i32>) {
    let hls = stack.pop().unwrap();
    let lls = stack.pop().unwrap();
    stack.push(hls / lls);
}

mod tests {
    #[test]
    fn test_add() {
        let mut stack = vec![1, 2];
        super::add(&mut stack);
        assert_eq!(stack, vec![3]);
    }

    #[test]
    fn test_sub() {
        let mut stack = vec![1, 2];
        super::sub(&mut stack);
        assert_eq!(stack, vec![1]);
    }
    #[test]
    fn test_mul() {
        let mut stack = vec![1, 2];
        super::mul(&mut stack);
        assert_eq!(stack, vec![2]);
    }
    #[test]
    fn test_div() {
        let mut stack = vec![2, 4];
        super::div(&mut stack);
        assert_eq!(stack, vec![2]);
    }
}
