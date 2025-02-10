use crossterm::event::KeyCode;
use ratatui::{
    layout::{Alignment, Margin, Rect},
    style::{Color, Style, Stylize},
    widgets::{Block, Clear},
    Frame,
};

pub trait StatefulModal {
    type Message;

    fn title(&self) -> &str;
    fn draw_inner(&self, area: Rect, ctx: &mut Frame);

    fn key_press(&mut self, _key: KeyCode) -> Option<Self::Message> {
        None
    }

    fn paste_clipboard_content(&mut self, _content: String) {}

    fn area(&self, master_area: Rect) -> Rect {
        master_area.inner(Margin::new(6, 6))
    }

    fn draw(&self, ctx: &mut Frame) {
        let block = Block::bordered()
            .bg(Color::Black)
            .border_style(Style::new().yellow())
            .title(self.title())
            .title_alignment(Alignment::Left);

        let block_area = self.area(ctx.area());
        let block_inner_area = block.inner(block_area);

        ctx.render_widget(Clear, block_area);
        ctx.render_widget(block, block_area);

        self.draw_inner(block_inner_area, ctx);
    }
}
