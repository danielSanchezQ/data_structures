use std::collections::{HashMap, HashSet};
use std::fmt::Display;
use std::str::FromStr;

pub type Stack<T> = Vec<T>;

pub fn check_parenthesis(sequence: &str, tokens: &HashMap<char, char>) -> Result<(), usize> {
    let closing: HashSet<char> = tokens.values().cloned().collect();
    let opening: HashSet<char> = tokens.keys().cloned().collect();
    let mut stack: Stack<(usize, char)> = Stack::new();
    for (i, c) in (1..).zip(sequence.chars()) {
        if opening.contains(&c) {
            stack.push((i, c));
        } else if closing.contains(&c) {
            let (_, open) = stack.pop().ok_or_else(|| i)?;
            if !tokens
                .get(&open)
                .map(|close| &c == close)
                .ok_or_else(|| i)?
            {
                return Err(i);
            }
        }
    }
    if !stack.is_empty() {
        return Err(stack.pop().unwrap().0);
    }
    Ok(())
}

pub struct MaxCachedStack<T: PartialOrd + Clone>(Stack<T>);

impl<T> MaxCachedStack<(T, T)>
where
    T: PartialOrd + Clone + Display,
{
    pub fn new() -> Self {
        Self(Stack::new())
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self(Stack::with_capacity(capacity))
    }

    pub fn push(&mut self, value: T) {
        let max = if let Some((_, m)) = self.0.last() {
            if m > &value {
                m.clone()
            } else {
                value.clone()
            }
        } else {
            value.clone()
        };
        self.0.push((value, max));
    }

    pub fn pop(&mut self) -> Option<T> {
        self.0.pop().map(|(e, _)| e)
    }

    pub fn max(&self) -> Option<T> {
        self.0.last().map(|(_, m)| m).cloned()
    }
}

pub enum StackCommand<T: PartialOrd + Clone> {
    Push(T),
    Pop,
    Max,
}

impl<T> StackCommand<T>
where
    T: PartialOrd + Clone + FromStr,
{
    pub fn from_str(input: &[&str]) -> Option<StackCommand<T>> {
        match input {
            ["push", e] => Some(Self::Push(e.parse().ok()?)),
            ["pop"] => Some(Self::Pop),
            ["max"] => Some(Self::Max),
            _ => None,
        }
    }
}

pub fn compute_max_stack_input<T>(commands: &[StackCommand<T>])
where
    T: PartialOrd + Clone + Display,
{
    let mut stack: MaxCachedStack<(T, T)> = MaxCachedStack::new();
    for command in commands {
        match command {
            StackCommand::Push(v) => {
                stack.push(v.clone());
            }
            StackCommand::Pop => {
                stack.pop();
            }
            StackCommand::Max => {
                println!("{}", stack.max().unwrap())
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::stack::{check_parenthesis, compute_max_stack_input, StackCommand};
    use std::collections::HashMap;

    #[test]
    fn test_check_parenthesis_examples() {
        let tokens: HashMap<char, char> = [('[', ']'), ('(', ')'), ('{', '}')]
            .iter()
            .cloned()
            .collect();
        assert_eq!(check_parenthesis("[]", &tokens), Ok(()));
        assert_eq!(check_parenthesis("[", &tokens), Err(1));
        assert_eq!(check_parenthesis("{[}", &tokens), Err(3));
        assert_eq!(check_parenthesis("foo(bar)", &tokens), Ok(()));
        assert_eq!(check_parenthesis("foo(bar[i)", &tokens), Err(10));
    }

    #[test]
    fn test_max_cached_stack_examples() {
        let commands = [
            StackCommand::Push(2usize),
            StackCommand::Push(1),
            StackCommand::Max,
            StackCommand::Pop,
            StackCommand::Max,
        ];
        compute_max_stack_input(&commands);
    }
}
