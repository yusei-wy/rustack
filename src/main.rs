use std::{
    collections::HashMap,
    io::{BufRead, BufReader},
};

/// Stack の要素
#[derive(Clone, Debug, PartialEq, Eq)]
enum Value {
    Num(i32),
    Op(String),
    Sym(String),
    Block(Vec<Value>),
    Native(NativeOp),
}

impl Value {
    fn as_num(&self) -> i32 {
        match self {
            Self::Num(val) => *val,
            _ => panic!("Value is not a number"),
        }
    }

    fn as_sym(&self) -> &str {
        if let Self::Sym(sym) = self {
            sym
        } else {
            panic!("Value is not a symbol");
        }
    }

    fn to_block(self) -> Vec<Value> {
        match self {
            Self::Block(val) => val,
            _ => panic!("Value is not a block"),
        }
    }

    fn to_string(&self) -> String {
        match self {
            Self::Num(i) => i.to_string(),
            Self::Op(s) | Self::Sym(s) => s.clone(),
            Self::Block(_) => "<Block>".to_string(),
            Self::Native(_) => "<Native".to_string(),
        }
    }
}

/// 変数
struct Vm {
    stack: Vec<Value>,
    vars: HashMap<String, Value>,
    blocks: Vec<Vec<Value>>,
}

impl Vm {
    fn new() -> Self {
        let functions: [(&str, fn(&mut Vm)); 12] = [
            ("+", add),
            ("-", sub),
            ("*", mul),
            ("/", div),
            ("<", lt),
            ("if", op_if),
            ("def", op_def),
            ("puts", puts),
            ("pop", pop),
            ("dup", dup),
            ("exch", exch),
            ("index", index),
        ];

        Self {
            stack: vec![],
            vars: functions
                .into_iter()
                .map(|(name, fun)| (name.to_owned(), Value::Native(NativeOp(fun))))
                .collect(),
            blocks: vec![],
        }
    }
}

/// ネイティブ関数
#[derive(Clone)]
struct NativeOp(fn(&mut Vm));

impl std::fmt::Debug for NativeOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<NativeOp>")
    }
}

impl PartialEq for NativeOp {
    fn eq(&self, other: &NativeOp) -> bool {
        self.0 as *const fn() == other.0 as *const fn()
    }
}

impl Eq for NativeOp {}

fn main() {
    if let Some(f) = std::env::args()
        .nth(1)
        .and_then(|f| std::fs::File::open(f).ok())
    {
        parse_batch(BufReader::new(f));
    } else {
        parse_interactive();
    }
}

/// ファイルをパースしてスタックを返す
fn parse_batch(source: impl BufRead) -> Vec<Value> {
    let mut vm = Vm::new();
    for line in source.lines().flatten() {
        for word in line.split(" ") {
            parse_word(word, &mut vm);
        }
    }

    vm.stack
}

/// 標準入力から読み込んだ文字列をパースする
fn parse_interactive() {
    let mut vm = Vm::new();
    for line in std::io::stdin().lines().flatten() {
        for word in line.split(" ") {
            parse_word(word, &mut vm);
        }
        println!("stack: {:?}", vm.stack);
    }
}

/// 入力をパースしてスタックを返す
fn parse_word(word: &str, vm: &mut Vm) {
    if word.is_empty() {
        return;
    }

    if word == "{" {
        vm.blocks.push(vec![]);
    } else if word == "}" {
        let top_block = vm.blocks.pop().expect("Block stack underrun!");
        eval(&Value::Block(top_block), vm);
    } else {
        let code = if let Ok(parsed) = word.parse::<i32>() {
            Value::Num(parsed)
        } else if word.starts_with("/") {
            Value::Sym(word[1..].to_string()) // 先頭の `/` をスキップ
        } else {
            Value::Op(word.to_string())
        };
        eval(&code, vm);
    }
}

/// Value を評価する
fn eval(code: &Value, vm: &mut Vm) {
    // ブロックは優先して処理する
    if let Some(top_block) = vm.blocks.last_mut() {
        top_block.push(code.clone());
        return;
    }

    if let Value::Op(op) = code {
        let val = vm
            .vars
            .get(op)
            .expect(&format!("{op:?} is not a defined operation"))
            .clone();
        match val {
            Value::Block(block) => {
                for code in block {
                    eval(&code, vm);
                }
            }
            Value::Native(op) => op.0(vm),
            _ => vm.stack.push(val),
        }
    } else {
        vm.stack.push(code.clone());
    }
}

