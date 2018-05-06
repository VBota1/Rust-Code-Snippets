pub struct UID {
    pub value: i32,
}

impl UID {
    pub fn init() -> UID {
        UID { value: 0 }
    }

    pub fn new(&mut self) -> i32{
        self.value += 1;
        self.value
    }
}
