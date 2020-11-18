use super::universe::*;
use std::fmt::{Display, Formatter};
use std::fmt;

pub fn cls() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}

impl Display for Cell {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let s = match self {
            Cell::Alive => "■",
            Cell::Dead => "□",
        };

        write!(f, "{}", s)
    }
}

impl Display for Universe {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let (h, w) = self.dim();
        let mut str = String::with_capacity(h * w * 2);

        for i in 0..h {
            for j in 0..w {
                str.push_str(format!("{}", self[(i, j)]).as_str());
                if j != w - 1 {
                    str.push(' ');
                }
            }
            str.push('\n');
        }

        write!(f, "{}", str)
    }
}
