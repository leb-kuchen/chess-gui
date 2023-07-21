mod chesspiece_svg;
mod settings;
mod start;
mod game;
#[macro_use]
extern crate lazy_static;
lazy_static! {
    static ref CHESSPIECE_SVG: HashMap<ChessPieceSvg, String> =
        chesspiece_svg::match_chesspiece_svgs();
}

use chess::{ChessBoard, ChessPlayerColor, ChessSquareColor, ChessSquareCoordinates};
use chesspiece_svg::ChessPieceSvgStyle;

pub(crate) use iced::alignment::{Horizontal, Vertical};

pub (crate) use iced::{color, subscription, theme, time, window, Alignment, Color, Event, Vector};
pub(crate) use iced::{
    executor,
    Renderer,
    widget::{button, column, container, row, svg::Svg, text, Column, Container},
    Application, Command, Length, Settings, Theme,
};

use rand::Rng;
use std::collections::HashMap;
use std::path::Path;
use std::time::{Duration, Instant};

use crate::chesspiece_svg::{ChessPieceSvg, ChessPieceSvgOption};

fn main() -> iced::Result {
    ChessGame::run(Settings {
        ..Default::default()
    })
}
struct ChessGame {
    chess_board: ChessBoard,
    player1: ChessPlayer,
    player2: ChessPlayer,
    stopwatch: Stopwatch,
    min_win: u16,
    page: Page,
    color: ColorDisplay,
}
#[derive(Debug, Clone, Copy, Default)]
pub enum Page {
    #[default]
    Start,
    Game,
    Settings,
}

enum State {
    Idle,
    Ticking { last_tick: Instant },
}
struct ChessPlayer {
    color: ChessPlayerColor,
    timer: Stopwatch,
}

impl ChessPlayer {
    fn new(color: ChessPlayerColor, timer: Duration) -> Self {
        ChessPlayer {
            color,
            timer: Stopwatch::new(timer),
        }
    }
}
impl Stopwatch {
    fn new(duration: Duration) -> Self {
        Stopwatch {
            duration,
            state: State::Idle,
            timer: Duration::from_secs(5 * 60),
        }
    }
}
struct ColorDisplay {
    color: Color,
    show_picker: bool,
}
impl ColorDisplay {
    fn new() -> Self {
        Self {
            color: Self::random_color(),
            show_picker: false,
        }
    }
    fn random_color() -> Color {
        color!(rand::thread_rng().gen_range(0x0..0xFFFFFF))
    }
}
impl Default for ColorDisplay {
    fn default() -> Self {
        Self::new()
    }
}

struct Stopwatch {
    duration: Duration,
    state: State,
    timer: Duration,
}
#[derive(Debug, Clone)]
pub enum Message {
    ChessSquareClicked(ChessSquareCoordinates),
    Tick(Instant),
    TogglePlayer,
    EventOccured(Event),
    PageChanged(Page),
    CancelColor,
    SubmitColor(Color),
    ChooseColor,
}

impl Application for ChessGame {
    type Executor = executor::Default;

    type Message = Message;

