pub struct User {
    pub(crate) username: String,
    pub(crate) active: bool,
}

impl User {
    //Associated function: No 'self' parameter
    //Often used as a 'new' constructor
    pub(crate) fn new(name: &str) -> Self {
        Self {
            username: String::from(name),
            active: false,
        }
    }
    //A standard method for comparison
    pub fn deactivate(&mut self) {
        self.active = false
    }
}
