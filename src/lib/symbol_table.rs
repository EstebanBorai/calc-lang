pub struct SymbolTable {
    entries: Vec<(String, f64)>,
}

impl SymbolTable {
    /// Create an empty symbol table
    pub fn new() -> Self {
        SymbolTable {
            entries: Vec::<(String, f64)>::new(),
        }
    }

    /// Inserts a new symbol into the symbol table.
    /// If the symbol doesn't exists returns an error.
    pub fn insert_symbol(&mut self, identifier: &str) -> Result<usize, String> {
        if let Some((existing_identifier, _)) = self
            .entries
            .iter()
            .find(|(existent_identifier, _)| existent_identifier == identifier)
        {
            return Err(format!(
                "Identifier '{}' already declared",
                existing_identifier
            ));
        }

        self.entries.push((identifier.to_string(), 0.));

        Ok(self.entries.len() - 1)
    }

    /// Retrieve the index of a symbol from the symbol table
    pub fn index_of_symbol(&self, identifier: &str) -> Result<usize, String> {
        if let Some(index) = self
            .entries
            .iter()
            .position(|(existing_identifier, _)| existing_identifier == identifier)
        {
            return Ok(index);
        }

        Err(format!("Undefined identifier '{}'", identifier))
    }

    pub fn get(&self, index: usize) -> f64 {
        self.entries[index].1
    }

    pub fn set(&mut self, index: usize, value: f64) {
        self.entries[index].1 = value;
    }

    pub fn iter(&self) -> std::slice::Iter<(String, f64)> {
        self.entries.iter()
    }

    pub fn print(&self) {
        if self.entries.is_empty() {
            println!("No variables assigned yet");
            return;
        }

        println!("Variables: ");

        for variable in self.iter() {
            println!("{}: {}", variable.0, variable.1);
        }
    }
}
