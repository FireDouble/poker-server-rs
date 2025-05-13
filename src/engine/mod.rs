use table::Table;

pub mod card;
pub mod player;
pub mod table;

#[derive(Clone)]
pub struct Engine {
    pub tables: Vec<Table>
}


impl Engine {
    pub fn new() -> Self {
        Self {
            tables: vec![
            ],
        }
    }

    pub fn get_tables(&mut self) -> &mut Vec<Table> { &mut self.tables }

    pub fn new_table(&mut self, name: String, key: String) {
        self.tables.push(Table::new(name, key));
    }
}
