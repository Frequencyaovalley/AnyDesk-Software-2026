use std::collections::HashMap;

struct Inventory {
    items: HashMap<String, u32>,
}

impl Inventory {
    fn new() -> Self {
        Self {
            items: HashMap::new(),
        }
    }

    fn add_item(&mut self, name: &str, quantity: u32) {
        self.items.insert(name.to_string(), quantity);
    }

    fn total_items(&self) -> u32 {
        self.items.values().sum()
    }

    fn print_report(&self) {
        println!("Inventory Summary");
        println!("=================");

        let mut entries: Vec<_> = self.items.iter().collect();
        entries.sort_by(|a, b| a.0.cmp(b.0));

        for (name, quantity) in entries {
            println!("{} : {}", name, quantity);
        }

        println!("=================");
        println!("Total Units: {}", self.total_items());
    }
}

fn main() {
    let mut inventory = Inventory::new();

    inventory.add_item("Keyboard", 15);
    inventory.add_item("Mouse", 28);
    inventory.add_item("Monitor", 9);
    inventory.add_item("Headset", 12);

    inventory.print_report();
}
