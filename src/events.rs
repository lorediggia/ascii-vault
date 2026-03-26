use crate::app::{App, AsciiItem, Mode};
use crate::utils::*;
use crossterm::event::{self, Event, KeyCode, KeyModifiers};
use std::fs;
use std::io::Result;

pub fn handle_input(app: &mut App) -> Result<bool> {
    if let Event::Key(key) = event::read()? {
        match app.mode {
            Mode::Browse => match key.code {
                KeyCode::Char('q') => return Ok(true),
                KeyCode::Char('n') => {
                    app.items.push(AsciiItem {
                        name: "New".to_string(),
                        content: String::new(),
                    });
                    let idx = app.items.len().saturating_sub(1);
                    app.list_state.select(Some(idx));
                    app.edit_buffer.clear();
                    app.edit_cursor = 0;
                    app.edit_scroll = 0;
                    app.status.clear();
                    app.mode = Mode::Edit;
                }
                KeyCode::Char('v') => {
                    let pasted = get_clipboard();
                    app.items.push(AsciiItem {
                        name: "Pasted".to_string(),
                        content: pasted,
                    });
                    save_data(&app.items, &app.config.db_file);
                    app.list_state
                        .select(Some(app.items.len().saturating_sub(1)));
                    app.status = " Pasted from clipboard ".to_string();
                }
                KeyCode::Char('c') if !app.items.is_empty() => {
                    copy_to_clipboard(&app.items[app.selected()].content);
                    app.status = " Copied to clipboard! ".to_string();
                }
                KeyCode::Char('e') if !app.items.is_empty() => {
                    let idx = app.selected();
                    app.edit_buffer = app.items[idx].content.clone();
                    app.edit_cursor = app.edit_buffer.len();
                    app.edit_scroll = 0;
                    app.status.clear();
                    app.mode = Mode::Edit;
                }
                KeyCode::Char('r') if !app.items.is_empty() => {
                    app.rename_buffer = app.items[app.selected()].name.clone();
                    app.status.clear();
                    app.mode = Mode::Rename;
                }
                KeyCode::Char('i') if !app.items.is_empty() => {
                    let content = app.items[app.selected()].content.clone();
                    app.status = match fs::write(&app.config.logo_file, &content) {
                        Ok(_) => " ✓ Logo set! ".to_string(),
                        Err(e) => format!(" ✗ Error: {} ", e),
                    };
                }
                KeyCode::Char('d') if !app.items.is_empty() => {
                    app.mode = Mode::ConfirmDelete;
                }
                KeyCode::Char('p') => {
                    app.path_buffer = app.config.db_file.clone();
                    app.status.clear();
                    app.mode = Mode::SetDbPath;
                }
                KeyCode::Char('l') => {
                    app.path_buffer = app.config.logo_file.clone();
                    app.status.clear();
                    app.mode = Mode::SetLogoPath;
                }
                KeyCode::Up if key.modifiers.contains(KeyModifiers::SHIFT) => app.move_item_up(),
                KeyCode::Down if key.modifiers.contains(KeyModifiers::SHIFT) => {
                    app.move_item_down()
                }
                KeyCode::Down if !app.items.is_empty() => {
                    let i = app.selected();
                    if i + 1 < app.items.len() {
                        app.list_state.select(Some(i + 1));
                        app.status.clear();
                    }
                }
                KeyCode::Up if !app.items.is_empty() => {
                    let i = app.selected();
                    if i > 0 {
                        app.list_state.select(Some(i - 1));
                        app.status.clear();
                    }
                }
                _ => {}
            },
            Mode::ConfirmDelete => match key.code {
                KeyCode::Char('y') | KeyCode::Enter => {
                    let idx = app.selected();
                    app.items.remove(idx);
                    save_data(&app.items, &app.config.db_file);
                    let new_sel = if app.items.is_empty() {
                        None
                    } else {
                        Some(idx.saturating_sub(1).min(app.items.len().saturating_sub(1)))
                    };
                    app.list_state.select(new_sel);
                    app.status = " Deleted ".to_string();
                    app.mode = Mode::Browse;
                }
                KeyCode::Char('n') | KeyCode::Esc => {
                    app.status = " Canceled ".to_string();
                    app.mode = Mode::Browse;
                }
                _ => {}
            },
            Mode::Rename => match key.code {
                KeyCode::Enter => {
                    let idx = app.selected();
                    app.items[idx].name = app.rename_buffer.clone();
                    save_data(&app.items, &app.config.db_file);
                    app.mode = Mode::Browse;
                }
                KeyCode::Esc => app.mode = Mode::Browse,
                KeyCode::Backspace => {
                    app.rename_buffer.pop();
                }
                KeyCode::Char(c) => app.rename_buffer.push(c),
                _ => {}
            },
            Mode::SetDbPath => match key.code {
                KeyCode::Enter => {
                    if is_safe_path(&app.path_buffer) {
                        app.config.db_file = app.path_buffer.clone();
                        app.config.save();
                        app.items = load_data(&app.config.db_file);
                        app.list_state
                            .select(if app.items.is_empty() { None } else { Some(0) });
                        app.status = format!(" ✓ DB → {} ", app.config.db_file);
                    } else {
                        app.status =
                            " ✗ ERRORE: Percorso non sicuro o fuori da /home/ ".to_string();
                    }
                    app.mode = Mode::Browse;
                }
                KeyCode::Esc => app.mode = Mode::Browse,
                KeyCode::Backspace => {
                    app.path_buffer.pop();
                }
                KeyCode::Char(c) => app.path_buffer.push(c),
                _ => {}
            },
            Mode::SetLogoPath => match key.code {
                KeyCode::Enter => {
                    if is_safe_path(&app.path_buffer) {
                        app.config.logo_file = app.path_buffer.clone();
                        app.config.save();
                        app.status = format!(" ✓ Logo path → {} ", app.config.logo_file);
                    } else {
                        app.status =
                            " ✗ ERRORE: Percorso non sicuro o fuori da /home/ ".to_string();
                    }
                    app.mode = Mode::Browse;
                }
                KeyCode::Esc => app.mode = Mode::Browse,
                KeyCode::Backspace => {
                    app.path_buffer.pop();
                }
                KeyCode::Char(c) => app.path_buffer.push(c),
                _ => {}
            },
            Mode::Edit => {
                if key.modifiers.contains(KeyModifiers::CONTROL) {
                    match key.code {
                        KeyCode::Char('s') => {
                            let idx = app.selected();
                            app.items[idx].content = app.edit_buffer.clone();
                            save_data(&app.items, &app.config.db_file);
                            app.status = " Saved ".to_string();
                            app.mode = Mode::Browse;
                        }
                        KeyCode::Char('a') => {
                            app.edit_cursor = line_start(&app.edit_buffer, app.edit_cursor)
                        }
                        KeyCode::Char('e') => {
                            app.edit_cursor = line_end(&app.edit_buffer, app.edit_cursor)
                        }
                        _ => {}
                    }
                } else {
                    match key.code {
                        KeyCode::Esc => app.mode = Mode::Browse,
                        KeyCode::Enter => {
                            insert_char(&mut app.edit_buffer, &mut app.edit_cursor, '\n')
                        }
                        KeyCode::Backspace => backspace(&mut app.edit_buffer, &mut app.edit_cursor),
                        KeyCode::Delete => delete_char(&mut app.edit_buffer, app.edit_cursor),
                        KeyCode::Left => {
                            app.edit_cursor = cursor_left(&app.edit_buffer, app.edit_cursor)
                        }
                        KeyCode::Right => {
                            app.edit_cursor = cursor_right(&app.edit_buffer, app.edit_cursor)
                        }
                        KeyCode::Up => {
                            app.edit_cursor = cursor_up(&app.edit_buffer, app.edit_cursor)
                        }
                        KeyCode::Down => {
                            app.edit_cursor = cursor_down(&app.edit_buffer, app.edit_cursor)
                        }
                        KeyCode::Home => {
                            app.edit_cursor = line_start(&app.edit_buffer, app.edit_cursor)
                        }
                        KeyCode::End => {
                            app.edit_cursor = line_end(&app.edit_buffer, app.edit_cursor)
                        }
                        KeyCode::Char(c) => {
                            insert_char(&mut app.edit_buffer, &mut app.edit_cursor, c)
                        }
                        _ => {}
                    }
                }
            }
        }
    }
    Ok(false)
}
