use std::collections::{HashMap, HashSet};

type Stack<T> = Vec<T>;

pub fn check_parenthesis(sequence: &str, tokens: &HashMap<char, char>) -> Result<(), usize> {
    let closing: HashSet<char> = tokens.values().cloned().collect();
    let opening: HashSet<char> = tokens.keys().cloned().collect();
    let mut stack: Stack<(usize, char)> = Stack::new();
    for (i, c) in (1..).zip(sequence.chars()) {
        if opening.contains(&c) {
            stack.push((i, c));
        } else if closing.contains(&c) {
            let (_, open) = stack.pop().ok_or_else(|| i)?;
            if !tokens.get(&open).map(|close| &c == close).ok_or_else(|| i)? {
                return Err(i);
            }
        }
    }
    if !stack.is_empty() {
        return Err(stack.pop().unwrap().0)
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use crate::stack::check_parenthesis;
    use std::collections::HashMap;

    #[test]
    fn test_check_parenthesis_examples() {
        let tokens : HashMap<char, char> = [('[', ']'), ('(', ')'), ('{', '}')].iter().cloned().collect();
        assert_eq!(check_parenthesis("[]", &tokens), Ok(()));
        assert_eq!(check_parenthesis("[", &tokens), Err(1));
        assert_eq!(check_parenthesis("{[}", &tokens), Err(3));
        assert_eq!(check_parenthesis("foo(bar)", &tokens), Ok(()));
        assert_eq!(check_parenthesis("foo(bar[i)", &tokens), Err(10));
    }
}