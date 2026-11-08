use core::str::from_utf8;
use freedesktop::ApplicationEntry;
use glob::glob;
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::prelude::*;
use std::os::unix::process::CommandExt;
use std::process::{Command, Stdio};

pub struct DestopHandler {
    command_list: Vec<String>,
    name_to_key: HashMap<String, String>,
}

impl crate::InputHandler for DestopHandler {
    fn get_canidates(&self, input: &str) -> Vec<String> {
        let mut canidates: Vec<String> = vec![];
        let mut prbly_valid = false;

        for c in &self.command_list {
            if c.contains(&input) {
                canidates.push(c.clone());
            }
            if input.starts_with(c) {
                prbly_valid = true;
            }
        }
        if prbly_valid && canidates.len() == 0 {
            canidates.push(input.to_string());
        }
        return canidates;
    }

    fn execute(&self, action: String) -> Option<(String, i32)> {
        if self.name_to_key.contains_key(&action) {
            let exec = self.name_to_key.get(&action).unwrap();
            let out = Command::new("sh").arg("-c").arg(&exec).exec();
            //return Some(("".to_string(), -1));
        }
        return None;
    }
}
impl DestopHandler {
    pub fn new() -> Self {
        let destop_commands = crate::destop_handler::get_destop_apps();
        let name_to_k = get_app_hasmap();
        // let commands: Vec<String> = from_utf8(&out)
        //     .unwrap_or("")
        //     .to_string()
        //     .lines()
        //     .map(|s| s.to_string())
        //     .filter(|s| !destop_commands.contains(s))
        //     .collect();
        // // optimisation pregenerate this list
        Self {
            command_list: destop_commands,
            name_to_key: name_to_k,
        }
    }
}

pub fn get_app_hasmap() -> HashMap<String, String> {
    // returns the list of destop apps commands (first word in execution) we will be filtering them out
    // maby ill make a hasmap instead for the actual handler so the namse show up but the corret comands are run.
    // this one spawns apps the other one shall run them
    let mut apps: HashMap<String, String> = HashMap::new();
    for app in ApplicationEntry::all() {
        apps.insert(app.name().unwrap().to_lowercase(), app.exec().unwrap());
    }
    return apps;
}

pub fn get_destop_apps() -> Vec<String> {
    // returns the list of destop apps commands (first word in execution) we will be filtering them out
    // maby ill make a hasmap instead for the actual handler so the namse show up but the corret comands are run.
    // this one spawns apps the other one shall run them
    let h = get_app_hasmap();
    let b = h.into_keys();
    let apps = b.into_iter().collect();
    return apps;
}
