use iced::widget::Text;
use iced::{executor, Application, Command, Length, Settings};
use widget::{Row, Column, Renderer, Button, Container};

use self::theme::Theme;
use self::widget::Element;

const BUTTON_SIZE: u16 = 200;
const TEXT_SIZE: u16 = ((BUTTON_SIZE as f64) * 0.8) as u16;

fn main() {
    let settings = Settings {
        window: iced::window::Settings {
            resizable: true,
            size: (1200, 1200),
            ..Default::default()
        },
        ..Default::default()
    };

    TicTacToe::run(settings).unwrap();
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Player {
    X,
    O,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum CellState {
    Empty,
    Occupied(Player),
}

#[derive(Debug)]
struct TicTacToe {
    player: Player,
    moves_count: usize,
    cells: [[CellState; 3]; 3],
    message: String,
    game_over: bool,
}

impl Default for TicTacToe {
    fn default() -> Self {
        TicTacToe {
            player: Player::X,
            moves_count: 0,
            cells: [[CellState::Empty; 3]; 3],
            message: "Player X's turn.".to_string(),
            game_over: false,
        }
    }
}

impl TicTacToe {
    fn validate_winner(cells: &[[CellState; 3]; 3]) -> bool {
        let winning_combinations = [
            // Rows
            [(0, 0), (0, 1), (0, 2)],
            [(1, 0), (1, 1), (1, 2)],
            [(2, 0), (2, 1), (2, 2)],
            // Columns
            [(0, 0), (1, 0), (2, 0)],
            [(0, 1), (1, 1), (2, 1)],
            [(0, 2), (1, 2), (2, 2)],
            // Diagonals
            [(0, 0), (1, 1), (2, 2)],
            [(0, 2), (1, 1), (2, 0)],
        ];

        for combination in &winning_combinations {
            let (row1, col1) = combination[0];
            let (row2, col2) = combination[1];
            let (row3, col3) = combination[2];

            if let CellState::Occupied(player) = cells[row1][col1] {
                if cells[row2][col2] == CellState::Occupied(player)
                    && cells[row3][col3] == CellState::Occupied(player)
                {
                    return true;
                }
            }
        }

        false
    }

    fn check_for_winner(&mut self) -> bool {
        if self.game_over {
            return true;
        }

        if TicTacToe::validate_winner(&self.cells) {
            self.message = format!("Player {} is the winner!!!!", match self.player {
                Player::X => "O",
                Player::O => "X",
            });
    
            self.game_over = true;
            return true;
        }

        if self.moves_count == 9 {
            self.message = "Players, we have a draw.".to_string();
            self.game_over = true;
            return true;
        }
        self.message = format!("Player {}'s turn.", match self.player {
            Player::X => "X",
            Player::O => "O",
        });
        false
    }
}

#[derive(Debug, Clone, Copy)]
enum Message {
    ButtonPress(usize),
    Restart,
}

impl Application for TicTacToe {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();
    type Theme = Theme;

    fn new(_flags: Self::Flags) -> (Self, Command<Message>) {
        (TicTacToe::default(), Command::none())
    }

    fn title(&self) -> String {
        "Tic Tac Toe".into()
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::Restart => {
                *self = TicTacToe::default();
            }
            Message::ButtonPress(index) => {
                let row = (index - 1) / 3;
                let col = (index - 1) % 3;

                if self.cells[row][col] == CellState::Empty && !self.game_over {
                    let cell_state = match self.player {
                        Player::X => CellState::Occupied(Player::X),
                        Player::O => CellState::Occupied(Player::O),
                    };
                    self.cells[row][col] = cell_state;
                    self.player = match self.player {
                        Player::X => Player::O,
                        Player::O => Player::X,
                    };
                    self.moves_count += 1;

                    self.check_for_winner();
                }
            }
        }

        Command::none()
    }

    fn view(&self) -> Element<Message> {
        let board_button = |state: &CellState, index: usize| -> Button<'_, Message, Renderer> {
            let bt_text = match state {
                CellState::Occupied(Player::X) => "X",
                CellState::Occupied(Player::O) => "O",
                _ => "",
            };
    
            Button::new(
                Text::new(bt_text)
                    .horizontal_alignment(iced::alignment::Horizontal::Center)
                    .vertical_alignment(iced::alignment::Vertical::Center)
                    .size(TEXT_SIZE),
            )
            .width(BUTTON_SIZE)
            .height(BUTTON_SIZE)
            .on_press(Message::ButtonPress(index))
        };
    
        let restart_button = Button::new(
            Text::new("Restart")
                .horizontal_alignment(iced::alignment::Horizontal::Center)
                .vertical_alignment(iced::alignment::Vertical::Center)
                .size(TEXT_SIZE),
        )
        .width(620)
        .height(BUTTON_SIZE)
        .on_press(Message::Restart);
    
        let message = Text::new(&self.message)
            .horizontal_alignment(iced::alignment::Horizontal::Center)
            .vertical_alignment(iced::alignment::Vertical::Center)
            .size(30);
    
        let cells: Vec<Element<Message>> = self.cells.iter().enumerate().map(|(row, row_cells)| {
            let row_buttons: Vec<Element<Message>> = row_cells.iter().enumerate().map(|(col, cell_state)| {
                let button = board_button(cell_state, row * 3 + col + 1);
                button.into() // Convert Button to Element
            }).collect();
    
            Row::new()
                .spacing(10)
                .align_items(iced::Alignment::Center)
                .push(Row::with_children(row_buttons)) // Wrap the buttons in another Row
                .into() // Convert Row to Element
        }).collect();
    
        let content = Column::new()
            .spacing(20)
            .align_items(iced::Alignment::Center)
            .push(message)
            .push(Column::with_children(cells)) // Convert Vec<Element> to a single widget element
            .push(restart_button);
    
        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
    

}

mod widget {
    #![allow(dead_code)]
    use crate::theme::Theme;

    pub type Renderer = iced::Renderer<Theme>;
    pub type Element<'a, Message> = iced::Element<'a, Message, Renderer>;
    pub type Container<'a, Message> = iced::widget::Container<'a, Message, Renderer>;
    pub type Button<'a, Message, Renderer> = iced::widget::Button<'a, Message, Renderer>;
    pub type Column<'a, Message, Renderer> = iced::widget::Column<'a, Message, Renderer>;
    pub type Row<'a, Message, Renderer> = iced::widget::Row<'a, Message, Renderer>;
}

mod theme {
    use iced::widget::{button, container, text};
    use iced::{application, color, Background, BorderRadius, Color};

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
                    border_radius: BorderRadius::from(4.0),
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
                    //background: Some(Background::from(Color::from_rgb(40.0, 40.0, 40.0))),
                    border_radius: BorderRadius::from(4.0),
                    border_width: 1.0,
                    border_color: color!(0x45, 0x85, 0x88),
                    ..Default::default()
                },
                Button::Secondary => button::Appearance {
                    background: Some(Background::from(Color::from_rgb(60.0, 56.0, 54.0))),
                    ..Default::default()
                },
            }
        }
    }
}
