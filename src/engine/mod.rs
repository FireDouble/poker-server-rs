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
            tables: vec![],
        }
    }

    pub fn get_tables(&mut self) -> &mut Vec<Table> { &mut self.tables }

    pub fn new_table(&mut self, name: String, table_name: String, max_players: usize, minimal_bid: i32, starting_chips: i32, key: String) {
        self.tables.push(Table::new(name,table_name, max_players, minimal_bid, starting_chips, key));
    }

    pub fn remove_table(&mut self, index: usize) {
        self.tables.remove(index);
    }
}
