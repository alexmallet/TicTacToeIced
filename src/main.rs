use iced::widget::{button, container, row, text, column};
use iced::{executor, Application, Command, Length, Settings};

use self::theme::Theme;
use self::widget::Element;

fn main() {
    let mut settings = Settings::default();
    settings.window.resizable = true;
    settings.window.size = (1200,1200);
    TicTacToe::run(settings).unwrap();
}

#[derive(Debug, Clone)]
enum Message {
    ButtonPress(u8),
    Restart,
}

#[derive(Debug, Clone)]
struct TicTacToe {
    player_id: i32,
    moves_count: i32,
    cell_r1_c1: String,
    cell_r1_c2: String,
    cell_r1_c3: String,
    cell_r2_c1: String,
    cell_r2_c2: String,
    cell_r2_c3: String,
    cell_r3_c1: String,
    cell_r3_c2: String,
    cell_r3_c3: String,
    message: String,
    game_over: bool,
}

const BUTTON_SIZE: u16 = 200;

fn validate_winner (content: String) -> bool {
    if content == "XXX".to_string() || content == "OOO".to_string() {
        return  true;
    } 

    false
}

fn check_for_winner (game: &mut TicTacToe) -> bool {
    if game.game_over {
        return  true;
    }

    let mut winner = 1;
    if game.player_id == 1 {
        winner = 2;
    }
    game.message = format!("Player {} is the winner!!!!", winner.to_string());

    // Checking the rows
    let row1: String =format!("{}{}{}", game.cell_r1_c1, game.cell_r1_c2 , game.cell_r1_c3);
    let row2: String =format!("{}{}{}", game.cell_r2_c1, game.cell_r2_c2 , game.cell_r2_c3);
    let row3: String =format!("{}{}{}", game.cell_r3_c1, game.cell_r3_c2 , game.cell_r3_c3);
    if validate_winner(row1) || validate_winner(row2) || validate_winner(row3) {
        return  true;
    } 
    
    // Checking the cols
    let col1: String =format!("{}{}{}", game.cell_r1_c1, game.cell_r2_c1 , game.cell_r3_c1);
    let col2: String =format!("{}{}{}", game.cell_r1_c2, game.cell_r2_c2 , game.cell_r3_c2);
    let col3: String =format!("{}{}{}", game.cell_r1_c3, game.cell_r2_c3 , game.cell_r3_c3);
    if validate_winner(col1) || validate_winner(col2) || validate_winner(col3) {
        return  true;
    } 

    // Checking the across
    let across1: String =format!("{}{}{}", game.cell_r1_c1, game.cell_r2_c2 , game.cell_r3_c3);
    let across2: String =format!("{}{}{}", game.cell_r1_c3, game.cell_r2_c2 , game.cell_r3_c1);
    if validate_winner(across1) || validate_winner(across2) {
        return  true;
    } 

    if game.moves_count == 9 {
        game.message = "Players, we have a draw.".to_string();
        return true;

    } else {
        game.message = format!("Player {} turn.", game.player_id.to_string());
    }

    false
}

