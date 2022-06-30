use std::fmt;

pub(crate) struct Table {
    rows: Box<Vec<Vec<Obj>>>,
    column: i32,
    pub column_len: Vec<i32>,
    print_header: bool,
}

#[derive(Clone)]
pub(crate) enum Obj {
    S(String),
    I(i32),
}

impl Obj {
    fn to_string(&mut self) -> String {
        match self.into() {
            Some(c) => match c {
                Obj::I(i) => i.to_string(),
                Obj::S(s) => s.to_string(),
            },
            _ => "NULL".to_string(),
        }
    }
}

impl Table {
    pub(crate) fn new(col: i32, h: bool) -> Self {
        Table {
            rows: Box::new(Vec::new()),
            column: col,
            column_len: (0..col).collect::<Vec<_>>(),
            print_header: h,
        }
    }

    pub(crate) fn append_row(&mut self) {
        if self.rows.len() != 0 {
            match self.rows.last() {
                Some(e) => {
                    if e.len() == 0 {
                        return;
                    }
                }
                _ => (),
            }
        }
        let row: Vec<Obj> = Vec::new();
        // row.push(Obj::I(self.column));
        self.rows.push(row);
    }

    pub(crate) fn append_column(&mut self, c: Obj) {
        let mut row_size: i32 = 0;
        if self.rows.len() > 0 {
            let mut v: Vec<Obj> = self.rows.pop().unwrap();
            v.push(c.clone());
            row_size = v.len() as i32;
            self.rows.push(v);
        }
        let len = match c.clone() {
            Obj::I(a) => a / 10 + 1,
            Obj::S(s) => s.len() as i32,
        };
        if self.column_len[(row_size - 1) as usize] < len {
            self.column_len[(row_size - 1) as usize] = len;
        };
    }

    pub fn string(&mut self) -> String {
        let mut sb = String::new();
        let mut sumlen = 0;
        for i in self.column_len.iter() {
            sumlen += i;
        }
        if self.print_header {
            sb.push('|');
            sb.push_str(&printchar(
                '=',
                sumlen + MARGIN * 2 * self.column + (self.column - 1),
            ));
            sb.push_str("|\n");
        } else {
            sb.push('|');
            sb.push_str(&printchar(
                '-',
                sumlen + MARGIN * 2 * self.column + (self.column - 1),
            ));
            sb.push_str("|\n");
        };
        for i in 0..self.rows.len() {
            let row = self.rows.iter().nth(i);
            match row {
                Some(r) => {
                    for ii in 0..self.column {
                        let mut o = String::new();
                        if ii < r.len() as i32 {
                            match r.iter().nth(ii as usize) {
                                Some(v) => {
                                    o = v.clone().to_string();
                                }
                                _ => (),
                            }
                        };
                        sb.push('|');
                        sb.push_str(&printchar(' ', MARGIN));
                        sb.push_str(&o);
                        match self.column_len.iter().nth(ii as usize) {
                            Some(f) => sb.push_str(&printchar(' ', f - (o.len() as i32) + MARGIN)),
                            _ => panic!("Error string of Table"),
                        }
                    }
                    sb.push_str("|\n");
                    if self.print_header && i == 0 {
                        sb.push('|');
                        sb.push_str(&printchar(
                            '=',
                            sumlen + MARGIN * 2 * self.column + (self.column - 1),
                        ));
                        sb.push_str("|\n");
                    } else {
                        sb.push('|');
                        sb.push_str(&printchar(
                            '-',
                            sumlen + MARGIN * 2 * self.column + (self.column - 1),
                        ));
                        sb.push_str("|\n");
                    };
                }
                _ => (),
            }
        }

        sb
    }
}

fn printchar(c: char, len: i32) -> String {
    let mut sb = String::new();
    for _i in 0..len {
        sb.push(c);
    }
    sb
}

impl fmt::Display for Table {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

const MARGIN: i32 = 2;
