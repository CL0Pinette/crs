use std::collections::HashMap;

pub struct DFA {
    m_initial: i32,
    m_state: i32,
    m_states: Vec<i32>,
    pub m_final_states: Vec<i32>,
    m_transitions: HashMap<(i32, char), i32>,
}

impl<'a> DFA {
    pub(crate) fn new(initial: i32, is_final: bool) -> DFA {
        let mut a = DFA {
            m_initial: initial,
            m_state: initial,
            m_states: Vec::new(),
            m_final_states: Vec::new(),
            m_transitions: HashMap::new(),
        };
        a.add_state(initial, is_final);
        a.add_state(-1, false);
        a
    }
    pub(crate) fn add_state(&mut self, s: i32, is_final: bool) {
        self.m_states.push(s);
        if is_final {
            self.m_final_states.push(s);
        }
    }

    pub(crate) fn add_transition(&mut self, src: i32, input: char, dest: i32) {
        self.m_transitions.insert((src, input), dest);
    }

    pub(crate) fn reset(&mut self) {
        self.m_state = self.m_initial;
    }

    pub(crate) fn input(&mut self, inp: char) -> i32 {
        if !self.m_transitions.contains_key(&(self.m_state, inp)) {
            self.m_state = -1;
            return -1;
        }
        let a = self.m_transitions.get(&(self.m_state, inp));
        self.m_state = *a.unwrap();
        *a.unwrap()
    }

    pub(crate) fn is_accepting(&mut self) -> bool {
        self.m_final_states.len() != 0
    }

    pub(crate) fn state(&mut self) -> i32 {
        self.m_state
    }

    pub(crate) fn is_final(&mut self) -> bool {
        self.m_final_states.contains(&self.m_state)
    }
}
