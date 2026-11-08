use core::str::from_utf8;
use std::process::Command;

pub struct CommandHandler {
    command_list: Vec<String>,
}

impl crate::InputHandler for CommandHandler {
    fn get_canidates(&self, input: &str) -> Vec<String> {
        let mut canidates: Vec<String> = vec![];
        let mut prbly_valid = false;

        for c in &self.command_list {
            if c.contains(&input) {
                canidates.push(c.clone());
            }
            if input.starts_with(c) {
                // this is for when we do not know what the user is trying to
                //do but they are typing somthing that starts wit a valid command
                prbly_valid = true;
            }
        }
        if prbly_valid && canidates.len() == 0 {
            canidates.push(input.to_string());
        }
        return canidates;
    }

    fn execute(&self, action: String) -> Option<(String, i32)> {
        let out = Command::new("sh")
            .arg("-c")
            .arg(&action)
            .output()
            .expect("fuck");
        if out.stderr.len() > 0 {
            return None;
        }
        match from_utf8(&out.stdout) {
            Ok(a) => Some((a.to_string(), 0)),
            Err(_) => None,
        }
    }
}
impl CommandHandler {
    pub fn new() -> Self {
        let out = Command::new("bash")
            .arg("-c")
            .arg("compgen -c")
            .output()
            .expect("fuck")
            .stdout;
        let destop_commands = crate::destop_handler::get_destop_apps();
        let mut commands: Vec<String> = from_utf8(&out)
            .unwrap_or("")
            .to_string()
            .lines()
            .map(|s| s.to_string())
            .filter(|s| !destop_commands.contains(s))
            .collect();
        println!("{:#?}", commands);
        commands = commands
            .into_iter()
            .collect::<std::collections::HashSet<String>>()
            .into_iter()
            .collect::<Vec<String>>();
        // optimisation pregenerate this list
        Self {
            command_list: commands,
        }
    }
}
