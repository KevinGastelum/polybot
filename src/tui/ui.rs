//! TUI rendering functions.

use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Cell, List, ListItem, Paragraph, Row, Table, Tabs},
    Frame,
};

use super::app::{App, Tab};

/// Main UI rendering function.
pub fn draw(frame: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // Header
            Constraint::Length(3),  // Tabs
            Constraint::Min(10),    // Main content
            Constraint::Length(3),  // Status bar
        ])
        .split(frame.area());

    draw_header(frame, app, chunks[0]);
    draw_tabs(frame, app, chunks[1]);
    draw_content(frame, app, chunks[2]);
    draw_status_bar(frame, app, chunks[3]);
}

fn draw_header(frame: &mut Frame, app: &App, area: Rect) {
    let summary = app.engine.summary();
    
    let pnl_color = if summary.total_pnl >= 0.0 { Color::Green } else { Color::Red };
    let pnl_sign = if summary.total_pnl >= 0.0 { "+" } else { "" };
    
    let header_text = vec![
        Span::styled("📊 ", Style::default()),
        Span::styled("Polymarket-Kalshi Arbitrage Bot", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
        Span::raw("  │  Balance: "),
        Span::styled(format!("${:.2}", summary.total_value), Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
        Span::raw("  │  P&L: "),
        Span::styled(
            format!("{}${:.2} ({}{:.1}%)", pnl_sign, summary.total_pnl.abs(), pnl_sign, summary.pnl_percent),
            Style::default().fg(pnl_color).add_modifier(Modifier::BOLD)
        ),
    ];

    let header = Paragraph::new(Line::from(header_text))
        .block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(Color::DarkGray)));
    
    frame.render_widget(header, area);
}

fn draw_tabs(frame: &mut Frame, app: &App, area: Rect) {
    let titles: Vec<Line> = [Tab::Dashboard, Tab::Markets, Tab::Trades, Tab::Strategies]
        .iter()
        .enumerate()
        .map(|(i, t)| {
            let style = if *t == app.active_tab {
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::Gray)
            };
            Line::from(Span::styled(format!("[{}] {}", i + 1, t.title()), style))
        })
        .collect();

    let tabs = Tabs::new(titles)
        .block(Block::default().borders(Borders::ALL).title(" Navigation "))
        .highlight_style(Style::default().fg(Color::Yellow))
        .divider(" │ ");
    
    frame.render_widget(tabs, area);
}

fn draw_content(frame: &mut Frame, app: &App, area: Rect) {
    match app.active_tab {
        Tab::Dashboard => draw_dashboard(frame, app, area),
        Tab::Markets => draw_markets(frame, app, area),
        Tab::Trades => draw_trades(frame, app, area),
        Tab::Strategies => draw_strategies(frame, app, area),
    }
}

fn draw_dashboard(frame: &mut Frame, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(area);

    let left_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(chunks[0]);

    let right_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(chunks[1]);

    // Performance block
    draw_performance(frame, app, left_chunks[0]);
    
    // Recent trades block
    draw_recent_trades(frame, app, left_chunks[1]);
    
    // Active positions block
    draw_positions(frame, app, right_chunks[0]);
    
    // Top traders block
    draw_top_traders(frame, app, right_chunks[1]);
}

