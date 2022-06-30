use std::fmt;

#[derive(Clone)]
pub(crate) struct Cell {
    pub(super) edge: i32,
    pub next: Option<Box<Cell>>,
    pub next2: Option<Box<Cell>>,
    pub(super) state: i32,
    pub(super) visited: bool,
}

pub(crate) const EPSILON: i32 = -1;
pub(crate) const EMPTY: i32 = -2;

impl Cell {
    pub(crate) fn new() -> Self {
        Cell {
            edge: EPSILON,
            next: None,
            next2: None,
            state: EMPTY,
            visited: false,
        }
    }

    pub(crate) fn set_edge(&mut self, cell_type: i32) {
        self.edge = cell_type;
    }
    pub(crate) fn set_visited(&mut self) {
        self.visited = true;
    }
    pub(crate) fn set_unvisited(&mut self) {
        self.visited = false;
    }
    pub(crate) fn is_visited(&self) -> bool {
        self.visited
    }
    pub(crate) fn get_state(&self) -> i32 {
        self.state
    }
    pub(crate) fn set_state(&mut self, num: i32) {
        self.state = num;
    }
    pub(crate) fn clear_state(&mut self) {
        self.next = None;
        self.next2 = None;
        self.state = -1;
    }

    pub(crate) fn _print(&self) {
        println!("{}", self);
        match self.clone().next {
            Some(c1) => c1._print(),
            None => {}
        };
        match self.clone().next2 {
            Some(c1) => c1._print(),
            None => {}
        }
    }
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let a: String;
        let b: String;
        match self.next.as_ref() {
            Some(c) => {
                a = format!(
                    "edge: {}, state: {}, is_visited: {}",
                    c.edge, c.state, c.visited
                )
            }
            None => a = "None".to_string(),
        };
        match self.next2.as_ref() {
            Some(c) => {
                b = format!(
                    "edge: {}, state: {}, is_visited: {}",
                    c.edge, c.state, c.visited
                )
            }
            None => b = "None".to_string(),
        };
        write!(
            f,
            "edge: {}, state: {}, is_visited: {}, next: [{}], next2: [{}]",
            self.edge, self.state, self.visited, a, b
        )
    }
}
