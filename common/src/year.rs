pub fn get_years() -> Vec<i32> {
    vec![2023, 2024]
}

pub trait AdventYear {
    fn run(&self, input: Option<i32>) -> Result<(), Box<dyn std::error::Error>>;
}
