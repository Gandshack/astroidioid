pub struct Input {
    pub w: bool,
    pub a: bool,
    pub s: bool,
    pub d: bool,
    pub space: bool,
}

impl Input {
    pub fn new() -> Self {
        Self {
            w: false,
            a: false,
            s: false,
            d: false,
            space: false,
        }
    }
}
