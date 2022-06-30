use dict::{Dict, DictIface};

use crate::lexer::utils::{
    console_table::{Obj, Table},
    stack::Stack,
};

use super::{
    cell::{Cell, EPSILON},
    cell_array::CellArray,
    nfa_constructor::NFAConstructor,
    nfa_manager::NFAManager,
};

pub(crate) struct NFA {
    pair: (Option<Box<Cell>>, Option<Box<Cell>>),
    restate: i32,
    table: Table,
    re: String,
    letter: Vec<String>,
    all_nodes: Vec<CellArray>,
    indexes: Vec<i32>,
    count: i32,
    map: Dict<i32>,
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
        // for c in &b {
        //     println!("{}", c);
        // }
        NFA {
            pair: (None, None),
            restate: 0,
            table: Table::new(b.len() as i32, true),
            re: s.to_string(),
            letter: b,
            all_nodes: Vec::<CellArray>::new(),
            count: 0,
            indexes: Vec::new(),
            map: Dict::new(),
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
        // println!("{}", self.re);
        let mut stack: Stack<(Option<Box<Cell>>, Option<Box<Cell>>)> = Stack::new();

        self.re.chars().for_each(|c| {
            // if stack.length() > 0 {
            //     println!("Start");
            //     println!("{}", stack.peek().unwrap().0.as_ref().unwrap());
            //     println!("{}", stack.peek().unwrap().1.as_ref().unwrap());
            //     println!("End");
            // }
            match c {
                '|' => match stack.pop() {
                    Some(right) => match stack.pop() {
                        Some(left) => {
                            self.pair = cons.construct_or_closure(left, right);
                            // println!("OR");
                            // self.pair.clone().0.unwrap().print();
                            // println!("--");
                            // self.pair.clone().1.unwrap().print();
                            stack.push(self.pair.clone())
                        }
                        _ => panic!("Bad regex {}", self.re),
                    },
                    _ => panic!("Bad Regex {}", self.re),
                },
                '*' => match stack.pop() {
                    Some(temp) => {
                        self.pair = cons.construct_star_closure(temp);
                        // println!("STAR");
                        // self.pair.clone().0.unwrap().print();
                        // println!("--");
                        // self.pair.clone().1.unwrap().print();
                        stack.push(self.pair.clone())
                    }
                    _ => panic!("Bad regex {}", self.re),
                },
                '+' => match stack.pop() {
                    Some(temp) => {
                        self.pair = cons.construct_plus_closure(temp);
                        // println!("PLUS");
                        // self.pair.clone().0.unwrap().print();
                        // println!("--");
                        // self.pair.clone().1.unwrap().print();
                        stack.push(self.pair.clone())
                    }
                    _ => panic!("Bad regex {}", self.re),
                },
                '.' => match stack.pop() {
                    Some(right) => match stack.pop() {
                        Some(left) => {
                            self.pair = cons.construct_connector(left, right);
                            // println!("DOT");
                            // self.pair.clone().0.unwrap().print();
                            // println!("--");
                            // self.pair.clone().1.unwrap().print();
                            stack.push(self.pair.clone())
                        }
                        _ => panic!("Bad regex {}", self.re),
                    },
                    _ => panic!("Bad Regex {}", self.re),
                },
                c => {
                    self.pair = cons.construct_single_char(c);
                    // println!("{}", c);
                    // self.pair.clone().0.unwrap().print();
                    // println!("--");
                    // self.pair.clone().1.unwrap().print();
                    stack.push(self.pair.clone());
                }
            }
        })
    }

    pub(crate) fn print(&mut self) {
        // println!("Start Print");
        // println!("{}", self.pair.0.as_ref().unwrap());
        // println!("{}", self.pair.1.as_ref().unwrap());
        // println!("End");
        let pair = self.pair.clone();
        self.prepare_nfa(pair.0.clone());
        // println!("{}", self.all_nodes.len());
        // println!("---------------------------------------------------------------------------");
        // for i in 0..self.all_nodes.len() {
        //     self.map
        //         .add(i.to_string(), self.all_nodes[i as usize].state);
        //     println!("{}", self.all_nodes[i as usize].state);
        // }
        // println!("{}", pair.0.clone().unwrap().get_state());
        self.revisit();
        self.restate();
        // println!("---------------------------------------------------------------------------");
        // for c in self.all_nodes.clone() {
        //     println!("{}", c);
        // }
        println!("--------NFA--------");
        self.table.append_row();
        self.revisit();
        let mut node = self.all_nodes[0].clone();
        self.print_nfa(&mut node);
        println!("{}", self.table.string());
        self.revisit();
        println!("--------NFA--------");
        println!("start state: {}", self.all_nodes[0].get_state());
        println!(
            "end state: {}",
            self.all_nodes[self.all_nodes.len() - 1].get_state()
        );
    }

