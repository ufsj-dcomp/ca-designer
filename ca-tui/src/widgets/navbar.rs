use ratatui::{
    layout::Rect,
    style::{Style, Stylize},
    text::{Line, Span},
    Frame,
};

pub struct Navbar;

impl Navbar {
    pub fn draw(controls: &[(&str, &str)], area: Rect, ctx: &mut Frame) {
        let key_style = Style::new().black().on_gray().bold();
        let desc_style = Style::new().gray().on_black();

        let spans = controls.iter().flat_map(|(key, desc)| {
            let key = Span::styled(*key, key_style);
            let desc = Span::styled(*desc, desc_style);
            [key, desc]
        });

        ctx.render_widget(Line::from_iter(spans).centered(), area);
    }
}
