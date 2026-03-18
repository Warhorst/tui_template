use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    Frame,
    layout::{Constraint, Layout},
    prelude::Rect,
    style::{Color, Style},
    symbols::border,
    text::Line,
    widgets::{Block, Cell, Row, Table, TableState},
};
use ratatui_tools::Component;

const HIGHLIGHT_STYLE: Style = Style::new().fg(Color::Green);

pub fn main() -> std::io::Result<()> {
    ratatui::run(|terminal| {
        let mut state = State::default();
        let mut app = App::default();
        let mut focus = Focus::MainPanel;

        while !state.should_exit {
            terminal.draw(|frame| {
                app.render(frame, frame.area(), &state, &focus);
            })?;

            let message_opt = app.handle_event(event::read()?, &state, &mut focus);

            if let Some(message) = message_opt {
                match message {
                    Message::Close => state.should_exit = true,
                }
            }
        }

        Ok(())
    })
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

/// The current focused component of the app.
enum Focus {
    MainPanel,
    SettingsPanel,
}

/// The TUI app main component
#[derive(Default)]
struct App {
    /// sub panel A
    main_panel: MainPanel,
    /// sub panel B
    setting_panel: SettingPanel,
}

impl Component<State, Focus, Message> for App {
    fn render(
        &mut self,
        frame: &mut Frame,
        area: Rect,
        _state: &State,
        focus: &Focus,
    ) {
        let block = Block::bordered()
            .title_top(Line::from(" {{ crate_name }} ").centered())
            .border_set(border::THICK);
        let inner_area = block.inner(area);

        let [panel_a_area, panel_b_area] =
            Layout::horizontal([Constraint::Percentage(70), Constraint::Percentage(30)])
                .areas(inner_area);

        frame.render_widget(block, area);

        self.main_panel.render(frame, panel_a_area, &(), focus);
        self.setting_panel.render(frame, panel_b_area, &(), focus);
    }

    fn handle_event(
        &mut self,
        event: Event,
        _state: &State,
        focus: &mut Focus,
    ) -> Option<Message> {
        if let Event::Key(key_event) = event
            && key_event.kind == KeyEventKind::Press
        {
            match key_event.code {
                KeyCode::Esc => return Some(Message::Close),
                KeyCode::Char('1') => {
                    *focus = Focus::MainPanel;
                    return None;
                }
                KeyCode::Char('2') => {
                    *focus = Focus::SettingsPanel;
                    return None;
                }
                // Handle messages from the sub widgets here
                _ => return self.setting_panel.handle_event(event, &(), focus),
            }
        }

        None
    }
}

#[derive(Default)]
struct MainPanel;

impl Component<(), Focus, ()> for MainPanel {
    fn render(
        &mut self,
        frame: &mut Frame,
        area: Rect,
        _state: &(),
        focus: &Focus,
    ) {
        let style = if matches!(focus, Focus::MainPanel) {
            HIGHLIGHT_STYLE
        } else {
            Style::new()
        };

        let block = Block::bordered()
            .title(Line::from(" Main Panel ").centered())
            .border_style(style);

        frame.render_widget(block, area);
    }
}

struct SettingPanel {
    table_state: TableState,
}

impl Default for SettingPanel {
    fn default() -> Self {
        SettingPanel {
            table_state: TableState::new().with_selected(Some(0)),
        }
    }
}

impl SettingPanel {
    const NUM_ROWS: usize = 3;
}

impl Component<(), Focus, Message> for SettingPanel {
    fn render(
        &mut self,
        frame: &mut Frame,
        area: Rect,
        _state: &(),
        focus: &Focus,
    ) {
        let style = if matches!(focus, Focus::SettingsPanel) {
            HIGHLIGHT_STYLE
        } else {
            Style::new()
        };

        let block = Block::bordered()
            .title(Line::from(" Settings ").centered())
            .border_style(style);

        let inner_area = block.inner(area);
        frame.render_widget(block, area);

        let rows = [
            Row::new([Cell::new("Setting A")]),
            Row::new([Cell::new("Setting B")]),
            Row::new([Cell::new("Setting C")]),
        ];

        let widths = [Constraint::Percentage(100)];

        let table = Table::new(rows, widths).row_highlight_style(style);
        frame.render_stateful_widget(table, inner_area, &mut self.table_state);
    }

    fn handle_event(
        &mut self,
        event: Event,
        _state: &(),
        focus: &mut Focus,
    ) -> Option<Message> {
        if !matches!(focus, Focus::SettingsPanel) {
            return None;
        }

        if let Event::Key(key_event) = event
            && key_event.kind == KeyEventKind::Press
        {
            match key_event.code {
                // Widget state is updated here directly. App state can be manipulated by sending messages.
                KeyCode::Char('j') => {
                    if let Some(index) = self.table_state.selected()
                        && index < Self::NUM_ROWS - 1
                    {
                        self.table_state.select(Some(index + 1))
                    }
                }
                KeyCode::Char('k') => {
                    if let Some(index) = self.table_state.selected()
                        && index > 0
                    {
                        self.table_state.select(Some(index - 1))
                    }
                }
                _ => {}
            }
        }

        None
    }
}
