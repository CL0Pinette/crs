use crate::lexer::utils::{
    console_table::{Obj, Table},
    stack::Stack,
};

use super::{
    cell::{Cell, EPSILON},
    nfa_constructor::NFAConstructor,
    nfa_manager::NFAManager,
};

pub(crate) struct NFA {
    pair: (Option<Box<Cell>>, Option<Box<Cell>>),
    restate: i32,
    table: Table,
    re: String,
    letter: Vec<String>,
}

impl NFA {
    pub(crate) fn new(s: &str) -> Self {
        let mut a = Vec::new();
        for c in s.chars() {
            if c.is_alphabetic() && !a.contains(&c) {
                a.push(c);
            }
        }
        let mut b: Vec<String> = Vec::new();
        b.push("".to_string());
        for c in &a {
            b.push(c.to_string());
        }
        b.push("EPSILON".to_string());
        for c in &b {
            println!("{}", c);
        }
        NFA {
            pair: (None, None),
            restate: 0,
            table: Table::new(b.len() as i32, true),
            re: s.to_string(),
            letter: b,
        }
    }

    pub(crate) fn populate(&mut self) {
        self.table.append_row();
        for l in self.letter.clone() {
            self.table.append_column(Obj::S(l));
        }
    }

    pub(crate) fn re2nfa(&mut self) {
        let mut cons = NFAConstructor {
            man: NFAManager::new(),
        };
        println!("{}", self.re);
        let mut stack: Stack<(Option<Box<Cell>>, Option<Box<Cell>>)> = Stack::new();

        self.re.chars().for_each(|c| {
            if stack.length() > 0 {
                println!("Start");
                println!("{}", stack.peek().unwrap().0.as_ref().unwrap());
                println!("{}", stack.peek().unwrap().1.as_ref().unwrap());
                println!("End");
            }
            match c {
                '|' => match stack.pop() {
                    Some(right) => match stack.pop() {
                        Some(left) => {
                            self.pair = cons.construct_or_closure(left, right);
                            stack.push(self.pair.clone())
                        }
                        _ => panic!("Bad regex {}", self.re),
                    },
                    _ => panic!("Bad Regex {}", self.re),
                },
                '*' => match stack.pop() {
                    Some(temp) => {
                        self.pair = cons.construct_star_closure(temp);
                        stack.push(self.pair.clone())
                    }
                    _ => panic!("Bad regex {}", self.re),
                },
                '+' => match stack.pop() {
                    Some(temp) => {
                        self.pair = cons.construct_plus_closure(temp);
                        stack.push(self.pair.clone())
                    }
                    _ => panic!("Bad regex {}", self.re),
                },
                '.' => match stack.pop() {
                    Some(right) => match stack.pop() {
                        Some(left) => {
                            self.pair = cons.construct_connector(left, right);
                            stack.push(self.pair.clone())
                        }
                        _ => panic!("Bad regex {}", self.re),
                    },
                    _ => panic!("Bad Regex {}", self.re),
                },
                c => {
                    self.pair = cons.construct_single_char(c);
                    stack.push(self.pair.clone());
                }
            }
        })
    }

