use chess::{
    ChessPieceNames::{self, *},
    ChessPlayerColor, ChessSquareColor,
};
use iced::{color, Theme};
use iced_style::svg;
use std::collections::HashMap;
pub fn match_chesspiece_svgs() -> HashMap<ChessPieceSvg, String> {
    let mut svg = HashMap::new();
    for square_color in [ChessSquareColor::Black, ChessSquareColor::White] {
        for piece_color in [ChessPlayerColor::Black, ChessPlayerColor::White] {
            for piece_name in [Rook, Bishop, King, Pawn, Queen, Knight] {
                let square_color_str = match square_color {
                    ChessSquareColor::White => "t",
                    ChessSquareColor::Black => "t",
                };
                let piece_color_str = match piece_color {
                    ChessPlayerColor::Black => "d",
                    ChessPlayerColor::White => "l",
                };
                let piece_name_str = match piece_name {
                    King => ("k", "f"),
                    Queen => ("q", "g"),
                    Rook => ("r", "m"),
                    Bishop => ("b", "e"),
                    Knight => ("n", "s"),
                    Pawn => ("p", "h"),
                };
                let path_rotated = format!(
                    "Chess_{}{}{}45.svg",
                    piece_name_str.0, piece_color_str, square_color_str
                );
                let path_normal = format!(
                    "Chess_{}{}{}45.svg",
                    piece_name_str.1, piece_color_str, square_color_str
                );
                svg.insert(
                    ChessPieceSvg::new(piece_name, square_color, piece_color, true),
                    path_rotated,
                );
                svg.insert(
                    ChessPieceSvg::new(piece_name, square_color, piece_color, false),
                    path_normal,
                );
            }
        }
    }
    svg
}
#[derive(Hash, PartialEq, Eq)]
pub struct ChessPieceSvg {
    piece_name: ChessPieceNames,
    square_color: ChessSquareColor,
    piece_color: ChessPlayerColor,
    rotated: bool,
}
pub enum ChessPieceSvgOption {
    Occoupied { piece_svg: ChessPieceSvg },
    Empty { square_color: ChessSquareColor },
}

// rook m r
// bishop b e
// king f k
// queen g q
// knigh s n
// pawn h p
impl ChessPieceSvg {
    pub fn new(
        piece_name: ChessPieceNames,
        square_color: ChessSquareColor,
        piece_color: ChessPlayerColor,
        rotated: bool,
    ) -> Self {
        ChessPieceSvg {
            piece_name,
            square_color,
            piece_color,
            rotated,
        }
    }
}
#[derive(Default)]
pub enum ChessPieceSvgStyle {
    #[default]
    Normal,
}
impl svg::StyleSheet for ChessPieceSvgStyle {
    type Style = Theme;

    fn appearance(&self, _style: &Self::Style) -> svg::Appearance {
        svg::Appearance {
            color: color!(0x9E_C3_01).into(),
        }
    }
}