/// operation を定義する
macro_rules! impl_op {
    {$name:ident, $op:tt} => {
        fn $name(vm: &mut Vm)  {
            let rhs = vm.stack.pop().unwrap().as_num();
            let lhs = vm.stack.pop().unwrap().as_num();
            vm.stack.push(Value::Num((lhs $op rhs) as i32));
        }
    }
}

impl_op!(add, +);
impl_op!(sub, -);
impl_op!(mul, *);
impl_op!(div, /);
impl_op!(lt, <);

/// if 文を評価する
/// ```
/// { cond } { true_branch } { false_branch } if
/// ```
fn op_if(vm: &mut Vm) {
    let false_branch = vm.stack.pop().unwrap().to_block();
    let true_branch = vm.stack.pop().unwrap().to_block();
    let cond = vm.stack.pop().unwrap().to_block();

    for code in cond {
        eval(&code, vm);
    }

    let cond_result = vm.stack.pop().unwrap().as_num();
    let branch = if cond_result == 0 {
        false_branch
    } else {
        true_branch
    };

    for code in branch {
        eval(&code, vm);
    }
}

/// 変数を定義する
fn op_def(vm: &mut Vm) {
    let value = vm.stack.pop().unwrap();
    eval(&value, vm);
    let value = vm.stack.pop().unwrap();
    let sym = vm.stack.pop().unwrap().as_sym().to_string();

    vm.vars.insert(sym, value);
}

/// スタックの最上位を文字列として出力する
fn puts(vm: &mut Vm) {
    let value = vm.stack.pop().unwrap();
    println!("{}", value.to_string());
}

/// スタックの最上位を取り除く
fn pop(vm: &mut Vm) {
    vm.stack.pop().unwrap();
}

/// スタックの最上位を複製する
fn dup(vm: &mut Vm) {
    let value = vm.stack.last().unwrap();
    vm.stack.push(value.clone());
}

///スタックの最上位とその下を交換する
fn exch(vm: &mut Vm) {
    let last = vm.stack.pop().unwrap();
    let second = vm.stack.pop().unwrap();
    vm.stack.push(last);
    vm.stack.push(second);
}

fn index(vm: &mut Vm) {
    let index = vm.stack.pop().unwrap().as_num() as usize;
    let value = vm.stack[vm.stack.len() - index - 1].clone();
    vm.stack.push(value);
}

#[cfg(test)]
mod tests {
    use super::{Value::*, *};
    use std::io::Cursor;

    fn parse(input: &str) -> Vec<Value> {
        parse_batch(Cursor::new(input))
    }

    #[test]
    fn test_group() {
        assert_eq!(
            parse("1 2 + { 3 4 }"),
            vec![Num(3), Block(vec![Num(3), Num(4)])]
        );
    }

    #[test]
    fn test_if_false() {
        assert_eq!(parse("{ 1 -1 + } { 100 } { -100 } if"), vec![Num(-100)],);
    }

    #[test]
    fn test_if_true() {
        assert_eq!(parse("{ 1 1 + } { 100 } { -100 } if"), vec![Num(100)]);
    }

    #[test]
    fn test_var() {
        assert_eq!(parse("/x 10 def /y 20 def x y *"), vec![Num(200)]);
    }

    #[test]
    fn test_var_if() {
        assert_eq!(
            parse("/x 10 def /y 20 def { x y < } { x } { y } if"),
            vec![Num(10)]
        );
    }

    #[test]
    fn test_multiline() {
        assert_eq!(
            parse(
                r#"
            /x 10 def
            /y 20 def

            { x y < }
            { x }
            { y }
            if
            "#
            ),
            vec![Num(10)]
        );
    }

    #[test]
    fn test_function() {
        assert_eq!(
            parse(
                r#"
/double { 2 * } def
10 double
"#
            ),
            vec![Num(20)]
        )
    }
}
