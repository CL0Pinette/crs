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
            Some(b) => {
                pair_in.1.as_mut().unwrap().next = Some(b);
                pair_out.0 = self.unvisit_all(pair_out.0);
                pair_out.0 = update_cell(&mut pair_out.0, &pair_in.1)
            }
            _ => {
                pair_in.1.as_mut().unwrap().next = None;
                pair_out.0 = self.unvisit_all(pair_out.0);
                pair_out.0 = update_cell(&mut pair_out.0, &pair_in.1)
            }
        }

        match pair_out.1.clone() {
            Some(a) => pair_out.0.as_mut().unwrap().next2 = Some(a),
            _ => pair_out.0.as_mut().unwrap().next2 = None,
        }

        match pair_in.0.clone() {
            Some(a) => {
                pair_in.1.as_mut().unwrap().next2 = Some(a);
                pair_out.0 = self.unvisit_all(pair_out.0);
                pair_out.0 = update_cell(&mut pair_out.0, &pair_in.1)
            }
            _ => {
                pair_in.1.as_mut().unwrap().next2 = None;
                pair_out.0 = self.unvisit_all(pair_out.0);
                pair_out.0 = update_cell(&mut pair_out.0, &pair_in.1)
            }
        }

        match pair_out.0.clone() {
            Some(a) => {
                pair_in.0 = Some(a);
                // update_cell(&mut pair_out.0, &pair_in.0)
            }
            _ => {
                pair_in.0 = None;
                // update_cell(&mut pair_out.0, &pair_in.0)
            }
        }
        match pair_out.1.clone() {
            Some(a) => {
                pair_in.1 = Some(a);
                pair_out.0 = self.unvisit_all(pair_out.0);
                pair_out.0 = update_cell(&mut pair_out.0, &pair_in.1)
            }
            _ => {
                pair_in.1 = None;
                pair_out.0 = self.unvisit_all(pair_out.0);
                pair_out.0 = update_cell(&mut pair_out.0, &pair_in.1)
            }
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
            Some(b) => {
                pair_in.1.as_mut().unwrap().next = Some(b);
                pair_out.0 = self.unvisit_all(pair_out.0);
                pair_out.0 = update_cell(&mut pair_out.0, &pair_in.1)
            }
            _ => {
                pair_in.1.as_mut().unwrap().next = None;
                pair_out.0 = self.unvisit_all(pair_out.0);
                pair_out.0 = update_cell(&mut pair_out.0, &pair_in.1)
            }
        }

        match pair_out.0.clone() {
            Some(a) => {
                pair_in.1.as_mut().unwrap().next2 = Some(a);
                pair_out.0 = self.unvisit_all(pair_out.0);
                pair_out.0 = update_cell(&mut pair_out.0, &pair_in.1)
            }
            _ => {
                pair_in.1.as_mut().unwrap().next2 = None;
                pair_out.0 = self.unvisit_all(pair_out.0);
                pair_out.0 = update_cell(&mut pair_out.0, &pair_in.1)
            }
        }

        match pair_out.0.clone() {
            Some(a) => {
                pair_in.0 = Some(a);
            }
            _ => {
                pair_in.0 = None;
            }
        }
        match pair_out.1.clone() {
            Some(a) => {
                pair_in.1 = Some(a);
                pair_out.0 = self.unvisit_all(pair_out.0);
                pair_out.0 = update_cell(&mut pair_out.0, &pair_in.1)
            }
            _ => {
                pair_in.1 = None;
                pair_out.0 = self.unvisit_all(pair_out.0);
                pair_out.0 = update_cell(&mut pair_out.0, &pair_in.1)
            }
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
            Some(a) => pair_out.0.as_mut().unwrap().next = Some(a),
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
            Some(b) => {
                left.1.as_mut().unwrap().next = Some(b);
                pair_out.0 = self.unvisit_all(pair_out.0);
                pair_out.0 = update_cell(&mut pair_out.0, &left.1);
                // println!("----------------------------------");
                // pair_out.clone().0.unwrap().print();
                // println!("----------------------------------");
            }
            _ => left.1.as_mut().unwrap().next = None,
        }

        match pair_out.1.clone() {
            Some(b) => {
                right.1.as_mut().unwrap().next = Some(b);
                pair_out.0 = self.unvisit_all(pair_out.0);
                pair_out.0 = update_cell(&mut pair_out.0, &right.1);
                // println!("----------------------------------");
                // pair_out.clone().0.unwrap().print();
                // println!("----------------------------------");
            }
            _ => {
                right.1.as_mut().unwrap().next = None;
                pair_out.0 = self.unvisit_all(pair_out.0);
                pair_out.0 = update_cell(&mut pair_out.0, &right.1)
            }
        };

        // pair_out
        //     .clone()
        //     .0
        //     .unwrap()
        //     .next
        //     .unwrap()
        //     .next
        //     .unwrap()
        //     .print();

        pair_out
    }

    pub(crate) fn construct_connector(
        &mut self,
        mut left: (Option<Box<Cell>>, Option<Box<Cell>>),
        right: (Option<Box<Cell>>, Option<Box<Cell>>),
    ) -> (Option<Box<Cell>>, Option<Box<Cell>>) {
        let mut pair_out: (Option<Box<Cell>>, Option<Box<Cell>>) = (None, None);

        // println!("keeeeeeeeeeeeeeeeek");
        // println!("{}", left.clone().0.unwrap());
        // println!("{}", left.clone().1.unwrap());

        match left.0.clone() {
            Some(a) => {
                pair_out.0 = Some(a);
                pair_out.0 = self.unvisit_all(pair_out.0);
                pair_out.0 = update_cell(&mut pair_out.0, &left.0)
            }
            _ => pair_out.0 = None,
        }

        // println!("{}", pair_out.clone().0.unwrap());
        // println!("{}", pair_out.clone().0.unwrap().next.clone().unwrap());

        match right.1.clone() {
            Some(a) => {
                pair_out.1 = Some(a);
                pair_out.0 = self.unvisit_all(pair_out.0);
                pair_out.0 = update_cell(&mut pair_out.0, &pair_out.1)
            }
            _ => pair_out.1 = None,
        }

        match right.0.clone() {
            Some(b) => {
                left.1.as_mut().unwrap().next = Some(b);
                pair_out.0 = self.unvisit_all(pair_out.0);
                pair_out.0 = update_cell(&mut pair_out.0, &left.1)
                // Some(left.clone().0.unwrap().next.clone().unwrap());
            }
            _ => {
                left.1.as_mut().unwrap().next = None;
                pair_out.0 = self.unvisit_all(pair_out.0);
                pair_out.0 = update_cell(&mut pair_out.0, &left.1)
                // Some(left.clone().0.unwrap().next.clone().unwrap());
            }
        }

        // println!("kaaaaaaaaaaaaaaak");
        // println!("{}", pair_out.clone().0.unwrap().next2.clone().unwrap());
        // println!("{}", pair_out.clone().0.unwrap());
        // println!("{}", pair_out.clone().1.unwrap());

        pair_out
    }

    pub(crate) fn unvisit_all(&mut self, c: Option<Box<Cell>>) -> Option<Box<Cell>> {
        match c {
            Some(mut c1) => {
                if !c1.is_visited() {
                    return Some(c1);
                }
                c1.set_unvisited();
                c1.next = self.unvisit_all(c1.clone().next);
                c1.next2 = self.unvisit_all(c1.clone().next2);
                Some(c1)
            }
            None => None,
        }
    }
}

fn update_cell(c1: &mut Option<Box<Cell>>, c2: &Option<Box<Cell>>) -> Option<Box<Cell>> {
    if c1.clone().unwrap().is_visited() {
        return c1.clone();
    };
    match c1.clone() {
        Some(c) => {
            match c.clone().next.clone() {
                Some(next_node) => {
                    if next_node.clone().get_state() == c2.clone().unwrap().get_state() {
                        c1.as_deref_mut().unwrap().next = c2.clone();
                    } else {
                        c1.as_mut().unwrap().next = update_cell(&mut c1.as_mut().unwrap().next, c2);
                    }
                }
                None => {}
            };
            match c.clone().next2.clone() {
                Some(next_node) => {
                    if next_node.clone().get_state() == c2.clone().unwrap().get_state() {
                        c1.as_deref_mut().unwrap().next2 = c2.clone();
                    } else {
                        c1.as_mut().unwrap().next2 =
                            update_cell(&mut c1.as_mut().unwrap().next2, c2);
                    }
                }
                None => {}
            };
            c1.as_mut().unwrap().set_visited();
            c1.clone()
        }

        None => None,
    }
}
