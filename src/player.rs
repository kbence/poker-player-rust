
pub struct Player {}

impl Player {
    pub fn new() -> Player {
        Player {}
    }

    pub fn version(&self) -> String {
        String::from("0.1.0")
    }
}
