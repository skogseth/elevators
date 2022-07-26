pub struct Requests {
    pub internal: Vec<bool>,
    pub external: Vec<bool>,
}

impl Requests {
    pub fn new(n_floors: usize) -> Requests {
        Requests {
            internal: vec![false; n_floors],
            external: vec![false; n_floors],
        }
    }
}

