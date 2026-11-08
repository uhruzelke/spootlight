mod calc;
mod command_handler;
mod destop_handler;
mod steam_handler;

use core::str::{self, from_utf8};
use iced::{
    Alignment, Element, Event, Program, Settings, Size, Task,
    advanced::widget::{operate, operation::focusable},
    event::{self, Status, listen_with},
    keyboard::{Event::KeyPressed, Key, key::Named},
    message,
    widget::{
        Column, button, column, container, operation::focus, text, text_editor::KeyPress,
        text_input,
    },
    window,
};
use std::{println, process::Command, todo, vec};

#[derive(Default)]
struct Launcher {
    content: String,
    output: String,
    handlers: Vec<Box<dyn InputHandler>>,
    auto_compleat: Vec<String>,
    auto_compleat_poss: usize,
    auto_compleat_taking_suggetions: bool,
}

#[derive(Debug, Clone)]
pub enum Message {
    ContentChanged(String),
    Submit,
    Typed(String),
    Move_suggestion(i32),
}

pub trait InputHandler {
    fn get_canidates(&self, input: &str) -> Vec<String>;
    fn execute(&self, action: String) -> Option<(String, i32)>; // the i32 is a code the handlers may pass to do special stuff like closing the program
}

impl Launcher {
    pub fn new() -> Self {
        let mut l = Launcher::default();
        l.handlers = vec![
            Box::new(destop_handler::DestopHandler::new()),
            Box::new(command_handler::CommandHandler::new()),
            Box::new(calc::MathHandler),
        ];
        return l;
    }
    pub fn view(&self) -> Column<Message> {
        column![
            text_input("...", &self.content)
                .id("input")
                .on_input(Message::ContentChanged),
            text(&self.output)
        ]
    }
    fn get_suggestions(&mut self) {
        let mut suggestions: String = "".to_string();
        let mut auto: Vec<String> = vec![];
        let mut index = 0;
        for e in &self.handlers {
            let canidates = e.get_canidates(&self.content);
            auto.append(&mut canidates.clone().as_mut());
            for s in canidates {
                index += 1;
                if self.auto_compleat_taking_suggetions {
                    if self.auto_compleat_poss == index {
                        suggestions = format!("{} \n>{}", suggestions, s);
                        continue;
                    }
                }
                suggestions = format!("{} \n {}", suggestions, s);
            }
        }
        self.output = suggestions;
        self.auto_compleat = auto;
    }
    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::ContentChanged(content) => {
                self.content = content;
                self.get_suggestions();

                Task::default()
            }
            Message::Submit => {
                /*
                let out = Command::new("sh")
                    .arg("-c")
                    .arg(&self.content)
                    .output()
                    .expect("fuck")
                    .stdout;
                self.output = str::from_utf8(&out).unwrap_or("").to_string();
                */
                let mut out: String = "".to_string();
                // as long as we arrange the handlers in order of recorce use were fine
                let mut command_ex = self.content.clone();
                if self.auto_compleat_taking_suggetions && self.auto_compleat_poss > 0 {
                    command_ex = self.auto_compleat[self.auto_compleat_poss - 1]
                        .clone()
                        .trim()
                        .to_string();
                    println!("{}", command_ex);
                }
                for e in &self.handlers {
                    let o = e.execute(command_ex.clone());
                    if o.is_some() {
                        let u = o.unwrap();
                        out = u.0;
                        if u.1 == -1 {
                            return iced::exit();
                        }
                        break;
                    }
                }
                self.output = out;
                //std::process::exit(78934);
                Task::default()
            }
            Message::Typed(a) => {
                self.content = a;
                self.get_suggestions();
                focus("input")
            }
            Message::Move_suggestion(by) => {
                let movment = self.auto_compleat_poss as i32 + by;
                if movment > 0 {
                    self.auto_compleat_taking_suggetions = true;
                    self.auto_compleat_poss = movment as usize;
                } else {
                    self.auto_compleat_taking_suggetions = false;
                    self.auto_compleat_poss = 0;
                }
                self.get_suggestions();
                Task::default()
            }
        }
    }
    fn subscription(&self) -> iced::Subscription<Message> {
        listen_with(|event, status, _| match (event, status) {
            (
                Event::Keyboard(KeyPressed {
                    key: Key::Named(Named::Enter),
                    ..
                }),
                Status::Ignored,
            ) => Some(Message::Submit),
            (
                Event::Keyboard(KeyPressed {
                    key: Key::Named(Named::ArrowUp),
                    ..
                }),
                Status::Ignored,
            ) => Some(Message::Move_suggestion(-1)),
            (
                Event::Keyboard(KeyPressed {
                    key: Key::Named(Named::ArrowDown),
                    ..
                }),
                Status::Ignored,
            ) => Some(Message::Move_suggestion(1)),
            (
                Event::Keyboard(KeyPressed {
                    key: Key::Character(a),
                    ..
                }),
                Status::Ignored,
            ) => Some(Message::Typed(a.to_string())),
            _ => None,
        })
    }
}

fn main() -> iced::Result {
    //let a = destop_handler::get_app_hasmap();
    //println!("{:#?}", a);

    iced::application(Launcher::new, Launcher::update, Launcher::view)
        .subscription(Launcher::subscription)
        .window_size(Size::new(400.0, 647.0))
        .resizable(false)
        .decorations(false)
        .centered()
        .run()
}
