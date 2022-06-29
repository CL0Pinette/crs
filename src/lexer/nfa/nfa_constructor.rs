use super::{cell::Cell, nfa_manager::NFAManager};

pub(crate) struct NFAConstructor {
    pub(crate) man: NFAManager,
}
impl NFAConstructor {
    pub(crate) fn construct_star_closure(
        &mut self,
        mut pair_in: (Option<Box<Cell>>, Option<Box<Cell>>),
    ) -> (Option<Box<Cell>>, Option<Box<Cell>>) {
        let mut pair_out: (Option<Box<Cell>>, Option<Box<Cell>>) = (
            Some(Box::new(self.man.new_nfa())),
            Some(Box::new(self.man.new_nfa())),
        );

        match pair_in.0.clone() {
            Some(a) => pair_out.0.as_mut().unwrap().next = Some(a),
            _ => pair_out.0.as_mut().unwrap().next = None,
        }

        match pair_out.1.clone() {
            Some(b) => pair_in.1.as_mut().unwrap().next = Some(b),
            _ => pair_in.1.as_mut().unwrap().next = None,
        }

        match pair_out.1.clone() {
            Some(a) => pair_out.0.as_mut().unwrap().next2 = Some(a),
            _ => pair_out.0.as_mut().unwrap().next2 = None,
        }
        match pair_in.0.clone() {
            Some(a) => pair_in.1.as_mut().unwrap().next2 = Some(a),
            _ => pair_in.1.as_mut().unwrap().next2 = None,
        }

        match pair_out.0.clone() {
            Some(a) => pair_in.0 = Some(a),
            _ => pair_in.0 = None,
        }
        match pair_out.1.clone() {
            Some(a) => pair_in.1 = Some(a),
            _ => pair_in.1 = None,
        }

        pair_out
    }

    pub(crate) fn construct_plus_closure(
        &mut self,
        mut pair_in: (Option<Box<Cell>>, Option<Box<Cell>>),
    ) -> (Option<Box<Cell>>, Option<Box<Cell>>) {
        let mut pair_out: (Option<Box<Cell>>, Option<Box<Cell>>) = (
            Some(Box::new(self.man.new_nfa())),
            Some(Box::new(self.man.new_nfa())),
        );

        match pair_in.0.clone() {
            Some(a) => pair_out.0.as_mut().unwrap().next = Some(a),
            _ => pair_out.0.as_mut().unwrap().next = None,
        }

        match pair_out.1.clone() {
            Some(b) => pair_in.1.as_mut().unwrap().next = Some(b),
            _ => pair_in.1.as_mut().unwrap().next = None,
        }

        match pair_out.0.clone() {
            Some(a) => pair_in.1.as_mut().unwrap().next2 = Some(a),
            _ => pair_in.1.as_mut().unwrap().next2 = None,
        }

        match pair_out.0.clone() {
            Some(a) => pair_in.0 = Some(a),
            _ => pair_in.0 = None,
        }
        match pair_out.1.clone() {
            Some(a) => pair_in.1 = Some(a),
            _ => pair_in.1 = None,
        }

        pair_out
    }

    pub(crate) fn construct_single_char(
        &mut self,
        c: char,
    ) -> (Option<Box<Cell>>, Option<Box<Cell>>) {
        let mut pair_out: (Option<Box<Cell>>, Option<Box<Cell>>) = (
            Some(Box::new(self.man.new_nfa())),
            Some(Box::new(self.man.new_nfa())),
        );

        match pair_out.1.clone() {
            Some(a) => pair_out.0.as_deref_mut().unwrap().next = Some(a),
            _ => pair_out.0.as_mut().unwrap().next = None,
        }

        pair_out.0.as_deref_mut().unwrap().set_edge(c as i32);

        pair_out
    }

    pub(crate) fn construct_or_closure(
        &mut self,
        mut left: (Option<Box<Cell>>, Option<Box<Cell>>),
        mut right: (Option<Box<Cell>>, Option<Box<Cell>>),
    ) -> (Option<Box<Cell>>, Option<Box<Cell>>) {
        let mut pair_out: (Option<Box<Cell>>, Option<Box<Cell>>) = (
            Some(Box::new(self.man.new_nfa())),
            Some(Box::new(self.man.new_nfa())),
        );

        match left.0.clone() {
            Some(a) => pair_out.0.as_mut().unwrap().next = Some(a),
            _ => pair_out.1.as_mut().unwrap().next = None,
        }

        match right.0.clone() {
            Some(a) => pair_out.0.as_mut().unwrap().next2 = Some(a),
            _ => pair_out.0.as_mut().unwrap().next2 = None,
        }

        match pair_out.1.clone() {
            Some(b) => left.1.as_mut().unwrap().next = Some(b),
            _ => left.1.as_mut().unwrap().next = None,
        }

        match pair_out.1.clone() {
            Some(b) => right.1.as_mut().unwrap().next = Some(b),
            _ => right.1.as_mut().unwrap().next = None,
        }

        pair_out
    }

    pub(crate) fn construct_connector(
        &mut self,
        mut left: (Option<Box<Cell>>, Option<Box<Cell>>),
        right: (Option<Box<Cell>>, Option<Box<Cell>>),
    ) -> (Option<Box<Cell>>, Option<Box<Cell>>) {
        let mut pair_out: (Option<Box<Cell>>, Option<Box<Cell>>) = (None, None);

        match left.0.clone() {
            Some(a) => pair_out.0 = Some(a),
            _ => pair_out.0 = None,
        }

        match right.1.clone() {
            Some(a) => pair_out.1 = Some(a),
            _ => pair_out.1 = None,
        }

        match right.0.clone() {
            Some(b) => left.1.as_mut().unwrap().next = Some(b),
            _ => left.1.as_mut().unwrap().next = None,
        }

        pair_out
    }
}
