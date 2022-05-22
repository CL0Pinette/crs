use super::super::utils::stack::Stack;
use std::collections::HashMap;

pub(crate) struct Regexer {
    map: HashMap<char, i32>,
}

impl Regexer {
    pub(crate) fn new() -> Self {
        let mut p = Regexer {
            map: HashMap::new(),
        };
        p.map.insert('(', 1);
        p.map.insert('|', 2);
        p.map.insert('.', 3);
        p.map.insert('?', 4);
        p.map.insert('*', 4);
        p.map.insert('+', 4);
        p.map.insert('^', 5);
        p
    }
    pub(crate) fn get_precedence(&mut self, c: char) -> i32 {
        if self.map.contains_key(&c) {
            *self.map.get(&c).unwrap()
        } else {
            6
        }
    }
    fn format_regex(&mut self, regex: &str) -> String {
        let mut res = String::new();
        let all_operators = vec!['|', '?', '+', '*', '^'];
        let binary_operators = vec!['|', '^'];
        let mut i: usize = 0;
        let mut c1: char;
        let mut c2: char;
        let leng: usize = regex.len();
        while i < leng {
            c1 = regex.chars().nth(i).unwrap();

            if (i + 1) < leng {
                c2 = regex.chars().nth(i + 1).unwrap();
                res.push(c1);
                if !c1.eq(&'(')
                    && !c2.eq(&')')
                    && !all_operators.contains(&c2)
                    && !binary_operators.contains(&c1)
                {
                    res.push('.');
                }
            }
            i += 1;
        }

        res.push(regex.chars().nth(leng - 1).unwrap());
        res
    }
    pub(crate) fn inf_to_post(&mut self, s: &str) -> String {
        let mut postfix = String::new();
        let mut stack = Stack::<char>::new();
        let formatted_regex = self.format_regex(s);
        let mut c1: char = 'c';
        let mut next_precedence: i32 = 0;
        let mut c_precedence: i32 = 0;

        formatted_regex.chars().for_each(|c: char| match c {
            '(' => stack.push(c),
            ')' => {
                while !stack.is_empty() && !stack.peek().unwrap().eq(&'(') {
                    postfix.push(stack.pop().unwrap())
                }
                stack.pop();
            }
            c => {
                while stack.length() > 0 {
                    c1 = *stack.peek().unwrap();
                    next_precedence = self.get_precedence(c1);
                    c_precedence = self.get_precedence(c);

                    if next_precedence >= c_precedence {
                        postfix.push(stack.pop().unwrap());
                    } else {
                        break;
                    }
                }
                stack.push(c);
            }
        });

        while stack.length() > 0 {
            postfix.push(stack.pop().unwrap());
        }
        postfix
    }

    pub(crate) fn replace_arrays(&mut self, s: &str) -> String {
        // let mut state = 0;
        let mut vletter = Vec::<(char, char)>::new();
        let mut is_escaped = false;
        let mut in_array = false;
        let mut pair = ('a', 'a');
        let mut in_pair = false;
        let mut vchar: Vec<char>;
        let mut iterated = String::new();
        for c in s.chars() {
            if c == '\\' {
                is_escaped = true;
            } else if c == '[' && !is_escaped {
                in_array = true;
                iterated.push('(');
            } else if in_array && c == ']' {
                in_array = false;
                vchar = self.gen_list(&mut vletter);
                let mut i = 0;
                while i < vchar.len() {
                    iterated.push(vchar[i]);
                    if i != vchar.len() - 1 {
                        iterated.push('|')
                    }
                    i += 1;
                }
                iterated.push(')');
            } else if in_array && c == '-' {
                in_pair = true;
            } else if in_array && in_pair {
                pair.1 = c;
                vletter.push(pair);
                in_pair = false;
            } else if in_array {
                pair.0 = c;
            } else {
                iterated.push(c)
            }
        }
        iterated
    }
    fn gen_list(&mut self, l: &mut Vec<(char, char)>) -> Vec<char> {
        let mut vchar = Vec::<char>::new();
        let mut i: u32 = 0;
        l.iter().for_each(|(c1, c2): &(char, char)| {
            i = (*c1).into();
            while i <= (*c2).into() {
                vchar.push(i.try_into().unwrap());
                i += 1;
            }
        });
        vchar
    }
}