    pub fn prepare_nfa(&mut self, c: Option<Box<Cell>>) {
        match c {
            Some(c1) => {
                if self.indexes.contains(&c1.get_state()) {
                    return;
                }
                self.indexes.push(c1.get_state());
                // c1.set_state(self.count);
                // self.count += 1;
                let c = CellArray {
                    edge: c1.edge,
                    next: match c1.clone().next {
                        Some(a) => a.get_state(),
                        None => -1,
                    },
                    next2: match c1.clone().next2 {
                        Some(a) => a.get_state(),
                        None => -1,
                    },
                    state: c1.state,
                    visited: c1.visited,
                    setup: false,
                };
                self.all_nodes.push(c);
                self.prepare_nfa(c1.clone().next);
                self.prepare_nfa(c1.clone().next2);
            }
            None => {}
        };
        // let copy = self.all_nodes.clone();
        // for c in copy {}
    }

    pub(crate) fn restate(&mut self) {
        // println!("Start Print");
        // println!("{}", c1.as_ref());
        // println!("{}", c1.as_ref());
        // println!("End");
        // println!("{}", c1.clone());

        // println!("---------------------------------------------------------------------------");
        // for c in self.all_nodes.clone() {
        //     println!("{}", c);
        // }

        for i in 0..self.all_nodes.len() {
            // println!("{{BEFORE IF}}{}", self.all_nodes[i as usize].is_visited());
            if self.all_nodes[i as usize].is_visited() {
                return;
            }
            self.all_nodes[i as usize].visited = true;
            // println!(
            //     "[RESTATE-BEFORE]{}, {}",
            //     self.all_nodes[i as usize].clone().get_state(),
            //     self.all_nodes[i as usize].clone().is_visited()
            // );

            let state = self.all_nodes[i as usize].get_state();
            let new_state = self.restate;
            self.restate += 1;

            self.map.add(state.to_string(), new_state);

            self.all_nodes[i as usize].state = new_state;
            self.all_nodes[i as usize].setup = true;

            // println!(
            //     "[RESTATE]{}, {}, {}, {}",
            //     self.all_nodes[i as usize].clone().get_state(),
            //     new_state,
            //     self.all_nodes[i as usize].next,
            //     self.all_nodes[i as usize].next2
            // );
        }

        for i in 0..self.all_nodes.len() {
            match self.all_nodes[i as usize].clone().next {
                -1 => {}
                j => {
                    self.all_nodes[i as usize].next = *self.map.get(&j.to_string()).unwrap();
                }
            };
            match self.all_nodes[i as usize].clone().next2 {
                -1 => {}
                j => {
                    self.all_nodes[i as usize].next2 = *self.map.get(&j.to_string()).unwrap();
                }
            }
        }
        // Some(c1.clone())
    }
    pub(crate) fn revisit(&mut self) {
        for i in 0..self.all_nodes.len() {
            self.all_nodes[i as usize].visited = false;
        }
    }
    pub(crate) fn print_nfa(&mut self, c: &mut CellArray) {
        if c.is_visited() {
            return;
        }
        // println!("{}", c);
        self.all_nodes[c.state as usize].visited = true;
        c.visited = true;
        self.print_nfanode(c.clone());
        match c.next {
            -1 => {}
            _ => self.table.append_row(),
        };
        match c.next {
            -1 => {}
            i => {
                let mut node = self.all_nodes.iter().nth(i as usize).unwrap().clone();
                self.print_nfa(&mut node);
            }
        };
        match c.next2 {
            -1 => {}
            i => {
                let mut node = self.all_nodes.iter().nth(i as usize).unwrap().clone();
                self.print_nfa(&mut node);
            }
        };
    }
    pub(crate) fn print_nfanode(&mut self, c: CellArray) {
        match c.clone().next {
            -1 => {
                self.table.append_column(Obj::I(self.count));
                self.count += 1;
                self.table.append_column(Obj::S(" ".to_string()));
                self.table.append_column(Obj::S(" ".to_string()));
                self.table.append_row();
            }
            _i => {
                // println!("c1 {}", c1.clone());
                self.table.append_column(Obj::I(self.count));
                self.count += 1;
                if c.get_edge() == EPSILON {
                    for _ in 0..(self.letter.len() - 2) as i32 {
                        self.table.append_column(Obj::S(" ".to_string()));
                    }
                    match c.clone().next2.clone() {
                        -1 => self
                            .table
                            .append_column(Obj::S(format!("{{{}}}", c.clone().next.clone()))),
                        _j => self.table.append_column(Obj::S(format!(
                            "{{{},{}}}",
                            c.clone().next.clone(),
                            c.clone().next2.clone()
                        ))),
                    };
                } else {
                    let index: i32 = self.get_index((c.get_edge() as u8 as char).to_string());
                    for f in 0..(self.letter.len() - 1) as i32 {
                        if f != index {
                            self.table.append_column(Obj::S(" ".to_string()));
                        } else {
                            match c.clone().next2.clone() {
                                -1 => self.table.append_column(Obj::S(format!(
                                    "{{{}}}",
                                    c.clone().next.clone()
                                ))),
                                _j => self.table.append_column(Obj::S(format!(
                                    "{{{},{}}}",
                                    c.clone().next.clone(),
                                    c.clone().next2.clone()
                                ))),
                            };
                        }
                    }
                }
            }
        }
    }

    fn get_index(&self, s: String) -> i32 {
        for i in 0..self.letter.len() - 1 {
            if self.letter.iter().nth(i).eq(&Some(&s)) {
                return i as i32 - 1;
            }
        }
        return -1;
    }
}
