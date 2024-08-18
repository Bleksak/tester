#[derive(Clone, Hash, PartialEq, Eq)]
pub struct Answer {
    name: String,
    correct: bool,
}

impl Answer {
    pub fn new(name: String, correct: bool) -> Self {
        Self { name, correct }
    }

    pub fn correct(&self) -> bool {
        self.correct
    }

    // pub fn name(&self) -> &str {
    //     &self.name
    // }
}

impl std::fmt::Display for Answer {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}
