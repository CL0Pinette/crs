use std::fmt;

#[derive(Clone, Copy)]
pub(crate) struct CellArray {
    pub(super) edge: i32,
    pub next: i32,
    pub next2: i32,
    pub(super) state: i32,
    pub(super) visited: bool,
    pub(super) setup: bool,
}

impl CellArray {
    pub(crate) fn get_edge(&self) -> i32 {
        self.edge
    }
    pub(crate) fn is_visited(&self) -> bool {
        self.visited
    }
    pub(crate) fn get_state(&self) -> i32 {
        self.state
    }
}

impl fmt::Display for CellArray {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let a: String;
        match self.next {
            -1 => a = "".to_string(),
            i => a = i.to_string(),
        }
        let b: String;
        match self.next2 {
            -1 => b = "".to_string(),
            i => b = i.to_string(),
        }
        write!(
            f,
            "edge: {}, state: {}, is_visited: {}, next: [{}], next2: [{}]",
            self.edge, self.state, self.visited, a, b
        )
    }
}
