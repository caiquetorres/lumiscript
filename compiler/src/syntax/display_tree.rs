pub trait DisplayTree {
    fn display(&self, layer: usize);
}

pub fn branch(branch_name: &str, layer: usize) {
    println!("{}", format!("{}├── {}", "│   ".repeat(layer), branch_name));
}
