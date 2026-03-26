use crate::app::{App, Mode};
use crate::utils::pos_to_line_col;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, List, ListItem, Paragraph},
    Frame,
};

pub fn draw(f: &mut Frame, app: &mut App) {
    let size = f.size();

    let vert = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(5)])
        .split(size);

    let horiz = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
        .split(vert[0]);

    if app.mode == Mode::Edit {
        let visible_h = horiz[1].height.saturating_sub(2);
        if visible_h > 0 {
            let (cur_line, _) = pos_to_line_col(&app.edit_buffer, app.edit_cursor);
            let cur_line = cur_line as u16;
            if cur_line < app.edit_scroll {
                app.edit_scroll = cur_line;
            } else if cur_line >= app.edit_scroll + visible_h {
                app.edit_scroll = cur_line.saturating_sub(visible_h).saturating_add(1);
            }
        }
    }

    let sel = app.list_state.selected();
    let list_items: Vec<ListItem> = app
        .items
        .iter()
        .enumerate()
        .map(|(i, item)| {
            let prefix = if sel == Some(i) { "➜ " } else { "  " };
            ListItem::new(Line::from(vec![Span::raw(prefix), Span::raw(&item.name)]))
        })
        .collect();

    let list = List::new(list_items)
        .block(
            Block::default()
                .title(format!(" VAULT ({}) ", app.items.len()))
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(Color::Magenta)),
        )
        .highlight_style(
            Style::default()
                .bg(Color::Magenta)
                .fg(Color::Black)
                .add_modifier(Modifier::BOLD),
        );
    f.render_stateful_widget(list, horiz[0], &mut app.list_state);

    let title = match app.mode {
        Mode::Browse => {
            if app.status.is_empty() {
                " PREVIEW "
            } else {
                &app.status
            }
        }
        Mode::ConfirmDelete => " DELETE CONFIRMATION ",
        Mode::Edit => " EDIT  ·  Ctrl+S: save  ·  Esc: cancel ",
        Mode::Rename => " RENAME  ·  Enter: confirm  ·  Esc: cancel ",
        Mode::SetDbPath => " SET DATABASE PATH  ·  Enter: confirm  ·  Esc: cancel ",
        Mode::SetLogoPath => " SET LOGO PATH  ·  Enter: confirm  ·  Esc: cancel ",
    };

    let border_col = match app.mode {
        Mode::Browse => Color::Cyan,
        Mode::ConfirmDelete => Color::Red,
        Mode::Edit | Mode::Rename => Color::Yellow,
        Mode::SetDbPath | Mode::SetLogoPath => Color::Green,
    };

    let mut para = match app.mode {
        Mode::Browse => {
            if app.items.is_empty() {
                Paragraph::new("Press [N] to create a new entry, or [V] to paste from clipboard.")
            } else {
                let idx = app.selected().min(app.items.len().saturating_sub(1));
                Paragraph::new(app.items[idx].content.as_str())
            }
        }
        Mode::ConfirmDelete => {
            let name = if app.items.is_empty() {
                "Unknown"
            } else {
                &app.items[app.selected()].name
            };
            Paragraph::new(format!(
                "\n\nAre you sure you want to delete '{}'?\n\n[Y / Enter] Yes    [N / Esc] No",
                name
            ))
            .alignment(Alignment::Center)
        }
        Mode::Edit => Paragraph::new(app.edit_buffer.as_str()),
        Mode::Rename => Paragraph::new(Line::from(vec![
            Span::raw("Name: "),
            Span::raw(&app.rename_buffer),
        ])),
        Mode::SetDbPath | Mode::SetLogoPath => Paragraph::new(Line::from(vec![
            Span::raw("Path: "),
            Span::raw(&app.path_buffer),
        ])),
    };

    let scroll = if app.mode == Mode::Edit {
        (app.edit_scroll, 0u16)
    } else {
        (0, 0)
    };

    para = para
        .block(
            Block::default()
                .title(title)
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(border_col)),
        )
        .scroll(scroll);

    f.render_widget(para, horiz[1]);

    match app.mode {
        Mode::Edit => {
            let (line, col) = pos_to_line_col(&app.edit_buffer, app.edit_cursor);
            let screen_row = (line as u16).saturating_sub(app.edit_scroll);
            f.set_cursor(horiz[1].x + 1 + col as u16, horiz[1].y + 1 + screen_row);
        }
        Mode::Rename => f.set_cursor(
            horiz[1].x + 1 + 6 + app.rename_buffer.chars().count() as u16,
            horiz[1].y + 1,
        ),
        Mode::SetDbPath | Mode::SetLogoPath => f.set_cursor(
            horiz[1].x + 1 + 6 + app.path_buffer.chars().count() as u16,
            horiz[1].y + 1,
        ),
        _ => {}
    }

    let cmd = match app.mode {
        Mode::Browse => {
            " [N] New  [V] Paste  [C] Copy  [E] Edit  [R] Rename  [D] Delete  [I] Set Logo\n \
             [P] Set DB Path  [L] Set Logo Path\n \
             [↑/↓] Navigate  [Shift+↑ / Shift+↓] Move item Up/Down  [Q] Quit"
        }
        Mode::Edit => {
            " [Ctrl+S] Save  [Esc] Cancel\n \
             [←/→] Char  [↑/↓] Line  [Home/End] Line start/end  [Ctrl+A] Start  [Ctrl+E] End\n \
             [Backspace] Delete back  [Delete] Delete forward"
        }
        Mode::Rename | Mode::SetDbPath | Mode::SetLogoPath => {
            " [Enter] Confirm  [Esc] Cancel  [Backspace] Delete\n\n"
        }
        Mode::ConfirmDelete => " [Y / Enter] Confirm Delete  [N / Esc] Cancel\n\n",
    };

    let cmd_para = Paragraph::new(cmd).block(
        Block::default()
            .title(" COMMANDS ")
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(Color::DarkGray)),
    );
    f.render_widget(cmd_para, vert[1]);
}
