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

mod tests {
    #[test]
    fn test_add() {
        let mut stack = vec![1, 2];
        super::add(&mut stack);
        assert_eq!(stack, vec![3]);
    }
}
