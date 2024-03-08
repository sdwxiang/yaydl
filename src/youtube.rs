
pub struct Youtube {

}

impl Youtube {
    pub fn new() -> Self {
        Self{}
    }

    /// test id TgoYoc8oBFw
    pub fn fetch_url(&self, id: &str) {
        println!("{id}");
    }
}