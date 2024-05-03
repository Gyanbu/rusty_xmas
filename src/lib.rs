#[macro_export]
macro_rules! load_input {
    () => {
        std::fs::read_to_string(format!("src/bin/{}/input.txt", module_path!())).unwrap()
    };
}
