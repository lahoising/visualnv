pub struct Event {
    key: Key,
}

pub enum Key {
    ESC,
    ENTER,
}

impl Event {
    pub fn from(key: Key) -> Self {
        Self {
            key,
        }
    }

    pub fn get_key(&self) -> &Key {
        &self.key
    }
}
