#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    use CalculatorInput::*;
    let mut stack = vec![];
    for i in inputs {
        if let Value(a) = i {
            stack.push(a.clone());
        } else {
            if stack.len() < 2 {
                return None;
            }
            let b = stack.pop().unwrap();
            let a = stack.pop().unwrap();
            match i {
                Add => stack.push(a + b),
                Multiply => stack.push(a * b),
                Subtract => stack.push(a - b),
                Divide => stack.push(a / b),
                _ => (),
            }
        }
    }
    if stack.len() == 1 {
        stack.pop()
    } else {
        None
    }
}
