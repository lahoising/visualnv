pub struct Label {
    text: String,
}

impl Label {
    pub fn from<T: Into<String>>(text: T) -> Self {
        Self {
            text: text.into(),
        }
    }

    pub fn get_text(&self) -> &String {
        &self.text
    }
}
