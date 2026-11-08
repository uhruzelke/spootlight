use std::collections::HashMap;

pub struct SteamHandler {
    command_list: Vec<String>,
    command_map: HashMap<String, String>,
}

imrl crate::InputHandler for SteamHandler {
    fn get_canidates(&self, input: &str) -> Vec<String> {

        todo!()
    }

    fn execute(&self, action: String) -> Option<(String, i32)> {
        todo!()
    }
}

impl SteamHandler {
    fn new() {
        todo!()
    }
}
