use super::dfa::DFA;

impl DFA {
    pub fn add_reg(&mut self, s: &str) {
        // let mut state = 0;
        let mut vletter = Vec::<(char, char)>::new();
        let mut is_escaped = false;
        let mut in_array = false;
        let mut pair = ('a', 'a');
        let mut in_pair = false;
        let mut vchar = Vec::<char>::new();
        for c in s.chars() {
            if c == '\\' {
                is_escaped = true;
            } else if c == '[' && !is_escaped {
                in_array = true;
            } else if in_array && c == ']' {
                in_array = false;
                vchar.append(&mut gen_list(&mut vletter));
            } else if in_array && c == '-' {
                in_pair = true;
            } else if in_array && in_pair {
                pair.1 = c;
                vletter.push(pair);
                in_pair = false;
            } else if in_array {
                pair.0 = c;
            }
        }

        println!("Test");
        vchar.iter().for_each(|f| {
            println!("{}", f);
        });
        println!("End Test");
    }
}
pub fn gen_list(l: &mut Vec<(char, char)>) -> Vec<char> {
    let mut vchar = Vec::<char>::new();
    let mut i: u32 = 0;
    l.iter().for_each(|(c1, c2): &(char, char)| {
        i = (*c1).into();
        while i <= (*c2).into() {
            vchar.push(i.try_into().unwrap());
            i += 1;
        }
    });
    vchar
}
