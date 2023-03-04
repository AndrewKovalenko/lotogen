use std::io::Stdout;

use tui::{
    backend::CrosstermBackend,
    layout::Rect,
    widgets::{Block as TuiBlock, Borders},
    Terminal,
};

struct Block<'a> {
    terminal: &'a mut Terminal<CrosstermBackend<Stdout>>,
}

impl<'a> Block<'a> {
    pub fn new(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Block {
        return Block { terminal };
    }

    fn internal_render(
        &mut self,
        dimensions: Rect,
        title: Option<&str>,
        render: &dyn Fn(&mut Terminal<CrosstermBackend<Stdout>>),
    ) {
        self.terminal
            .draw(|frame| {
                let block = if let Some(title_string) = title {
                    TuiBlock::default()
                        .title(title_string)
                        .borders(Borders::ALL)
                } else {
                    TuiBlock::default().borders(Borders::ALL)
                };

                frame.render_widget(block, dimensions);
            })
            .unwrap();

        render(self.terminal);
    }

    pub fn show_block(
        &mut self,
        dimensions: Rect,
        render: &dyn Fn(&mut Terminal<CrosstermBackend<Stdout>>),
    ) {
        self.internal_render(dimensions, None, render)
    }

    pub fn show_block_with_title(
        &mut self,
        dimensions: Rect,
        title: &str,
        render: &dyn Fn(&mut Terminal<CrosstermBackend<Stdout>>),
    ) {
        self.internal_render(dimensions, Some(title), render)
    }
}
