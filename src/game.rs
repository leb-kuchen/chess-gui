use crate::ChessGame;
use chess::{ChessPlayerColor, ChessSquareColor};
use chesspiece_svg::ChessPieceSvgStyle;
use crate::chesspiece_svg;

use crate::ChessSquareColorImpl;
use crate::*;
use self::column;

use std::collections::HashMap;
use std::path::Path;

lazy_static! {
    static ref CHESSPIECE_SVG: HashMap<ChessPieceSvg, String> =
        chesspiece_svg::match_chesspiece_svgs();
}


use crate::chesspiece_svg::{ChessPieceSvg, ChessPieceSvgOption};
impl ChessGame {
pub fn draw_board<'a>(&self, width: u16) -> Column<'a, Message, Renderer> {
            return self.chess_board
                .board_vec
                .iter()
                .fold(column!(), |row, col| {
                    let new_row = col.iter().fold(row!(), |row_in, col_in| {
                        let piece_option = self.chess_board.squares_map[&col_in.coordinates];
                        let svg_option = match piece_option {
                            Some(piece) => {
                                let piece_svg = match piece.player_color {
                                    ChessPlayerColor::Black => ChessPieceSvg::new(
                                        piece.name,
                                        col_in.color,
                                        piece.player_color,
                                        false,
                                    ),
                                    ChessPlayerColor::White => ChessPieceSvg::new(
                                        piece.name,
                                        col_in.color,
                                        piece.player_color,
                                        true,
                                    ),
                                };

                                ChessPieceSvgOption::Occoupied { piece_svg }
                            }
                            None => ChessPieceSvgOption::Empty {
                                square_color: col_in.color,
                            },
                        };
                        row_in.push(
                            button(
                                {
                                    // text(col_in.coordinates.to_string())

                                    let path = match svg_option {
                                        ChessPieceSvgOption::Occoupied { piece_svg } => {
                                            &CHESSPIECE_SVG[&piece_svg]
                                        }
                                        ChessPieceSvgOption::Empty { square_color: _ } => {
                                            "Chess_bdd45.svg"
                                        }
                                    };
                                    //   println!("{path}");
                                    container(
                                        Svg::from_path(Path::new("img").join(path))
                                            .width(Length::Fill)
                                            .height(Length::Fill)
                                            .style(theme::Svg::Custom(Box::new(
                                                ChessPieceSvgStyle::default(),
                                            ))),
                                    )
                                    .width(width)
                                    .height(width)
                                    .padding(0)
                                }, //    Svg::from_path("img/Chess_bdd45.svg")
                                   //    .vertical_alignment(Vertical::Center)
                                   //  .horizontal_alignment(Horizontal::Center),
                            )
                            .on_press(Message::ChessSquareClicked(col_in.coordinates))
                            .padding(0)
                            //   .height(100)
                            //  .height(Length::Fill)
                            .style(match col_in.color {
                                ChessSquareColor::Black => {
                                    theme::Button::Custom(Box::new(ChessSquareColorImpl::Black))
                                }
                                ChessSquareColor::White => {
                                    theme::Button::Custom(Box::new(ChessSquareColorImpl::White))
                                }
                            }), //        .style(theme::Button::Custom(Theme::Dark))
                        )
                    });
                    row.push(new_row)
                })
        }
pub fn game(&self) {
    
}
}