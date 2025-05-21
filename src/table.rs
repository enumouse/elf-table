//! Simple ASCII table drawing

pub struct Table {
    pub rows: Vec<(String, String)>,
}

impl Table {
    pub fn new() -> Self {
        Self {
            rows: vec![],
        }
    }

    /// Add a row to the table
    pub fn add_row(&mut self, title: &str, value: &str) {
        self.rows.push(
            (title.to_string(), value.to_string())
        );
    }

    /// Draw table to standard output
    pub fn draw(&self) {
        let title_width = self.rows.iter()
            .map(|(x, _)| x.len())
            .max()
            .unwrap_or(0);
        
        let value_width = self.rows.iter()
            .map(|(_, x)| x.len())
            .max()
            .unwrap_or(0);

        let total_width = title_width + value_width + 7;
        
        println!("{}", "-".repeat(total_width));

        for row in &self.rows {
            println!(
                "| {:<title_width$} | {:<value_width$} |",
                row.0,
                row.1,
                title_width = title_width,
                value_width = value_width
            );
        }

        println!("{}", "-".repeat(total_width));
    }
}