impl Application for TicTacToe {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Message>) {
        (
            TicTacToe {
                player_id: 1,
                moves_count: 0,
                cell_r1_c1: "".to_string(),
                cell_r1_c2: "".to_string(),
                cell_r1_c3: "".to_string(),
                cell_r2_c1: "".to_string(),
                cell_r2_c2: "".to_string(),
                cell_r2_c3: "".to_string(),
                cell_r3_c1: "".to_string(),
                cell_r3_c2: "".to_string(),
                cell_r3_c3: "".to_string(),
                message: format!("Player {} turn.", "1"),
                game_over: false,
            },
            Command::none())
    }

    fn title(&self) -> String {
        "Custom Theme".into()
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        self.moves_count += 1;
        match message {
            Message::Restart => {
                self.player_id = 1;
                self.moves_count = 0;
                self.cell_r1_c1 = "".to_string();
                self.cell_r1_c2 = "".to_string();
                self.cell_r1_c3 = "".to_string();
                self.cell_r2_c1 = "".to_string();
                self.cell_r2_c2 = "".to_string();
                self.cell_r2_c3 = "".to_string();
                self.cell_r3_c1 = "".to_string();
                self.cell_r3_c2 = "".to_string();
                self.cell_r3_c3 = "".to_string();
                self.message = format!("Player {} turn.", self.player_id.to_string());
                self.game_over = false;
                println!("{:?}", self);
                
            },
            Message::ButtonPress(1) => {
                if self.cell_r1_c1 != "X".to_string() && self.cell_r1_c1 != "O".to_string() && !self.game_over {
                    if self.player_id == 1 {
                        self.cell_r1_c1 = "X".to_string();
                        self.player_id = 2;
                    } else {
                        self.cell_r1_c1 = "O".to_string();
                        self.player_id = 1;
                    }
                    
                }
                self.game_over = check_for_winner(self);
                println!("{:?}", self);
            },
            Message::ButtonPress(2) => {
                if self.cell_r1_c2 != "X".to_string() && self.cell_r1_c2 != "O".to_string() && !self.game_over {
                    if self.player_id == 1 {
                        self.cell_r1_c2 = "X".to_string();
                        self.player_id = 2;
                    } else {
                        self.cell_r1_c2 = "O".to_string();
                        self.player_id = 1;
                    }
                    
                }
                self.game_over = check_for_winner(self);
                println!("{:?}", self);
            },
            Message::ButtonPress(3) => {
                if self.cell_r1_c3 != "X".to_string() && self.cell_r1_c3 != "O".to_string() && !self.game_over {
                    if self.player_id == 1 {
                        self.cell_r1_c3 = "X".to_string();
                        self.player_id = 2;
                    } else {
                        self.cell_r1_c3 = "O".to_string();
                        self.player_id = 1;
                    }
                    
                }
                self.game_over = check_for_winner(self);
                println!("{:?}", self);
            },
            Message::ButtonPress(4) => {
                if self.cell_r2_c1 != "X".to_string() && self.cell_r2_c1 != "O".to_string() && !self.game_over {
                    if self.player_id == 1 {
                        self.cell_r2_c1 = "X".to_string();
                        self.player_id = 2;
                    } else {
                        self.cell_r2_c1 = "O".to_string();
                        self.player_id = 1;
                    }
                    
                }
                self.game_over = check_for_winner(self);
                println!("{:?}", self);
            },
            Message::ButtonPress(5) => {
                if self.cell_r2_c2 != "X".to_string() && self.cell_r2_c2 != "O".to_string() && !self.game_over {
                    if self.player_id == 1 {
                        self.cell_r2_c2 = "X".to_string();
                        self.player_id = 2;
                    } else {
                        self.cell_r2_c2 = "O".to_string();
                        self.player_id = 1;
                    }
                    
                }
                self.game_over = check_for_winner(self);
                println!("{:?}", self);
            },
            Message::ButtonPress(6) => {
                if self.cell_r2_c3 != "X".to_string() && self.cell_r2_c3 != "O".to_string() && !self.game_over {
                    if self.player_id == 1 {
                        self.cell_r2_c3 = "X".to_string();
                        self.player_id = 2;
                    } else {
                        self.cell_r2_c3 = "O".to_string();
                        self.player_id = 1;
                    }
                    
                }
                self.game_over = check_for_winner(self);
                println!("{:?}", self);
            },
            Message::ButtonPress(7) => {
                if self.cell_r3_c1 != "X".to_string() && self.cell_r3_c1 != "O".to_string() && !self.game_over {
                    if self.player_id == 1 {
                        self.cell_r3_c1 = "X".to_string();
                        self.player_id = 2;
                    } else {
                        self.cell_r3_c1 = "O".to_string();
                        self.player_id = 1;
                    }
                    
                }
                self.game_over = check_for_winner(self);
                println!("{:?}", self);
            },
            Message::ButtonPress(8) => {
                if self.cell_r3_c2 != "X".to_string() && self.cell_r3_c2 != "O".to_string() && !self.game_over {
                    if self.player_id == 1 {
                        self.cell_r3_c2 = "X".to_string();
                        self.player_id = 2;
                    } else {
                        self.cell_r3_c2 = "O".to_string();
                        self.player_id = 1;
                    }
                    
                }
                self.game_over = check_for_winner(self);
                println!("{:?}", self);
            },
            Message::ButtonPress(9) => {
                if self.cell_r3_c3 != "X".to_string() && self.cell_r3_c3 != "O".to_string() && !self.game_over {
                    if self.player_id == 1 {
                        self.cell_r3_c3 = "X".to_string();
                        self.player_id = 2;
                    } else {
                        self.cell_r3_c3 = "O".to_string();
                        self.player_id = 1;
                    }
                    
                }
                self.game_over = check_for_winner(self);
                println!("{:?}", self);
            },
            _ => {}
        }
        Command::none()
    }

    fn view(&self) -> Element<Message> {
        container(
            container(
                column![
                    container(
                        row![
                            text(self.message.clone())
                            .width(600)
                            .horizontal_alignment(iced::alignment::Horizontal::Center)
                            .vertical_alignment(iced::alignment::Vertical::Center)
                            .size(50)
                    ]
                        .align_items(iced::Alignment::Center)
                        
                    )
                    .padding(20)
                    .style(theme::Container::Bordered),
                    row![
                        button(
                            text(self.cell_r1_c1.clone())
                            .horizontal_alignment(iced::alignment::Horizontal::Center)
                            .vertical_alignment(iced::alignment::Vertical::Center)
                            .size(200)
                        )
                            .style(theme::Button::Primary)
                            .width(BUTTON_SIZE)
                            .height(BUTTON_SIZE)
                            .on_press(Message::ButtonPress(1)),
                        button(text(self.cell_r1_c2.clone())                            
                            .horizontal_alignment(iced::alignment::Horizontal::Center)
                            .vertical_alignment(iced::alignment::Vertical::Center)
                            .size(200)
                        )
                            .style(theme::Button::Primary)
                            .width(BUTTON_SIZE)
                            .height(BUTTON_SIZE)
                            .on_press(Message::ButtonPress(2)),
                        button(text(self.cell_r1_c3.clone())
                            .horizontal_alignment(iced::alignment::Horizontal::Center)
                            .vertical_alignment(iced::alignment::Vertical::Center)
                            .size(200)
                        )
                            .style(theme::Button::Primary)
                            .width(BUTTON_SIZE)
                            .height(BUTTON_SIZE)
                            .on_press(Message::ButtonPress(3))
                    ]
                        .spacing(10)
                        .padding(10),
                    row![
                        button(text(self.cell_r2_c1.clone())
                            .horizontal_alignment(iced::alignment::Horizontal::Center)
                            .vertical_alignment(iced::alignment::Vertical::Center)
                            .size(200)
                        )
                            .style(theme::Button::Primary)
                            .width(BUTTON_SIZE)
                            .height(BUTTON_SIZE)
                            .on_press(Message::ButtonPress(4)),
                        button(text(self.cell_r2_c2.clone())
                            .horizontal_alignment(iced::alignment::Horizontal::Center)
                            .vertical_alignment(iced::alignment::Vertical::Center)
                            .size(200)
                        )
                            .style(theme::Button::Primary)
                            .width(BUTTON_SIZE)
                            .height(BUTTON_SIZE)
                            .on_press(Message::ButtonPress(5)),
                        button(text(self.cell_r2_c3.clone())
                            .horizontal_alignment(iced::alignment::Horizontal::Center)
                            .vertical_alignment(iced::alignment::Vertical::Center)
                            .size(200)
                        )
                            .width(BUTTON_SIZE)
                            .height(BUTTON_SIZE)
                            .style(theme::Button::Primary)
                            .on_press(Message::ButtonPress(6))
                    ]
                        .spacing(10)
                        .padding(10),
                    row![
                        button(text(self.cell_r3_c1.clone())
                            .horizontal_alignment(iced::alignment::Horizontal::Center)
                            .vertical_alignment(iced::alignment::Vertical::Center)
                            .size(200)
                        )
                            .style(theme::Button::Primary)
                            .width(BUTTON_SIZE)
                            .height(BUTTON_SIZE)
                            .on_press(Message::ButtonPress(7)),
                        button(text(self.cell_r3_c2.clone())
                            .horizontal_alignment(iced::alignment::Horizontal::Center)
                            .vertical_alignment(iced::alignment::Vertical::Center)
                            .size(200)
                        )
                            .style(theme::Button::Primary)
                            .width(BUTTON_SIZE)
                            .height(BUTTON_SIZE)
                            .on_press(Message::ButtonPress(8)),
                        button(text(self.cell_r3_c3.clone())
                            .horizontal_alignment(iced::alignment::Horizontal::Center)
                            .vertical_alignment(iced::alignment::Vertical::Center)
                            .size(200)
                        )
                            .width(BUTTON_SIZE)
                            .height(BUTTON_SIZE)
                            .style(theme::Button::Primary)
                            .on_press(Message::ButtonPress(9))
                    ]
                        .spacing(10)
                        .padding(10),
                    row![
                        button(text("Restart")
                            .size(50)
                            .horizontal_alignment(iced::alignment::Horizontal::Center)
                            .vertical_alignment(iced::alignment::Vertical::Center)

                        )
                            .style(theme::Button::Secondary)                            
                            .width(620)
                            .height(BUTTON_SIZE)
                            .on_press(Message::Restart),
                    ]
                        .spacing(10)
                        .padding(10),

                ],              
            )
            .padding(20)
            .style(theme::Container::Default),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x()
        .center_y()
        .into()
    }
}