    pub(crate) fn print(&mut self) {
        println!("Start Print");
        println!("{}", self.pair.0.as_ref().unwrap());
        println!("{}", self.pair.1.as_ref().unwrap());
        println!("End");
        let mut pair = self.pair.clone();
        pair.0 = self.restate(self.pair.0.clone());
        pair.0 = self.revisit(pair.0.clone());
        println!("{}", pair.0.clone().unwrap().get_state());
        println!("--------NFA--------");
        self.table.append_row();
        pair.0 = self.print_nfa(pair.0.clone());
        println!("{}", self.table.string());
        println!("bite");
        self.revisit(pair.0.clone());
        println!("--------NFA--------");
        println!("start state: {}", pair.0.clone().unwrap().get_state());
        println!("end state: {}", pair.1.clone().unwrap().get_state());
    }
    pub(crate) fn restate(&mut self, c: Option<Box<Cell>>) -> Option<Box<Cell>> {
        match c {
            Some(mut c1) => {
                if c1.is_visited() {
                    return None;
                }
                c1.set_visited();
                c1.set_state(self.restate);
                self.restate += 1;
                println!("{}", c1.clone().get_state());
                c1.next = self.restate(c1.clone().next);
                c1.next2 = self.restate(c1.clone().next2);
                Some(c1)
            }
            None => None,
        }
    }
    pub(crate) fn revisit(&mut self, c: Option<Box<Cell>>) -> Option<Box<Cell>> {
        match c {
            Some(mut c1) => {
                if !c1.is_visited() {
                    return None;
                }
                c1.set_unvisited();
                c1.next = self.revisit(c1.clone().next);
                c1.next2 = self.revisit(c1.clone().next2);
                Some(c1)
            }
            None => None,
        }
    }
    pub(crate) fn print_nfa(&mut self, c: Option<Box<Cell>>) -> Option<Box<Cell>> {
        match c {
            Some(mut c1) => {
                if c1.is_visited() {
                    return None;
                }
                c1.set_visited();
                self.print_nfanode(Some(c1.clone()));
                match c1.next {
                    Some(_) => self.table.append_row(),
                    None => {}
                };
                self.print_nfa(c1.clone().next);
                self.print_nfa(c1.clone().next2);
                Some(c1)
            }
            None => None,
        }
    }
    pub(crate) fn print_nfanode(&mut self, c: Option<Box<Cell>>) {
        println!("c {}", c.clone().unwrap());
        match c {
            Some(mut c4) => {
                println!("c4 {}", c4.clone());
                match c4.clone().next {
                    Some(c1) => {
                        println!("c1 {}", c1.clone());
                        self.table.append_column(Obj::I(c4.get_state()));
                        if c4.get_edge() == EPSILON {
                            for _ in 0..(self.letter.len() - 2) as i32 {
                                self.table.append_column(Obj::S(" ".to_string()));
                            }
                            match c4.clone().next2.clone() {
                                Some(_c3) => self.table.append_column(Obj::S(format!(
                                    "{{{},{}}}",
                                    c4.clone().next.clone().unwrap().get_state(),
                                    c4.clone().next2.clone().unwrap().get_state()
                                ))),
                                None => self.table.append_column(Obj::S(format!(
                                    "{{{}}}",
                                    c4.clone().next.clone().unwrap().get_state()
                                ))),
                            };
                        } else {
                            let index: i32 =
                                self.get_index((c4.get_edge() as u8 as char).to_string());
                            for f in 0..(self.letter.len() - 1) as i32 {
                                if f != index {
                                    self.table.append_column(Obj::S(" ".to_string()));
                                } else {
                                    match c4.clone().next2.clone() {
                                        Some(_c3) => self.table.append_column(Obj::S(format!(
                                            "{{{},{}}}",
                                            c4.clone().next.clone().unwrap().get_state(),
                                            c4.clone().next2.clone().unwrap().get_state()
                                        ))),
                                        None => self.table.append_column(Obj::S(format!(
                                            "{{{}}}",
                                            c4.clone().next.clone().unwrap().get_state()
                                        ))),
                                    };
                                }
                            }
                        }
                    }
                    None => {
                        self.table.append_column(Obj::I(c4.get_state()));
                        self.table.append_column(Obj::S(" ".to_string()));
                        self.table.append_column(Obj::S(" ".to_string()));
                        self.table.append_row();
                    }
                }
            }
            None => {}
        }
    }

    fn get_index(&self, s: String) -> i32 {
        println!("[String] {}", s);
        for i in 0..self.letter.len() - 1 {
            if self.letter.iter().nth(i).eq(&Some(&s)) {
                println!("{}", i);
                return i as i32 - 1;
            }
        }
        return -1;
    }
}
