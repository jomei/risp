mod parser;

pub struct Lisp {
    eval: Eval,
}

impl Lisp {
    pub fn new() -> Self {
        Lisp {
            eval: Eval::new()
        }
    }
}