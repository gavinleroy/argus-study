use std::fmt;

pub struct Database<T> {
    values: Vec<T>,
}

impl<T: fmt::Display> fmt::Display for Database<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[")?;
        for (i, value) in self.values.iter().enumerate() {
            if i > 0 {
                write!(f, ",")?;
            }
            write!(f, "  {}\n", value)?;
        }
        write!(f, "]\n")?;
        Ok(())
    }
}

impl<T> Database<T> {
    pub fn new() -> Self {
        Database {
            values: Vec::new(),
        }
    }

    pub fn insert(&mut self, value: T) {
        self.values.push(value);
    }
}
