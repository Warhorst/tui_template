use crossterm::event::{Event, KeyCode, KeyEvent, read};
use ratatui::{
    DefaultTerminal,
    layout::{Constraint, Layout},
    prelude::{Buffer, Rect},
    style::{Color, Style},
    symbols::border,
    text::Line,
    widgets::{Block, Cell, Row, StatefulWidget, Table, TableState, Widget},
};
use ratatui_tools::Messages;

const HIGHLIGHT_STYLE: Style = Style::new().fg(Color::Green);

pub fn main() -> std::io::Result<()> {
    ratatui::run(|t| App::default().run(t))
}

/// The whole state of the app itself
#[derive(Default)]
struct State {
    should_exit: bool,
}

/// A message which causes an app state change
#[derive(Default)]
enum Message {
    #[default]
    Close,
}

/// The TUI app
#[derive(Default)]
struct App {
    /// The current app state
    state: State,
    /// sub panel A
    main_panel: MainPanel,
    /// sub panel B
    setting_panel: SettingPanel,
}

impl App {
    /// Main loop which renders the app and handles events
    fn run(
        &mut self,
        terminal: &mut DefaultTerminal,
    ) -> std::io::Result<()> {
        while !self.state.should_exit {
            terminal.draw(|frame| frame.render_widget(&mut *self, frame.area()))?;

            if let Event::Key(key) = read()?
                && key.is_press()
            {
                for message in self.handle_events(key) {
                    match message {
                        Message::Close => self.state.should_exit = true,
                    }
                }
            }
        }

        Ok(())
    }

    fn handle_events(
        &mut self,
        event: KeyEvent,
    ) -> Messages<Message> {
        match event.code {
            KeyCode::Esc => Messages::single(Message::Close),
            KeyCode::Char('1') => {
                self.main_panel.selected = true;
                self.setting_panel.selected = false;
                Messages::none()
            }
            KeyCode::Char('2') => {
                self.setting_panel.selected = true;
                self.main_panel.selected = false;
                Messages::none()
            }
            // Handle messages from the sub widgets here
            _ => Messages::from_messages([self.setting_panel.handle_events(event)]),
        }
    }
}

impl Widget for &mut App {
    fn render(
        self,
        area: Rect,
        buf: &mut Buffer,
    ) {
        let block = Block::bordered()
            .title_top(Line::from(" {{ crate_name }} ").centered())
            .border_set(border::THICK);
        let inner_area = block.inner(area);

        let [panel_a_area, panel_b_area] =
            Layout::horizontal([Constraint::Percentage(70), Constraint::Percentage(30)])
                .areas(inner_area);

        block.render(area, buf);

        self.main_panel.render(panel_a_area, buf, &mut self.state);
        self.setting_panel
            .render(panel_b_area, buf, &mut self.state);
    }
}

#[derive(Default)]
struct MainPanel {
    selected: bool,
}

impl StatefulWidget for &mut MainPanel {
    type State = State;

    fn render(
        self,
        area: Rect,
        buf: &mut Buffer,
        _state: &mut Self::State,
    ) {
        let style = if self.selected {
            HIGHLIGHT_STYLE
        } else {
            Style::new()
        };

        let block = Block::bordered()
            .title(Line::from(" Main Panel ").centered())
            .border_style(style);

        block.render(area, buf);
    }
}

struct SettingPanel {
    selected: bool,
    table_state: TableState,
}

impl Default for SettingPanel {
    fn default() -> Self {
        SettingPanel {
            selected: false,
            table_state: TableState::new().with_selected(Some(0)),
        }
    }
}

impl SettingPanel {
    const NUM_ROWS: usize = 3;

    fn handle_events(
        &mut self,
        event: KeyEvent,
    ) -> Messages<Message> {
        if !self.selected {
            return Messages::none();
        }

        match event.code {
            // Widget state is updated here directly. App state can be manipulated by sending messages.
            KeyCode::Char('j') => {
                if let Some(index) = self.table_state.selected()
                    && index < Self::NUM_ROWS - 1
                {
                    self.table_state.select(Some(index + 1))
                }
                Messages::none()
            }
            KeyCode::Char('k') => {
                if let Some(index) = self.table_state.selected()
                    && index > 0
                {
                    self.table_state.select(Some(index - 1))
                }
                Messages::none()
            }
            _ => Messages::none(),
        }
    }
}

impl StatefulWidget for &mut SettingPanel {
    type State = State;

    fn render(
        self,
        area: Rect,
        buf: &mut Buffer,
        _state: &mut Self::State,
    ) {
        let style = if self.selected {
            HIGHLIGHT_STYLE
        } else {
            Style::new()
        };

        let block = Block::bordered()
            .title(Line::from(" Settings ").centered())
            .border_style(style);

        let inner_area = block.inner(area);
        block.render(area, buf);

        let rows = [
            Row::new([Cell::new("Setting A")]),
            Row::new([Cell::new("Setting B")]),
            Row::new([Cell::new("Setting C")]),
        ];

        let widths = [Constraint::Percentage(100)];

        let table = Table::new(rows, widths).row_highlight_style(style);
        StatefulWidget::render(table, inner_area, buf, &mut self.table_state);
    }
}
