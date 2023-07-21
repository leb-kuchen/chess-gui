use iced::widget::column as col;
use crate::*;
use iced_aw::ColorPicker;
use crate::{ChessGame, Message};
impl ChessGame {
fn color_picker<'a>(&self) -> Column<'a, Message, Renderer> {
        let color_button = button(text("Farbe ändern")).on_press(Message::ChooseColor);
       let color_picker = ColorPicker::new(
            self.color.show_picker,
            self.color.color,
            color_button,
            Message::CancelColor,
            Message::SubmitColor,
        );
        
        col!(color_picker)
        
    
}
pub fn settings<'a>(&self) -> Container<'a, Message, Renderer> {
    container(self.color_picker())
}
}

