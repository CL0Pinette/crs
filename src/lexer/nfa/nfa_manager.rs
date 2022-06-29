use crate::lexer::utils::stack::Stack;

use super::cell::{Cell, EPSILON};

pub(crate) struct NFAManager {
    _nfa_max: i32,
    nfa_states_arr: Vec<Box<Cell>>,
    nfa_stack: Stack<Box<Cell>>,
    next_alloc: i32,
    nfa_states: i32,
}

impl NFAManager {
    pub(crate) fn new() -> Self {
        let mut arr: Vec<Box<Cell>> = Vec::new();
        for _ in 0..256 {
            arr.push(Box::new(Cell::new()));
        }
        Self {
            _nfa_max: 256,
            nfa_states_arr: arr,
            nfa_stack: Stack::new(),
            next_alloc: 0,
            nfa_states: 0,
        }
    }

    pub(crate) fn new_nfa(&mut self) -> Cell {
        let mut nfa: Cell;
        if self.nfa_stack.length() > 0 {
            match self.nfa_stack.pop() {
                Some(a) => nfa = *a,
                _ => unreachable!(),
            }
            nfa.clear_state();
            nfa.set_state(self.nfa_states);
            self.nfa_states += 1;
            nfa.set_edge(EPSILON);
        } else {
            // println!("{}", self.nfa_states_arr.len());
            // println!("{}", self.next_alloc);
            nfa = match self.nfa_states_arr.iter().nth(self.next_alloc as usize) {
                Some(cell) => *cell.clone(),
                _ => panic!("error nfa_arr"),
            };
            self.next_alloc += 1;
            nfa.clear_state();
            nfa.set_state(self.nfa_states);
            self.nfa_states += 1;
            nfa.set_edge(EPSILON);
        }

        nfa
    }

    pub(crate) fn _discard_nfa(&mut self, mut nfa_discarded: Cell) {
        self.nfa_states -= 1;
        nfa_discarded.clear_state();
        self.nfa_stack.push(Box::new(nfa_discarded));
    }
}