// Always import widget types from this module since it
// uses our custom theme instead of the built-in iced::Theme.
// Otherwise you will get compilation errors since iced::Element
// expects use of iced::Theme by default.
mod widget {
    #![allow(dead_code)]
    use crate::theme::Theme;

    pub type Renderer = iced::Renderer<Theme>;
    pub type Element<'a, Message> = iced::Element<'a, Message, Renderer>;
    pub type Container<'a, Message> = iced::widget::Container<'a, Message, Renderer>;
    pub type Button<'a, Message> = iced::widget::Button<'a, Message, Renderer>;
}

mod theme {
    use iced::widget::{button, container, text};
    use iced::{application, color};

    #[derive(Debug, Clone, Copy, Default)]
    pub struct Theme;

    impl application::StyleSheet for Theme {
        type Style = ();

        fn appearance(&self, _style: &Self::Style) -> application::Appearance {
            application::Appearance {
                background_color: color!(0x28, 0x28, 0x28),
                text_color: color!(0xeb, 0xdb, 0xb2),
            }
        }
    }

    impl text::StyleSheet for Theme {
        type Style = ();

        fn appearance(&self, _style: Self::Style) -> text::Appearance {
            text::Appearance {
                color: color!(0xeb, 0xdb, 0xb2).into(),
            }
        }
    }

    #[derive(Debug, Clone, Copy, Default)]
    pub enum Container {
        #[default]
        Default,
        Bordered,
    }

    impl container::StyleSheet for Theme {
        type Style = Container;

        fn appearance(&self, style: &Self::Style) -> container::Appearance {
            match style {
                Container::Default => container::Appearance::default(),
                Container::Bordered => container::Appearance {
                    border_color: color!(0x45, 0x85, 0x88),
                    border_width: 1.0,
                    border_radius: 4.0,
                    ..Default::default()
                },
            }
        }
    }

    #[derive(Debug, Clone, Copy, Default)]
    pub enum Button {
        #[default]
        Primary,
        Secondary,
    }

    impl button::StyleSheet for Theme {
        type Style = Button;

        fn active(&self, style: &Self::Style) -> button::Appearance {
            match style {
                Button::Primary => button::Appearance {
                    background: color!(0x28, 0x28, 0x28).into(),
                    border_radius: 4.0,
                    border_width: 1.0,
                    border_color: color!(0x45, 0x85, 0x88),
                    ..Default::default()
                },
                Button::Secondary => button::Appearance {
                    background: color!(0x3c, 0x38, 0x36).into(),
                    ..Default::default()
                },
            }
        }
    }
}