    type Theme = Theme;

    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        let timer = Duration::from_secs(60);
        (
            ChessGame {
                chess_board: ChessBoard::new(),
                player1: ChessPlayer::new(ChessPlayerColor::Black, timer),
                player2: ChessPlayer::new(ChessPlayerColor::White, timer),
                stopwatch: Stopwatch::new(timer),
                min_win: 50,
                page: Page::default(),
                color: ColorDisplay::new(),
            },
            //  window::change_mode(iced::window::Mode::)
            Command::none(),
        )
    }

    fn title(&self) -> String {
        "Schach".into()
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        match message {
            Message::ChessSquareClicked(square) => println!("{}", square),
            Message::Tick(now) => {
                if let State::Ticking { last_tick } = &mut self.stopwatch.state {
                    if self.stopwatch.duration.is_zero() {
                        self.stopwatch.state = State::Idle
                    } else {
                        self.stopwatch.duration = self
                            .stopwatch
                            .duration
                            .saturating_sub(now.saturating_duration_since(*last_tick));
                        *last_tick = now;
                    }
                }
            }
            Message::TogglePlayer => {
                if let State::Idle = self.stopwatch.state {
                    self.stopwatch.state = State::Ticking {
                        last_tick: (Instant::now()),
                    }
                }
            }
            Message::EventOccured(event) => {
                println!("{event:#?}");

                if let Event::Window(window) = event {
                    if let window::Event::Resized { width, height } = window {
                        self.min_win = std::cmp::min(width as u16, height as u16) / 9
                    }
                }
            }
            Message::PageChanged(page) => self.page = page,
            Message::CancelColor => self.color.show_picker = false,
            Message::SubmitColor(color) => {
                self.color.color = color;
                self.color.show_picker = false;
            }
            Message::ChooseColor => self.color.show_picker = true,
        }
        Command::none()
    }

    fn view(&self) -> iced::Element<'_, Self::Message, iced::Renderer<Self::Theme>> {
        //   let svg_paths = CHESSPIECE_
        /*   let start_button = button(text("Starten").horizontal_alignment(Horizontal::Center))
            .on_press(Message::TogglePlayer)
            .padding(10)
            .width(80);

        */

        let page_choice_fn = |page_choice: &[Page]| {
            page_choice
                .iter()
                .fold(Column::new(), |acc, x| {
                    acc.push(
                        button(
                            text(format!("{:?}", x))
                                .horizontal_alignment(Horizontal::Center)
                                .vertical_alignment(Vertical::Center)
                                //  .width(Length::Fill)
                                //.height(Length::Fill)
                                .size(50),
                        )
                        .on_press(Message::PageChanged(*x))
                        .style(theme::Button::Primary),
                    )
                    .max_width(150)
                    .padding([5, 0])
                })
                .spacing(2)
                .align_items(Alignment::Center)
        };
        let page_choice = page_choice_fn(&[Page::Game, Page::Settings]);

        let Stopwatch { duration, .. } = self.stopwatch;
        let _remaing_time = duration;
        //  let message = text(format!("Noch: {}", remaing_time.as_secs()))
        //    .horizontal_alignment(Horizontal::Center)
        //  .size(50);
        //    let row = column![start_button, message]
        //     .align_items(Alignment::Center)
        //   .spacing(20);
        let drawn_board = self.draw_board(self.min_win);
        // let board = text(self.chess_board.to_string());
        //  let content = column![drawn_board, page_choice].spacing(0).padding(0);
        //.align_items(Alignment::Center);
        // .spacing(20);
        let content = match self.page {
            Page::Start => container(column!(page_choice)),
            Page::Game => container(column!(drawn_board)),
            Page::Settings => self.settings()
        };
        content
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }

    fn theme(&self) -> Self::Theme {
        Theme::Dark
    }

    fn style(&self) -> <Self::Theme as iced::application::StyleSheet>::Style {
        <Self::Theme as iced::application::StyleSheet>::Style::default()
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        if let State::Ticking { .. } = self.stopwatch.state {
            time::every(Duration::from_millis(100)).map(Message::Tick)
        } else {
            subscription::events().map(Message::EventOccured)
        }
    }

    fn scale_factor(&self) -> f64 {
        1.0
    }
}
#[derive(Debug, Clone, Copy, Default)]
pub enum ChessSquareColorImpl {
    #[default]
    White,
    Black,
}
impl button::StyleSheet for ChessSquareColorImpl {
    type Style = Theme;
    fn active(&self, _style: &Self::Style) -> button::Appearance {
        /*
            button::Appearance {
                 shadow_offset: (),
                 background: (),
                  border_radius:
                 (), border_width: (),
                 border_color: (),
                 text_color: () }

        */
        button::Appearance {
            shadow_offset: Vector::ZERO,
            background: Color::BLACK.into(),
            border_radius: 0.0,
            border_width: 0.0,
            ..Default::default()
        }
    }
}
