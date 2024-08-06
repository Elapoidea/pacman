use std::fmt;

pub struct BitBoard(pub u64);

impl fmt::Display for BitBoard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in self.0.to_be_bytes() {
            write!(f, "{0}{i:b}\n", (0..(8-i.checked_ilog2().unwrap_or(0))).map(|_| "0").collect::<String>());
        }

        Ok(())
    }
}
