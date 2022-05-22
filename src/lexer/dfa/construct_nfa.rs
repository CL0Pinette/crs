use super::cell::Cell;
use super::{super::utils::stack::Stack, cell::EPSILON};

pub(crate) struct NFAManager<'a> {
    nfa_max: i32,
    //nfa_states_arr: Vec<&'a Cell<'a>>,
    nfa_stack: Stack<&'a mut Cell<'a>>,
    next_alloc: i32,
    nfa_states: i32,
    no_cell: Cell<'a>,
}

impl<'a> NFAManager<'a> {
    pub(crate) fn new() -> Self {
        Self {
            nfa_max: 256,
            nfa_stack: Stack::new(),
            next_alloc: 0,
            nfa_states: 0,
            no_cell: Cell::new(),
        }
    }

    pub(crate) fn new_nfa(&mut self) -> &mut Cell<'a> {
        let mut nfa: &mut Cell<'a> = &mut self.no_cell;
        if self.nfa_stack.length() > 0 {
            match self.nfa_stack.pop() {
                Some(a) => nfa = a,
                _ => (),
            }
        }
        nfa.clear_state();
        self.nfa_states += 1;
        nfa.set_state(self.nfa_states);
        nfa.set_edge(EPSILON);
        nfa
    }

    pub(crate) fn discard_nfa(&'a mut self, nfa_discarded: &'a mut Cell<'a>) {
        self.nfa_states -= 1;
        nfa_discarded.clear_state();
        self.nfa_stack.push(nfa_discarded);
    }
}

pub(crate) struct Pair<'a> {
    start: Option<&'a mut Cell<'a>>,
    end: Option<&'a mut Cell<'a>>,
}

pub(crate) struct NFAConstructor<'a> {
    nfa_manager: NFAManager<'a>,
}

impl<'a> NFAConstructor<'a> {
    pub(crate) fn new() -> Self {
        Self {
            nfa_manager: NFAManager::new(),
        }
    }
    pub(crate) fn construct_star_closure(&'a mut self, pair_in: &'a mut Pair<'a>) -> Pair {
        let mut pair_out = Pair {
            start: None,
            end: None,
        };
        let ref mut man = self.nfa_manager;
        let a = self.nfa_manager.new_nfa();
        pair_out.start = Some(a);
        let a = man.new_nfa();
        pair_out.end = Some(a);

        pair_out.start.unwrap().next = Some(pair_in.start.unwrap());
        pair_in.end.unwrap().next = Some(pair_out.end.unwrap());

        pair_out.start.unwrap().next2 = pair_out.end;
        pair_in.end.unwrap().next2 = pair_in.start;

        pair_in.start = pair_out.start;
        pair_in.end = pair_out.end;

        pair_out
    }
}
