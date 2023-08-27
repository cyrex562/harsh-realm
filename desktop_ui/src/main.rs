use iced::executor;
use iced::Alignment;
use iced::Application;
use iced::Command;
use iced::Element;
use iced::Length;
use iced::Theme;
use iced::Settings;

use env_logger;
use iced::widget::button;
use iced::widget::column;
use iced::widget::container;
use iced::widget::row;
use iced::widget::scrollable;
use iced::widget::scrollable::Properties;
use iced::widget::text;
use iced::widget::text_input;
use iced::widget::Column;
use iced::widget::Text;
use log::debug;

use crate::color::color_from_web;
use crate::command::process_string;

mod character_stats;
mod color;
mod command;
mod dice;

#[derive(Debug, Clone)]
pub enum Message {
    TextInputChanged(String),
    SendButtonPressed,
}

pub enum TextType {
    CommandInput,
    CommandOutput,
    Help,
    Error,
}

pub struct TextLine {
    pub text: String,
    pub text_type: TextType,
}

pub enum GameState {
    Default,
    CharacterCreation,
}

pub struct ChatRpg {
    pub text_input_value: String,
    pub lines: Vec<TextLine>,
    pub game_state: GameState,
    pub game_state_stack: Vec<GameState>,
}

impl Application for ChatRpg {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Message>) {
        (
            ChatRpg {
                text_input_value: String::from(""),
                lines: Vec::new(),
                game_state: GameState::Default,
                game_state_stack: Vec::new(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("ChatRPG")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::TextInputChanged(value) => {
                self.text_input_value = value;
                Command::none()
            }
            Message::SendButtonPressed => {
                debug!("send button pressed");

                let in_line_str = self.text_input_value.clone();
                let out_line = process_string(&self.text_input_value);

                let in_line = TextLine {
                    text: in_line_str,
                    text_type: TextType::CommandInput,
                };
                self.lines.push(in_line);
                self.lines.push(out_line);
                self.text_input_value.clear();

                Command::none()
            }
        }
    }

    fn view(&self) -> Element<Message> {
        // Text::new("Hello World!").into()

        let mut col = Column::new()
            .width(Length::Fill)
            .height(Length::Fill)
            .width(Length::Fill)
            .height(Length::Fill);
        for line in &self.lines {
            let text_color = match line.text_type {
                TextType::CommandInput => color_from_web("#ba55d3"), // medium orchid
                TextType::CommandOutput => color_from_web("#800080"), // purple
                TextType::Help => color_from_web("#b8860b"),         // dark goldenrod
                TextType::Error => color_from_web("#b22222"),        // firebrick
            };

            col = col.push(Text::new(line.text.clone()).style(text_color));
        }

        let scrollable_content: Element<Message> = scrollable(col)
            .width(Length::Fill)
            .height(Length::Fill)
            .vertical_scroll(Properties::new().scroller_width(10).width(10).margin(10))
            .horizontal_scroll(Properties::new().scroller_width(10).width(10).margin(10))
            .into();

        let txt_input_box: Element<Message> = text_input("input content", &self.text_input_value)
            .on_input(Message::TextInputChanged)
            .on_submit(Message::SendButtonPressed)
            .into();
        let send_btn: Element<Message> = button(text("send"))
            .on_press(Message::SendButtonPressed)
            .into();

        let input_content: Element<Message> = row![text("input"), txt_input_box, send_btn]
            .width(Length::Fill)
            .height(40)
            .into();

        let content: Element<Message> = column![scrollable_content, input_content]
            .width(Length::Fill)
            .height(Length::Fill)
            .align_items(Alignment::Center)
            .spacing(10)
            .into();

        Element::from(
            container(content)
                .width(Length::Fill)
                .height(Length::Fill)
                .padding(40)
                .center_x()
                .center_y(),
        )
    }
}

fn main() -> Result<(), iced::Error> {
    env_logger::init();
    ChatRpg::run(Settings::default())
}
