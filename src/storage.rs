pub trait Storage {
    fn get_item(&self, key: &str) -> Option<String>;
    fn set_item(&mut self, key: &str, value: &str);
    fn remove_item(&mut self, key: &str);
}
