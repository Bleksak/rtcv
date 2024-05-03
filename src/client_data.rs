pub struct ClientData {
    name: String,
}

impl ClientData {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}
