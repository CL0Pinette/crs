use std::fmt;

pub(crate) struct Cell<'a> {
    pub(super) edge: i32,
    pub next: Option<&'a mut Cell<'a>>,
    pub next2: Option<&'a mut Cell<'a>>,
    pub(super) state: i32,
    pub(super) visited: bool,
}

pub(crate) const EPSILON: i32 = -1;
pub(crate) const EMPTY: i32 = -2;

impl Cell<'_> {
    pub(crate) fn new() -> Self {
        Cell {
            edge: EPSILON,
            next: None,
            next2: None,
            state: EMPTY,
            visited: false,
        }
    }

    pub(crate) fn get_edge(&mut self) -> i32 {
        self.edge
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
    pub(crate) fn is_visited(&mut self) -> bool {
        self.visited
    }
    pub(crate) fn get_state(&mut self) -> i32 {
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
}

impl fmt::Display for Cell<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}{}", self.edge, self.state, self.visited)
    }
}