fn draw_performance(frame: &mut Frame, app: &App, area: Rect) {
    let summary = app.engine.summary();
    let pnl_color = if summary.total_pnl >= 0.0 { Color::Green } else { Color::Red };
    
    let text = vec![
        Line::from(vec![
            Span::raw("Total P&L:     "),
            Span::styled(
                format!("${:.2} ({:.1}%)", summary.total_pnl, summary.pnl_percent),
                Style::default().fg(pnl_color).add_modifier(Modifier::BOLD)
            ),
        ]),
        Line::from(vec![
            Span::raw("Win Rate:      "),
            Span::styled(
                format!("{:.0}% ({}/{})", summary.win_rate * 100.0, summary.wins, summary.total_trades),
                Style::default().fg(Color::Cyan)
            ),
        ]),
        Line::from(vec![
            Span::raw("Best Trade:    "),
            Span::styled(
                format!("${:.2}", summary.best_trade_pnl.unwrap_or(0.0)),
                Style::default().fg(Color::Green)
            ),
        ]),
        Line::from(vec![
            Span::raw("Worst Trade:   "),
            Span::styled(
                format!("${:.2}", summary.worst_trade_pnl.unwrap_or(0.0)),
                Style::default().fg(Color::Red)
            ),
        ]),
        Line::from(vec![
            Span::raw("Cash Balance:  "),
            Span::styled(
                format!("${:.2}", summary.cash_balance),
                Style::default().fg(Color::Yellow)
            ),
        ]),
    ];

    let block = Paragraph::new(text)
        .block(Block::default()
            .borders(Borders::ALL)
            .title(" 📈 Performance ")
            .border_style(Style::default().fg(Color::Blue)));
    
    frame.render_widget(block, area);
}

fn draw_recent_trades(frame: &mut Frame, app: &App, area: Rect) {
    let trades = app.recent_trades();
    
    let items: Vec<ListItem> = trades.iter().take(5).map(|trade| {
        let (icon, color) = if trade.is_profitable() {
            ("✅", Color::Green)
        } else if trade.pnl.is_some() {
            ("❌", Color::Red)
        } else {
            ("⏳", Color::Yellow)
        };
        
        let pnl_str = trade.pnl.map(|p| format!("{:+.2}", p)).unwrap_or_else(|| "open".to_string());
        
        ListItem::new(Line::from(vec![
            Span::raw(format!("{} ", icon)),
            Span::styled(&trade.market, Style::default().fg(Color::White)),
            Span::raw(" "),
            Span::styled(format!("${}", pnl_str), Style::default().fg(color)),
            Span::raw(" "),
            Span::styled(&trade.strategy, Style::default().fg(Color::DarkGray)),
        ]))
    }).collect();

    let list = List::new(items)
        .block(Block::default()
            .borders(Borders::ALL)
            .title(" 📋 Recent Trades ")
            .border_style(Style::default().fg(Color::Magenta)));
    
    frame.render_widget(list, area);
}

fn draw_positions(frame: &mut Frame, app: &App, area: Rect) {
    let positions = app.open_positions();
    
    let items: Vec<ListItem> = positions.iter().map(|(_, pos)| {
        let pnl_color = if pos.unrealized_pnl >= 0.0 { Color::Green } else { Color::Red };
        
        ListItem::new(Line::from(vec![
            Span::styled(&pos.market, Style::default().fg(Color::White)),
            Span::raw(": "),
            Span::styled(format!("${:.0}", pos.size * pos.avg_price), Style::default().fg(Color::Yellow)),
            Span::raw(" @ "),
            Span::styled(format!("{:.2}", pos.avg_price), Style::default().fg(Color::Cyan)),
            Span::raw(" ("),
            Span::styled(format!("{:+.2}", pos.unrealized_pnl), Style::default().fg(pnl_color)),
            Span::raw(")"),
        ]))
    }).collect();

    let list = List::new(items)
        .block(Block::default()
            .borders(Borders::ALL)
            .title(" 🎯 Active Positions ")
            .border_style(Style::default().fg(Color::Green)));
    
    frame.render_widget(list, area);
}

fn draw_top_traders(frame: &mut Frame, app: &App, area: Rect) {
    let items: Vec<ListItem> = app.top_traders.iter().map(|trader| {
        let copy_icon = if trader.is_copying { "📋" } else { "  " };
        
        ListItem::new(Line::from(vec![
            Span::raw(format!("{} ", copy_icon)),
            Span::styled(&trader.name, Style::default().fg(Color::White)),
            Span::raw("  "),
            Span::styled(
                format!("+${:.0}K", trader.monthly_pnl / 1000.0),
                Style::default().fg(Color::Green)
            ),
        ]))
    }).collect();

    let list = List::new(items)
        .block(Block::default()
            .borders(Borders::ALL)
            .title(" 👥 Top Traders ")
            .border_style(Style::default().fg(Color::Yellow)));
    
    frame.render_widget(list, area);
}

