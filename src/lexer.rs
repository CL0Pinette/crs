use std::{fs::File, io::Read};

mod dfa;
mod nfa;
mod token;
mod utils;
use nfa::nfa::NFA;
use nfa::regex::Regexer;
//use dfa::dfa::DFA;
//use token::Token;

pub fn lex(f: &mut File) {
    //-> (Token, i32) {
    let mut f_chars = String::new();
    match f.read_to_string(&mut f_chars) {
        _ => (),
    }
    // let mut line: i32 = 1;
    // let mut column: i32 = 0;
    //
    // for c in f_chars.chars() {
    //     if c == '\n' {
    //         line += 1;
    //         column = 0;
    //     } else {
    //         column += 1;
    //         println!("{} at {}:{}", c, line, column);
    //     }
    // }

    // let mut dfa = DFA::new(0, false);

    let mut reg = Regexer::new();

    // let s = reg.replace_arrays("(a|bc)*d");

    let s = reg.replace_arrays("(test|tt)");
    let mut nfa = NFA::new(&reg.inf_to_post(&s));

    nfa.populate();
    nfa.re2nfa();
    nfa.print();

    // dfa.add_state(1, false);
    // dfa.add_state(2, true);
    // dfa.add_state(3, true);
    // dfa.add_transition(0, '1', 1);
    // dfa.add_transition(0, '2', 1);
    // dfa.add_transition(0, '3', 1);
    // dfa.add_transition(0, '4', 1);
    // dfa.add_transition(0, '5', 1);
    // dfa.add_transition(0, '6', 1);
    // dfa.add_transition(0, '7', 1);
    // dfa.add_transition(0, '8', 1);
    // dfa.add_transition(0, '9', 1);

    // dfa.add_transition(1, '0', 1);
    // dfa.add_transition(1, '1', 1);
    // dfa.add_transition(1, '2', 1);
    // dfa.add_transition(1, '3', 1);
    // dfa.add_transition(1, '4', 1);
    // dfa.add_transition(1, '5', 1);
    // dfa.add_transition(1, '6', 1);
    // dfa.add_transition(1, '7', 1);
    // dfa.add_transition(1, '8', 1);
    // dfa.add_transition(1, '9', 1);

    // dfa.add_transition(1, ' ', 2);
    // dfa.add_transition(1, ';', 2);

    // dfa.add_transition(1, 'b', 2);
    // dfa.add_transition(1, 'c', 3);

    // let mut tok = Token::new(line, column);

    // for c in f_chars.chars() {
    //     if dfa.state() == 0 {
    //         tok = Token::new(line, column)
    //     }

    //     column += 1;
    //     if dfa.state() == -1 {
    //         println!(
    //             "Erreur on lexeme {} at {}:{}",
    //             tok.get_lexeme(),
    //             tok.get_location().0,
    //             tok.get_location().1
    //         );
    //         dfa.reset();
    //         tok = Token::new(line, column)
    //     }
    //     if dfa.is_final() {
    //         dfa.reset();
    //     }
    //     //print!("{}", dfa.input(c));
    //     dfa.input(c);

    //     if c == '\n' {
    //         line += 1;
    //         column = 1;
    //     }
    //     tok.add(c);
    //     println!(
    //         "State: {} {}, current lexeme: {}",
    //         dfa.state(),
    //         if dfa.is_accepting() { "true" } else { "false" },
    //         tok.get_lexeme()
    //     );
    // }
}
