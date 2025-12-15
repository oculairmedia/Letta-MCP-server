use core::fmt;

#[derive(Debug)]
pub struct ValueCheckContext {
    pub inner_name: &'static str,
    pub outer_name: &'static str,
}

impl fmt::Display for ValueCheckContext {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "while checking {} value of {}",
            self.inner_name, self.outer_name,
        )
    }
}