fn draw_markets(frame: &mut Frame, app: &App, area: Rect) {
    let header = Row::new(vec![
        Cell::from("Market").style(Style::default().fg(Color::Yellow)),
        Cell::from("Coin").style(Style::default().fg(Color::Yellow)),
        Cell::from("Poly").style(Style::default().fg(Color::Yellow)),
        Cell::from("Kalshi").style(Style::default().fg(Color::Yellow)),
        Cell::from("Spread").style(Style::default().fg(Color::Yellow)),
        Cell::from("Liquidity").style(Style::default().fg(Color::Yellow)),
        Cell::from("Time").style(Style::default().fg(Color::Yellow)),
    ]).height(1);

    let rows: Vec<Row> = app.markets.iter().enumerate().map(|(i, market)| {
        let style = if i == app.selected_index {
            Style::default().bg(Color::DarkGray)
        } else {
            Style::default()
        };
        
        let spread_color = if market.spread.unwrap_or(0.0) > 0.02 { Color::Green } else { Color::White };
        
        Row::new(vec![
            Cell::from(market.name.clone()),
            Cell::from(market.coin.clone()).style(Style::default().fg(Color::Cyan)),
            Cell::from(format!("{:.3}", market.poly_price.unwrap_or(0.0))),
            Cell::from(format!("{:.3}", market.kalshi_price.unwrap_or(0.0))),
            Cell::from(format!("{:.1}%", market.spread.unwrap_or(0.0) * 100.0))
                .style(Style::default().fg(spread_color)),
            Cell::from(format!("${:.0}K", market.liquidity / 1000.0)),
            Cell::from(market.time_to_resolve.clone()),
        ]).style(style).height(1)
    }).collect();

    let table = Table::new(rows, [
        Constraint::Percentage(25),
        Constraint::Percentage(10),
        Constraint::Percentage(12),
        Constraint::Percentage(12),
        Constraint::Percentage(12),
        Constraint::Percentage(15),
        Constraint::Percentage(14),
    ])
    .header(header)
    .block(Block::default()
        .borders(Borders::ALL)
        .title(" 🔄 Live Markets (↑↓ navigate, B=buy, S=sell, R=refresh) ")
        .border_style(Style::default().fg(Color::Cyan)));
    
    frame.render_widget(table, area);
}

fn draw_trades(frame: &mut Frame, app: &App, area: Rect) {
    let trades = app.engine.trade_log.get_all();
    
    let header = Row::new(vec![
        Cell::from("Time").style(Style::default().fg(Color::Yellow)),
        Cell::from("Market").style(Style::default().fg(Color::Yellow)),
        Cell::from("Side").style(Style::default().fg(Color::Yellow)),
        Cell::from("Size").style(Style::default().fg(Color::Yellow)),
        Cell::from("Entry").style(Style::default().fg(Color::Yellow)),
        Cell::from("Exit").style(Style::default().fg(Color::Yellow)),
        Cell::from("P&L").style(Style::default().fg(Color::Yellow)),
        Cell::from("Strategy").style(Style::default().fg(Color::Yellow)),
    ]).height(1);

    let rows: Vec<Row> = trades.iter().rev().take(20).enumerate().map(|(i, trade)| {
        let style = if i == app.selected_index {
            Style::default().bg(Color::DarkGray)
        } else {
            Style::default()
        };
        
        let pnl_str = trade.pnl.map(|p| format!("{:+.2}", p)).unwrap_or_else(|| "-".to_string());
        let pnl_color = if trade.is_profitable() { Color::Green } else if trade.pnl.is_some() { Color::Red } else { Color::White };
        let side_color = if matches!(trade.side, crate::paper_trading::Side::Buy) { Color::Green } else { Color::Red };
        
        Row::new(vec![
            Cell::from(trade.timestamp.format("%H:%M:%S").to_string()),
            Cell::from(trade.market.chars().take(20).collect::<String>()),
            Cell::from(trade.side.to_string()).style(Style::default().fg(side_color)),
            Cell::from(format!("${:.0}", trade.size)),
            Cell::from(format!("{:.3}", trade.entry_price)),
            Cell::from(trade.exit_price.map(|p| format!("{:.3}", p)).unwrap_or_else(|| "-".to_string())),
            Cell::from(pnl_str).style(Style::default().fg(pnl_color)),
            Cell::from(trade.strategy.clone()),
        ]).style(style).height(1)
    }).collect();

    let table = Table::new(rows, [
        Constraint::Percentage(12),
        Constraint::Percentage(22),
        Constraint::Percentage(8),
        Constraint::Percentage(10),
        Constraint::Percentage(10),
        Constraint::Percentage(10),
        Constraint::Percentage(12),
        Constraint::Percentage(16),
    ])
    .header(header)
    .block(Block::default()
        .borders(Borders::ALL)
        .title(" 📜 Trade History ")
        .border_style(Style::default().fg(Color::Magenta)));
    
    frame.render_widget(table, area);
}

