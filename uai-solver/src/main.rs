
fn main() {
    println!("Hello, World!");
}


#[cfg(tests)]
mod tests {
    #[test]
    fn test_model_parse() {
        let input_model = include_str!("../tests/test1.uai");
        
    }

    #[test]
    fn test_data_parse() {
        let input_data = include_str!("../tests/test1.data");
    }
}