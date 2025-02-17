use std::env;
use std::fs::{self, DirEntry};
use std::io::{self, Write, stdout};
use std::os::windows::fs::MetadataExt;
use std::path::Path;
use tui::backend::CrosstermBackend;
use tui::layout::{Constraint, Direction, Layout};
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Block, Borders, List, ListItem, Paragraph};
use tui::Terminal;
use crossterm::event::{self, Event, KeyCode};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
fn list_files_in_directory(path: &Path) -> io::Result<Vec<DirEntry>> {
    let entries = fs::read_dir(path)?
        .filter_map(Result::ok)
        .collect::<Vec<_>>();
    Ok(entries)
}

fn display_files(entries: &[DirEntry]) {
    for (i, entry) in entries.iter().enumerate() {
        let file_name = entry.file_name();
        let entry_name = file_name.to_string_lossy().into_owned();
        let entry_type = if entry.file_type().unwrap().is_dir() {
            "[DIR]".to_string()
        } else {
            "[FILE]".to_string()
        };
        println!("{}: {} {}", i + 1, entry_type, entry_name);
    }
}

pub fn navigate_directory() -> io::Result<()> {
    let mut current_path = env::current_dir()?;
    
    loop {
        println!("\nCurrent directory: {}", current_path.display());

        // List files and directories
        let entries = list_files_in_directory(&current_path)?;
        display_files(&entries);

        println!("\nEnter the number of the directory to enter (or 'back' to go up, 'exit' to quit): ");
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim().to_lowercase();

        if input == "exit" {
            break;
        } else if input == "back" {
            if let Some(parent) = current_path.parent() {
                current_path = parent.to_path_buf();
            } else {
                println!("You are already at the root directory.");
            }
        } else if let Ok(choice) = input.parse::<usize>() {
            if choice > 0 && choice <= entries.len() {
                let selected_entry = &entries[choice - 1];
                let selected_path = selected_entry.path();

                if selected_path.is_dir() {
                    current_path = selected_path;
                } else {
                    println!("You selected a file: {:?}", selected_path.display());
                }
            } else {
                println!("Invalid choice. Please enter a valid number.");
            }
        } else {
            println!("Invalid input. Please enter 'back', 'exit', or a number.");
        }
    }

    Ok(())
}

pub fn navigate_directory_tui() -> io::Result<()> {
    enable_raw_mode()?;
    let stdout = stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;
    let mut current_path = env::current_dir()?;
    let mut entries = list_files_in_directory(&current_path)?;
    let mut selected_index = 0;
    let mut bottom_message = "".to_string();
    loop {
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => break,
                KeyCode::Up => {
                    if selected_index > 0 {
                        selected_index -= 1;
                    }
                }
                KeyCode::Down => {
                    if selected_index < entries.len() - 1 {
                        selected_index += 1;
                    }
                }
                KeyCode::Enter => {
                    let selected_entry = &entries[selected_index];
                    let selected_path = selected_entry.path();

                    if selected_path.is_dir() {
                        current_path = selected_path;
                        entries = list_files_in_directory(&current_path)?;
                        selected_index = 0;
                    } else {
                        bottom_message = format!("{:?}", selected_path.metadata()?.last_write_time());
                    }
                }
                KeyCode::Backspace => {
                    if let Some(parent) = current_path.parent() {
                        current_path = parent.to_path_buf();
                        entries = list_files_in_directory(&current_path)?;
                        selected_index = 0;
                    } else {
                        bottom_message = "You are already at the root directory.".to_string();
                    }
                }
                _ => {}
            }
        }

        terminal.draw(|f| {
            let size = f.size();
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Ratio(3, 5), Constraint::Ratio(1, 5), Constraint::Ratio(1, 5)].as_ref())
                .split(size);

            let items: Vec<ListItem> = entries
                .iter()
                .enumerate()
                .map(|(i, entry)| {
                    let file_name = entry.file_name();
                    let entry_name = file_name.to_string_lossy().into_owned();
                    let entry_type = if entry.file_type().unwrap().is_dir() {
                        "[DIR]".to_string()
                    } else {
                        "[FILE]".to_string()
                    };
                    let style = if i == selected_index {
                        Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
                    } else {
                        Style::default()
                    };
                    ListItem::new(Spans::from(vec![
                        Span::styled(format!("{}: ", i + 1), style),
                        Span::styled(entry_type, style),
                        Span::styled(entry_name, style),
                    ]))
                })
                .collect();
                let max_visible_items = chunks[0].height as usize - 2; // Subtracting 2 for borders
                let start = selected_index.saturating_sub(max_visible_items / 2);
                let end = start + max_visible_items;
                let visible_items = &items[start..end.min(items.len())];
            let list = List::new(visible_items)
                .block(Block::default().borders(Borders::ALL).title("Files"))
                .highlight_style(Style::default().bg(Color::Blue));

            f.render_widget(list, chunks[0]);

            let instructions = Paragraph::new("Use arrow keys to navigate, Enter to select, Backspace to go up, 'q' to quit.")
                .block(Block::default().borders(Borders::ALL).title("Instructions"));
            f.render_widget(instructions, chunks[1]);

            let message = Paragraph::new(bottom_message.clone())
                .block(Block::default().borders(Borders::ALL).title("Message"))
                .style(Style::default().fg(Color::Yellow));
            f.render_widget(message, chunks[2]);
        })?;

        
    }

    disable_raw_mode()?;
    terminal.clear()?;
    Ok(())
}