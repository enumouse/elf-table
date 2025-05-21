//! Simple ASCII table drawing

pub struct Table {
    pub title: String,
    pub items: Vec<(String, String)>,
}

impl Table {
    pub fn new() -> Self {
        Self {
            title: "ELF File Header".to_string(),
            items: vec![],
        }
    }

    /// Add a row to the table
    pub fn add_row(&mut self, title: &str, value: &str) {

    }

    /// Draw table to standard output
    pub fn draw(&self) {
        
    }
}