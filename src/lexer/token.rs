pub enum Types {
    Function,
    Str,
    Condition,
    Operator,
    ParLeft,
    ParRight,
    DoubleQuotes,
    SingleQuote,
    Undefined,
}

pub struct Token {
    lexeme: String,
    tok_type: Types,
    location: (i32, i32),
}

impl Token {
    pub(crate) fn new(line: i32, column: i32) -> Self {
        Token {
            lexeme: "".to_string(),
            tok_type: Types::Undefined,
            location: (line, column),
        }
    }

    pub(crate) fn add(&mut self, c: char) {
        self.lexeme.push(c);
    }
    pub(crate) fn get_lexeme(&mut self) -> String {
        self.lexeme.clone()
    }
    pub(crate) fn get_location(&mut self) -> (i32, i32) {
        self.location
    }
}
