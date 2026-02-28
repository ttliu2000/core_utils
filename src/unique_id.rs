#[derive(Debug, Clone)]
pub struct UniqueId {
    id : u32,
}

impl UniqueId {
    pub fn next_id(&mut self) -> u32 {
        let r = self.id;
        self.id = self.id + 1;
        r
    }

    pub fn peek(&self) -> u32 {
        self.id
    }

    pub fn reset(&mut self) {
        self.id = 0
    }

    pub fn from_unique_id(id:&Self) -> Self {
        let mut r = UniqueId::default();
        r.id = id.id;
        r
    }
}

impl Default for UniqueId {
    fn default() -> Self {
        Self { id: 0 }
    }
}