fn draw_strategies(frame: &mut Frame, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(area);

    // Strategies list
    let items: Vec<ListItem> = app.strategies.iter().enumerate().map(|(i, strategy)| {
        let style = if i == app.selected_index {
            Style::default().bg(Color::DarkGray)
        } else {
            Style::default()
        };
        
        let status = if strategy.enabled { "✅ ON " } else { "❌ OFF" };
        let status_color = if strategy.enabled { Color::Green } else { Color::Red };
        
        ListItem::new(Line::from(vec![
            Span::styled(status, Style::default().fg(status_color)),
            Span::raw(" "),
            Span::styled(&strategy.name, Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
            Span::raw("  │  Trades: "),
            Span::styled(format!("{}", strategy.trades_today), Style::default().fg(Color::Cyan)),
            Span::raw("  │  P&L: "),
            Span::styled(
                format!("${:.2}", strategy.pnl_today),
                Style::default().fg(if strategy.pnl_today >= 0.0 { Color::Green } else { Color::Red })
            ),
        ])).style(style)
    }).collect();

    let list = List::new(items)
        .block(Block::default()
            .borders(Borders::ALL)
            .title(" ⚙️ Strategies (Enter to toggle) ")
            .border_style(Style::default().fg(Color::Yellow)));
    
    frame.render_widget(list, chunks[0]);

    // Help text
    let help_text = vec![
        Line::from(""),
        Line::from(Span::styled("Keyboard Shortcuts:", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))),
        Line::from(""),
        Line::from("  1-4    Switch tabs"),
        Line::from("  Tab    Next tab"),
        Line::from("  ↑/↓    Navigate list"),
        Line::from("  Enter  Select/Toggle"),
        Line::from("  B      Paper Buy"),
        Line::from("  S      Paper Sell"),
        Line::from("  R      Refresh data"),
        Line::from("  Q      Quit"),
        Line::from(""),
        Line::from(Span::styled("Strategies:", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))),
        Line::from(""),
        Line::from("  Arbitrage:   Price discrepancies"),
        Line::from("  Copy Trade:  Mirror top traders"),
        Line::from("  Manual:      User-initiated trades"),
    ];

    let help = Paragraph::new(help_text)
        .block(Block::default()
            .borders(Borders::ALL)
            .title(" ❓ Help ")
            .border_style(Style::default().fg(Color::DarkGray)));
    
    frame.render_widget(help, chunks[1]);
}

fn draw_status_bar(frame: &mut Frame, app: &App, area: Rect) {
    let status = app.status_message.as_deref().unwrap_or("Ready");
    
    let text = Line::from(vec![
        Span::raw(" "),
        Span::styled(status, Style::default().fg(Color::White)),
        Span::raw("  │  "),
        Span::styled("Q", Style::default().fg(Color::Yellow)),
        Span::raw("uit  "),
        Span::styled("R", Style::default().fg(Color::Yellow)),
        Span::raw("efresh  "),
        Span::styled("B", Style::default().fg(Color::Green)),
        Span::raw("uy  "),
        Span::styled("S", Style::default().fg(Color::Red)),
        Span::raw("ell"),
    ]);

    let status_bar = Paragraph::new(text)
        .block(Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::DarkGray)));
    
    frame.render_widget(status_bar, area);
}
