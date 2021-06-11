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
}
