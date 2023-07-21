use crate::*;

fn start<'a>() -> Column<'a, Message, Renderer> {
    Column::new().push(text("hi"))
}
