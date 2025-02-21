use crate::env;
use crate::{
    loader::{self, Loader},
    Machine,
};
use iced::widget::Scrollable;
use iced::{
    executor, time,
    widget::{Button, Column, Row, Space, Text, TextInput},
    Application, Command, Element, Font, Settings, Subscription, Theme,
};
use std::time::Duration;
#[derive(Debug, Clone)]
pub enum Message {
    TogglePause,
    Tick,
    Step,
    InputChanged(String),
    ScrollUp,
    ScrollDown,
}
fn create_scrollable_from_string(input: String) -> impl Into<Element<'static, Message>> {
    // Split the input string into lines and map each line to a `Text` widget
    let lines = input
        .lines()
        .map(|line| Text::new(line.to_string()).into()) // Convert each Text widget into Element<Message>
        .collect::<Vec<Element<Message>>>();

    // Create a column with each line as a separate `Text` widget
    let content = column(lines);

    // Make the column scrollable
    scrollable(content)
}

use iced::widget::{column, scrollable, vertical_space};
pub struct App {
    paused: bool,
    machine: Machine,
    input_value: String,
    frequency: usize,
    file_path: String,
}
use crate::read_file_to_string;
impl App {
    pub fn new(file_path: &str) -> Self {
        let mut machine = Machine::new(); // "./src/dtd.obj"
        match read_file_to_string(file_path) {
            Ok(obj_file) => {
                if let Some(loader) = Loader::parse(&obj_file) {
                    //machine = Machine::new();
                    // Load program
                    loader.load_into_memory(&mut machine);
                    // Set PC to start addres
                    machine.set_reg(Machine::PC, loader.start_record.start_address);
                    // ######## GUI ###########

                    // Lengath of program
                    let prog_len = loader.start_record.program_length;
                    machine.set_prog_len(prog_len as usize);
                } else {
                    println!("Failed to parse .obj file.");
                }
            }
            Err(e) => eprintln!("Error reading file: {}", e),
        }
        let mut app = Self {
            paused: true,
            machine,
            input_value: String::new(),
            frequency: 1,
            file_path: file_path.to_string(),
        };
        app.refresh_memory_display();
        app
    }

    fn refresh_memory_display(&mut self) {}
}
impl Application for App {
    type Executor = executor::Default;
    type Message = Message;
    //type Flags = ();
    type Flags = String;
    type Theme = iced::Theme;

    fn new(file_path: Self::Flags) -> (Self, Command<Self::Message>) {
        (App::new(&file_path), iced::Command::none())
    }

    fn title(&self) -> String {
        String::from("My App")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::TogglePause => {
                self.paused = !self.paused;
            }
            Message::Tick => {
                if !self.paused {
                    for _ in 0..self.frequency {
                        self.machine.execute();
                    }
                }
            }
            Message::Step => {
                self.machine.execute();
                //print!("{}", self.machine.memory.memory_to_string());
            }
            Message::InputChanged(new_value) => {
                self.input_value = new_value;
                self.frequency = match self.input_value.parse() {
                    Ok(value) => std::cmp::min(value, 5000),
                    Err(_) => self.frequency,
                }
            }
            Message::ScrollUp => {}
            Message::ScrollDown => {}
        }
        Command::none()
    }

    fn view(&self) -> Element<Self::Message> {
        let toggle_pause_text = if self.paused { "Resume" } else { "Pause" };
        let scrollable_widget = create_scrollable_from_string(self.machine.memory.text_screen());
        Column::<Message>::new()
            .push(
                Row::new()
                    .push(Button::new(Text::new(toggle_pause_text)).on_press(Message::TogglePause))
                    .push(Space::with_width(iced::Length::Fixed(10.0)))
                    .push(Button::new(Text::new("Step")).on_press(Message::Step))
                    .push(Space::with_width(iced::Length::Fixed(10.0)))
                    .push(Button::new(Text::new("Up")).on_press(Message::ScrollUp))
                    .push(Space::with_width(iced::Length::Fixed(10.0)))
                    .push(Button::new(Text::new("Down")).on_press(Message::ScrollDown)),
            )
            .push(Space::with_height(iced::Length::Fixed(20.0)))
            .push(
                TextInput::new("Frequency", &self.input_value)
                    .on_input(Message::InputChanged)
                    .padding(10),
            )
            .push(Text::new("Registers:")) // Apply font to other text if desired
            .push(Text::new(format!(
                "A: {:X}",
                self.machine.get_reg(0).unwrap()
            )))
            .push(Text::new(format!(
                "X: {:X}",
                self.machine.get_reg(1).unwrap()
            )))
            .push(Text::new(format!(
                "L: {:X}",
                self.machine.get_reg(2).unwrap()
            )))
            .push(Text::new(format!(
                "B: {:X}",
                self.machine.get_reg(3).unwrap()
            )))
            .push(Text::new(format!(
                "S: {:X}",
                self.machine.get_reg(4).unwrap()
            )))
            .push(Text::new(format!(
                "T: {:X}",
                self.machine.get_reg(5).unwrap()
            )))
            .push(Text::new(format!(
                "PC: {:X}",
                self.machine.get_reg(8).unwrap()
            )))
            .push(Text::new(format!(
                "SW: {:X}",
                self.machine.get_reg(9).unwrap()
            )))
            .push(Text::new("Textual screen:\n"))
            // .push(scrollable(column![
            //     "Scroll me!",
            //     vertical_space(3000),
            //     "You did it!",
            // ]))
            //.push(Text::new(self.machine.memory.memory_to_string()))
            .push(Column::new().push(scrollable_widget))
            .into()
        //.push(Column::with_children())
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        time::every(Duration::from_millis(100)).map(|_| Message::Tick)
    }
}
