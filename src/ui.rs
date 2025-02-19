use crate::app::App;
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    symbols,
    text::{Span, Spans},
    widgets::canvas::{Canvas, Line, Map, MapResolution, Rectangle},
    widgets::{
        Axis, BarChart, Block, Borders, Cell, Chart, Dataset, Gauge, LineGauge, List, ListItem,
        Paragraph, Row, Sparkline, Table, Tabs, Wrap,
    },
    Frame,
};

pub fn draw<B: Backend>(f: &mut Frame<B>, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0), Constraint::Length(3)].as_ref())
        .split(f.size());
    let titles = app
        .tabs
        .titles
        .iter()
        .map(|t| Spans::from(Span::styled(*t, Style::default().fg(Color::Green))))
        .collect();
    let tabs = Tabs::new(titles)
        .block(Block::default().borders(Borders::ALL).title(app.title))
        .highlight_style(Style::default().fg(Color::Yellow))
        .select(app.tabs.tab_index);
    f.render_widget(tabs, chunks[0]);
    match app.tabs.tab_index {
        0 => draw_playlist(f, chunks[1], app),
        1 => draw_explorer(f, chunks[1], app),
        2 => draw_turntable(f, chunks[1], app),
        _ => {}
    }

    let notification = vec![
        Spans::from(vec![
            Span::styled("Title: ", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(app.message.clone()),
        ]),
    ];

    let block = Block::default().borders(Borders::ALL).title(Span::styled(
        "Notification",
        Style::default()
            .fg(Color::Magenta)
            .add_modifier(Modifier::BOLD),
    ));
    let notification_view = Paragraph::new(notification)
        .block(block)
        .wrap(Wrap { trim: true });

    f.render_widget(notification_view, chunks[2]);
}

fn draw_playlist<B>(f: &mut Frame<B>, area: Rect, app: &App)
where
    B: Backend,
{
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(62), Constraint::Percentage(38)].as_ref())
        .split(area);
    let playlist: Vec<ListItem> = app
        .playlist
        .records
        .iter()
        .enumerate()
        .map(|(i, record)| {
            let style = if app.playlist.selected_index == i {
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            };
            ListItem::new(Spans::from(vec![Span::styled(record.title, style)]))
        })
        .collect();
    let playlist_view = List::new(playlist)
        .block(Block::default().borders(Borders::ALL).title("Items"))
        .highlight_style(Style::default().add_modifier(Modifier::BOLD))
        .highlight_symbol(">>");
    f.render_widget(playlist_view, chunks[0]);

    let current_info = vec![
        Spans::from(vec![
            Span::styled("Title: ", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(app.playlist.current().unwrap().title),
        ]),
        Spans::from(vec![
            Span::styled("Artist: ", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(app.playlist.current().unwrap().artist),
        ]),
        Spans::from(vec![
            Span::styled("Album: ", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(app.playlist.current().unwrap().album),
        ]),
    ];

    let block = Block::default().borders(Borders::ALL).title(Span::styled(
        "Footer",
        Style::default()
            .fg(Color::Magenta)
            .add_modifier(Modifier::BOLD),
    ));
    let paragraph = Paragraph::new(current_info)
        .block(block)
        .wrap(Wrap { trim: true });
    f.render_widget(paragraph, chunks[1]);
}

fn draw_turntable<B>(f: &mut Frame<B>, area: Rect, app: &App)
where
    B: Backend,
{
    let datasets = vec![
        Dataset::default()
            .name("data2")
            .marker(symbols::Marker::Dot)
            .style(Style::default().fg(Color::Cyan))
            .data(&[(0.0, 1.0), (1.0, 2.0), (2.0, 1.0), (3.0, 1.5)]),
        Dataset::default()
            .name("data1")
            .marker(symbols::Marker::Braille)
            .style(Style::default().fg(Color::Yellow))
            .data(&[(0.0, 2.0), (1.0, 1.0), (2.0, 2.5), (3.0, 2.0)]),
    ];
    let chart = Chart::new(datasets)
        .block(Block::default().title("Chart").borders(Borders::ALL))
        .x_axis(
            Axis::default()
                .title("X-AXIS")
                .style(Style::default().fg(Color::Gray)),
        )
        .y_axis(
            Axis::default()
                .title("Y-AXIS")
                .style(Style::default().fg(Color::Gray)),
        );
    f.render_widget(chart, area);
}

fn draw_explorer<B>(f: &mut Frame<B>, area: Rect, app: &App)
where
    B: Backend,
{
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(62), Constraint::Percentage(38)].as_ref())
        .split(area);
    let entries: Vec<ListItem> = app.explorer.get_entries_text().iter().enumerate().map(
        |(i, entry)| {
            let style = if app.explorer.selected_index == i {
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            };
            ListItem::new(Spans::from(vec![Span::styled(entry.clone(), style)]))
        },
    ).collect();
    let max_visible_items = chunks[0].height as usize - 2; // Subtracting 2 for borders
    let start = app.explorer.selected_index.saturating_sub(max_visible_items / 2);
    let end = start + max_visible_items;
    let visible_entries = &entries[start..end.min(entries.len())];
    let list_view = List::new(visible_entries)
                .block(Block::default().borders(Borders::ALL).title("Files"))
                .highlight_style(Style::default().bg(Color::Blue));
    f.render_widget(list_view, chunks[0]);
}
