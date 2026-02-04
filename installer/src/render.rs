//! Rendering logic for the wizard UI
//! Catppuccin Mocha meets Commodore 64 phosphor green

use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    symbols,
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Gauge, List, ListItem, Paragraph, Wrap},
    Frame,
};

use crate::{
    catalog::{self, Category},
    effects::{HackerTheme, Theme},
    wizard::{SetupType, WizardPhase},
    App,
};

pub fn render_app(frame: &mut Frame, app: &mut App) {
    let size = frame.size();

    // Dark background — Catppuccin Crust
    let bg = Block::default().style(Style::default().bg(Theme::CRUST));
    frame.render_widget(bg, size);

    match app.wizard.phase {
        WizardPhase::Boot => render_boot(frame, app, size),
        WizardPhase::Identity => render_identity(frame, app, size),
        WizardPhase::Shell => render_shell(frame, app, size),
        WizardPhase::DevTools => render_devtools(frame, app, size),
        WizardPhase::Apps => render_apps(frame, app, size),
        WizardPhase::Review => render_review(frame, app, size),
        WizardPhase::Install => render_install(frame, app, size),
        WizardPhase::Complete => render_complete(frame, app, size),
    }
}

// ═══════════════════════════════════════════════════════════════════════
//  Boot screen — matrix rain + typewriter, the landing page you love
// ═══════════════════════════════════════════════════════════════════════

fn render_boot(frame: &mut Frame, app: &mut App, area: Rect) {
    // Matrix rain background
    frame.render_widget(&app.matrix_rain, area);

    // Center content
    let center = centered_rect(60, 50, area);
    frame.render_widget(Clear, center);

    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(ratatui::widgets::BorderType::Rounded)
        .border_style(Style::default().fg(Theme::PHOSPHOR_DIM))
        .style(Style::default().bg(Color::Rgb(10, 10, 15)));

    let inner = block.inner(center);
    frame.render_widget(block, center);

    let mut lines: Vec<Line> = Vec::new();
    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        "    **** COMMODORE 64 BASIC V2 ****",
        Style::default()
            .fg(Theme::BLUE)
            .add_modifier(Modifier::BOLD),
    )));
    lines.push(Line::from(""));

    for (i, (msg, complete)) in app.boot_sequence.messages.iter().enumerate() {
        let status = if *complete {
            Span::styled(" OK ", HackerTheme::success())
        } else if i == app.boot_sequence.stage {
            Span::styled(
                format!(" {} ", app.spinner.current()),
                Style::default().fg(Theme::YELLOW),
            )
        } else {
            Span::styled(" .. ", HackerTheme::muted())
        };

        let text = if i == app.boot_sequence.stage {
            if let Some(tw) = &app.boot_sequence.current_typewriter {
                format!("{}{}", tw.visible_text(), tw.cursor())
            } else {
                msg.to_string()
            }
        } else if *complete {
            msg.to_string()
        } else {
            String::new()
        };

        lines.push(Line::from(vec![
            Span::raw("   "),
            status,
            Span::raw(" "),
            Span::styled(text, HackerTheme::phosphor()),
        ]));
    }

    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        "   Press any key to skip...",
        HackerTheme::muted(),
    )));

    let paragraph = Paragraph::new(lines);
    frame.render_widget(paragraph, inner);
}

// ═══════════════════════════════════════════════════════════════════════
//  Identity screen
// ═══════════════════════════════════════════════════════════════════════

fn render_identity(frame: &mut Frame, app: &mut App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Header
            Constraint::Length(1), // Phase indicator
            Constraint::Length(1), // System info
            Constraint::Length(1), // Spacer
            Constraint::Min(0),    // Content
            Constraint::Length(1), // Footer
        ])
        .split(area);

    render_header(frame, chunks[0], "IDENTITY MATRIX");
    render_phase_indicator(frame, chunks[1], &app.wizard.phase);

    // System info bar
    let sys = &app.system;
    let brew_status = if sys.has_homebrew() {
        Span::styled(" brew ", HackerTheme::success())
    } else {
        Span::styled(" brew ", HackerTheme::warning())
    };
    let sys_info = Line::from(vec![
        Span::styled("  ", Style::default()),
        Span::styled(
            format!(" {} {} ", sys.os.name(), sys.arch.name()),
            Style::default().fg(Theme::SUBTEXT0),
        ),
        Span::styled("  ", HackerTheme::muted()),
        brew_status,
        Span::styled("  ", HackerTheme::muted()),
        Span::styled(
            format!(" {} ", sys.hostname),
            Style::default().fg(Theme::OVERLAY1),
        ),
    ]);
    frame.render_widget(Paragraph::new(sys_info), chunks[2]);

    // Main content
    let form_area = centered_rect(65, 90, chunks[4]);

    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(ratatui::widgets::BorderType::Rounded)
        .border_style(HackerTheme::border())
        .style(Style::default().bg(Theme::MANTLE));

    let inner = block.inner(form_area);
    frame.render_widget(block, form_area);

    let field_chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(5),
            Constraint::Min(0),
        ])
        .split(inner);

    render_input_field(
        frame,
        field_chunks[0],
        "NAME",
        &app.wizard.identity.name,
        app.wizard.input_field == 0,
    );
    render_input_field(
        frame,
        field_chunks[1],
        "EMAIL",
        &app.wizard.identity.email,
        app.wizard.input_field == 1,
    );
    render_input_field(
        frame,
        field_chunks[2],
        "GITHUB",
        &app.wizard.identity.github_username,
        app.wizard.input_field == 2,
    );
    render_setup_type_selector(
        frame,
        field_chunks[3],
        app.wizard.identity.setup_type,
        app.wizard.input_field == 3,
    );

    render_footer(
        frame,
        chunks[5],
        &[("tab", "next"), ("enter", "continue"), ("esc", "back")],
    );
}

// ═══════════════════════════════════════════════════════════════════════
//  Shell screen
// ═══════════════════════════════════════════════════════════════════════

fn render_shell(frame: &mut Frame, app: &mut App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Header
            Constraint::Length(1), // Phase indicator
            Constraint::Length(1), // Spacer
            Constraint::Min(0),    // Content
            Constraint::Length(1), // Footer
        ])
        .split(area);

    render_header(frame, chunks[0], "COMMAND INTERFACE");
    render_phase_indicator(frame, chunks[1], &app.wizard.phase);

    let content_area = centered_rect(75, 90, chunks[3]);

    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(ratatui::widgets::BorderType::Rounded)
        .border_style(HackerTheme::border())
        .style(Style::default().bg(Theme::MANTLE));

    let inner = block.inner(content_area);
    frame.render_widget(block, content_area);

    let option_chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(4),
            Constraint::Length(4),
            Constraint::Length(4),
            Constraint::Length(4),
            Constraint::Min(0),
        ])
        .split(inner);

    render_option_selector(
        frame,
        option_chunks[0],
        "SHELL",
        app.wizard.shell_config.shell.name(),
        app.wizard.shell_config.shell.description(),
        app.wizard.cursor_position == 0,
    );
    render_option_selector(
        frame,
        option_chunks[1],
        "PROMPT",
        app.wizard.shell_config.prompt.name(),
        app.wizard.shell_config.prompt.description(),
        app.wizard.cursor_position == 1,
    );
    render_option_selector(
        frame,
        option_chunks[2],
        "TERMINAL",
        app.wizard.shell_config.terminal.name(),
        app.wizard.shell_config.terminal.description(),
        app.wizard.cursor_position == 2,
    );

    let mux_name = app
        .wizard
        .shell_config
        .multiplexer
        .map(|m| m.name())
        .unwrap_or("None");
    let mux_desc = app
        .wizard
        .shell_config
        .multiplexer
        .map(|m| m.description())
        .unwrap_or("No terminal multiplexer");
    render_option_selector(
        frame,
        option_chunks[3],
        "MULTIPLEXER",
        mux_name,
        mux_desc,
        app.wizard.cursor_position == 3,
    );

    render_footer(
        frame,
        chunks[4],
        &[
            ("↑↓", "navigate"),
            ("←→", "change"),
            ("enter", "continue"),
            ("esc", "back"),
        ],
    );
}

// ═══════════════════════════════════════════════════════════════════════
//  DevTools screen
// ═══════════════════════════════════════════════════════════════════════

fn render_devtools(frame: &mut Frame, app: &mut App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(1),
            Constraint::Min(0),
            Constraint::Length(1),
        ])
        .split(area);

    render_header(frame, chunks[0], "DEVELOPMENT ARSENAL");
    render_phase_indicator(frame, chunks[1], &app.wizard.phase);

    let content_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(22), Constraint::Min(0)])
        .split(chunks[2]);

    let categories = &[
        Category::Language,
        Category::Editor,
        Category::Git,
        Category::Container,
        Category::Cloud,
    ];

    render_category_sidebar(
        frame,
        content_chunks[0],
        categories,
        app.wizard.scroll_offset,
    );
    let cat = &categories[app.wizard.scroll_offset % categories.len()];
    render_app_list(frame, content_chunks[1], cat, app);

    render_footer(
        frame,
        chunks[3],
        &[
            ("tab", "category"),
            ("space", "toggle"),
            ("a", "all"),
            ("n", "none"),
            ("enter", "continue"),
        ],
    );
}

// ═══════════════════════════════════════════════════════════════════════
//  Apps screen
// ═══════════════════════════════════════════════════════════════════════

fn render_apps(frame: &mut Frame, app: &mut App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(1),
            Constraint::Min(0),
            Constraint::Length(1),
        ])
        .split(area);

    render_header(frame, chunks[0], "SOFTWARE COMPANIONS");
    render_phase_indicator(frame, chunks[1], &app.wizard.phase);

    let content_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(24), Constraint::Min(0)])
        .split(chunks[2]);

    let categories = Category::all();
    render_category_sidebar(
        frame,
        content_chunks[0],
        categories,
        app.wizard.scroll_offset,
    );

    let cat = &categories[app.wizard.scroll_offset % categories.len()];
    render_app_list(frame, content_chunks[1], cat, app);

    render_footer(
        frame,
        chunks[3],
        &[
            ("tab/S-tab", "category"),
            ("space", "toggle"),
            ("d", "details"),
            ("enter", "continue"),
        ],
    );
}

// ═══════════════════════════════════════════════════════════════════════
//  Review screen
// ═══════════════════════════════════════════════════════════════════════

fn render_review(frame: &mut Frame, app: &mut App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Min(0),
            Constraint::Length(1),
        ])
        .split(area);

    render_header(frame, chunks[0], "CONFIGURATION REVIEW");
    render_phase_indicator(frame, chunks[1], &app.wizard.phase);

    let content_area = centered_rect(80, 95, chunks[3]);

    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(ratatui::widgets::BorderType::Rounded)
        .border_style(HackerTheme::border())
        .style(Style::default().bg(Theme::MANTLE));

    let inner = block.inner(content_area);
    frame.render_widget(block, content_area);

    let mut lines: Vec<Line> = vec![
        section_header("IDENTITY"),
        review_line("  Name", &app.wizard.identity.name),
        review_line("  Email", &app.wizard.identity.email),
        review_line("  GitHub", &app.wizard.identity.github_username),
    ];
    let setup_str = format!(
        "{} {}",
        app.wizard.identity.setup_type.icon(),
        app.wizard.identity.setup_type.name()
    );
    lines.push(review_line("  Setup", &setup_str));
    lines.push(Line::from(""));

    // Shell
    lines.push(section_header("SHELL"));
    lines.push(review_line("  Shell", app.wizard.shell_config.shell.name()));
    lines.push(review_line(
        "  Prompt",
        app.wizard.shell_config.prompt.name(),
    ));
    lines.push(review_line(
        "  Terminal",
        app.wizard.shell_config.terminal.name(),
    ));
    lines.push(review_line(
        "  Multiplexer",
        app.wizard
            .shell_config
            .multiplexer
            .map(|m| m.name())
            .unwrap_or("None"),
    ));
    lines.push(Line::from(""));

    // Apps
    lines.push(section_header("APPLICATIONS"));
    let selected_str = format!("{} apps", app.wizard.selected_app_count());
    lines.push(review_line("  Selected", &selected_str));
    let time_str = format!("~{} minutes", app.wizard.estimated_install_time());
    lines.push(review_line("  Est. time", &time_str));
    lines.push(Line::from(""));

    for cat in Category::all() {
        let selected_apps = app.wizard.get_selected_apps();
        let cat_apps: Vec<_> = selected_apps
            .iter()
            .filter(|a| a.category == *cat)
            .collect();
        if !cat_apps.is_empty() {
            let names: Vec<_> = cat_apps.iter().map(|a| a.name).collect();
            lines.push(Line::from(vec![
                Span::styled(format!("  {} ", cat.icon()), HackerTheme::dim()),
                Span::styled(
                    format!("{}: ", cat.name()),
                    Style::default().fg(Theme::SUBTEXT0),
                ),
                Span::styled(names.join(", "), HackerTheme::primary()),
            ]));
        }
    }
    lines.push(Line::from(""));

    // Git & GitHub
    lines.push(section_header("GIT & GITHUB"));
    if !app.wizard.identity.name.is_empty() {
        let identity_str = format!(
            "{} <{}>",
            app.wizard.identity.name, app.wizard.identity.email
        );
        lines.push(review_line("  Identity", &identity_str));
    }
    if app.wizard.generate_ssh_key {
        lines.push(review_line(
            "  SSH key",
            "Generate ed25519 (or use existing)",
        ));
    }
    if app.wizard.selected_apps.contains("gh") {
        lines.push(review_line("  GitHub CLI", "Configure auth (post-install)"));
    }
    if app.wizard.setup_git_signing {
        lines.push(review_line("  GPG signing", "Configure (or guide)"));
    }
    if app.wizard.selected_apps.contains("delta") {
        lines.push(review_line("  Delta", "Set as git pager"));
    }
    lines.push(Line::from(""));

    // Config files
    lines.push(section_header("CONFIG FILES"));
    lines.push(Line::from(Span::styled(
        "  ~/.gitconfig",
        HackerTheme::primary(),
    )));
    if app.wizard.shell_config.shell == crate::wizard::ShellChoice::Zsh {
        lines.push(Line::from(Span::styled(
            "  ~/.zshrc",
            HackerTheme::primary(),
        )));
    }
    if app.wizard.shell_config.prompt == crate::wizard::PromptChoice::Starship
        || app.wizard.selected_apps.contains("starship")
    {
        lines.push(Line::from(Span::styled(
            "  ~/.config/starship.toml",
            HackerTheme::primary(),
        )));
    }
    if app.wizard.shell_config.multiplexer == Some(crate::wizard::MultiplexerChoice::Tmux)
        || app.wizard.selected_apps.contains("tmux")
    {
        lines.push(Line::from(Span::styled(
            "  ~/.tmux.conf",
            HackerTheme::primary(),
        )));
    }
    lines.push(Line::from(Span::styled(
        "  ~/.editorconfig",
        HackerTheme::primary(),
    )));
    lines.push(Line::from(Span::styled(
        "  (existing files backed up with .load-backup)",
        HackerTheme::muted(),
    )));

    lines.push(Line::from(""));
    lines.push(Line::from(vec![
        Span::styled(
            "  Ready to transform your machine? ",
            Style::default().fg(Theme::YELLOW),
        ),
        Span::styled("[y/enter]", HackerTheme::key_hint_key()),
    ]));

    let paragraph = Paragraph::new(lines)
        .wrap(Wrap { trim: true })
        .scroll((app.wizard.scroll_offset as u16, 0));
    frame.render_widget(paragraph, inner);

    render_footer(
        frame,
        chunks[4],
        &[("↑↓", "scroll"), ("enter/y", "install"), ("esc/n", "back")],
    );
}

// ═══════════════════════════════════════════════════════════════════════
//  Install screen
// ═══════════════════════════════════════════════════════════════════════

fn render_install(frame: &mut Frame, app: &mut App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Header
            Constraint::Length(1), // Phase indicator
            Constraint::Length(1), // Spacer
            Constraint::Length(3), // Status line + counters
            Constraint::Length(3), // Progress bar
            Constraint::Min(0),    // Log
            Constraint::Length(1), // Footer
        ])
        .split(area);

    render_header(frame, chunks[0], "REALITY MODIFICATION");
    render_phase_indicator(frame, chunks[1], &app.wizard.phase);

    // Status line — current package + timer + counters
    let status_area = centered_rect(85, 100, chunks[3]);
    let current = app.current_package.as_deref().unwrap_or("Preparing...");
    let elapsed_str = if let Some(started) = app.package_started_at {
        let secs = started.elapsed().as_secs();
        if secs >= 60 {
            format!(" {}m{:02}s", secs / 60, secs % 60)
        } else if secs > 0 {
            format!(" {}s", secs)
        } else {
            String::new()
        }
    } else {
        String::new()
    };
    let elapsed_style = if app
        .package_started_at
        .map(|s| s.elapsed().as_secs() >= 30)
        .unwrap_or(false)
    {
        HackerTheme::warning()
    } else {
        HackerTheme::dim()
    };

    let status_line = Line::from(vec![
        Span::styled(
            format!(" {} ", app.spinner.current()),
            Style::default().fg(Theme::YELLOW),
        ),
        Span::styled(
            current,
            Style::default()
                .fg(Theme::TEXT)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(elapsed_str, elapsed_style),
        Span::styled("  ", Style::default()),
        Span::styled(
            format!(" {} ok ", app.install_succeeded),
            Style::default().fg(Theme::GREEN),
        ),
        Span::styled(
            format!(" {} skip ", app.install_skipped),
            Style::default().fg(Theme::YELLOW),
        ),
        Span::styled(
            format!(" {} fail ", app.install_failed),
            if app.install_failed > 0 {
                Style::default().fg(Theme::RED)
            } else {
                HackerTheme::muted()
            },
        ),
        Span::styled("  ", Style::default()),
        Span::styled(
            format!("{}/{}", app.install_completed, app.install_total),
            Style::default().fg(Theme::LAVENDER),
        ),
    ]);
    let status_paragraph = Paragraph::new(status_line);
    frame.render_widget(status_paragraph, status_area);

    // Progress bar
    let progress_area = centered_rect(85, 100, chunks[4]);
    let pct = (app.install_progress as u16).min(100);
    let gauge = Gauge::default()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(ratatui::widgets::BorderType::Rounded)
                .border_style(HackerTheme::border()),
        )
        .gauge_style(Style::default().fg(Theme::GREEN).bg(Theme::SURFACE0))
        .percent(pct)
        .label(Span::styled(
            format!("{:.0}%", app.install_progress),
            Style::default()
                .fg(Theme::TEXT)
                .add_modifier(Modifier::BOLD),
        ));
    frame.render_widget(gauge, progress_area);

    // Log output
    let log_area = centered_rect(85, 100, chunks[5]);
    let log_block = Block::default()
        .borders(Borders::ALL)
        .border_type(ratatui::widgets::BorderType::Rounded)
        .border_style(HackerTheme::border())
        .style(Style::default().bg(Theme::MANTLE));

    let log_inner = log_block.inner(log_area);
    frame.render_widget(log_block, log_area);

    let log_lines: Vec<Line> = app
        .install_log
        .iter()
        .rev()
        .take(log_inner.height as usize)
        .rev()
        .map(|s| {
            let style = if s.starts_with("[OK]") || s.contains("[COMPLETE]") {
                Style::default().fg(Theme::GREEN)
            } else if s.starts_with("[FAIL]") || s.starts_with("[FATAL]") || s.contains("[ERROR]") {
                Style::default().fg(Theme::RED)
            } else if s.starts_with("[SKIP]") || s.contains("[WARN]") || s.starts_with("[ABORT]") {
                Style::default().fg(Theme::YELLOW)
            } else if s.starts_with("[PHASE]") {
                Style::default()
                    .fg(Theme::BLUE)
                    .add_modifier(Modifier::BOLD)
            } else if s.starts_with("[DONE]") {
                Style::default()
                    .fg(Theme::TEAL)
                    .add_modifier(Modifier::BOLD)
            } else if s.starts_with("[INSTALL]") || s.starts_with("[RUN]") {
                Style::default().fg(Theme::SUBTEXT0)
            } else if s.starts_with("  ") {
                HackerTheme::muted()
            } else {
                HackerTheme::primary()
            };
            Line::from(Span::styled(s.as_str(), style))
        })
        .collect();

    let log_paragraph = Paragraph::new(log_lines);
    frame.render_widget(log_paragraph, log_inner);

    render_footer(frame, chunks[6], &[("ctrl+c", "abort")]);
}

// ═══════════════════════════════════════════════════════════════════════
//  Complete screen
// ═══════════════════════════════════════════════════════════════════════

fn render_complete(frame: &mut Frame, app: &mut App, area: Rect) {
    // Matrix rain background — callback to the boot screen
    frame.render_widget(&app.matrix_rain, area);

    let center = centered_rect(60, 65, area);
    frame.render_widget(Clear, center);

    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(ratatui::widgets::BorderType::Rounded)
        .border_style(Style::default().fg(Theme::GREEN))
        .style(Style::default().bg(Color::Rgb(10, 10, 15)));

    let inner = block.inner(center);
    frame.render_widget(block, center);

    let has_failures = app.install_failed > 0;

    let mut text: Vec<Line> = Vec::new();

    // ─── C64 header ─────────────────────────────────────────────
    text.push(Line::from(""));
    text.push(Line::from(Span::styled(
        "    LOAD\"*\",8,1",
        Style::default()
            .fg(Theme::PHOSPHOR)
            .add_modifier(Modifier::BOLD),
    )));
    text.push(Line::from(Span::styled(
        "    SEARCHING FOR *",
        Style::default().fg(Theme::PHOSPHOR_DIM),
    )));
    text.push(Line::from(Span::styled(
        "    LOADING",
        Style::default().fg(Theme::PHOSPHOR_DIM),
    )));
    text.push(Line::from(""));

    // ─── Divider ────────────────────────────────────────────────
    text.push(Line::from(Span::styled(
        "    ────────────────────────────────────────",
        Style::default().fg(Theme::SURFACE1),
    )));
    text.push(Line::from(""));

    // ─── Summary stats — prominent ──────────────────────────────
    text.push(Line::from(vec![
        Span::styled("    ", Style::default()),
        Span::styled(
            format!("{}", app.install_succeeded),
            Style::default()
                .fg(Theme::GREEN)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(" installed", Style::default().fg(Theme::SUBTEXT0)),
        Span::styled("   ", Style::default()),
        Span::styled(
            format!("{}", app.install_skipped),
            Style::default().fg(Theme::YELLOW),
        ),
        Span::styled(" skipped", Style::default().fg(Theme::SUBTEXT0)),
        Span::styled("   ", Style::default()),
        Span::styled(
            format!("{}", app.install_failed),
            if has_failures {
                Style::default().fg(Theme::RED).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Theme::SURFACE2)
            },
        ),
        Span::styled(
            " failed",
            if has_failures {
                Style::default().fg(Theme::RED)
            } else {
                Style::default().fg(Theme::SURFACE2)
            },
        ),
    ]));
    text.push(Line::from(""));

    if has_failures {
        text.push(Line::from(Span::styled(
            "    ?PACKAGE ERROR  — check logs above",
            Style::default().fg(Theme::RED),
        )));
        text.push(Line::from(""));
    }

    // ─── Config files generated ─────────────────────────────────
    let config_count: usize = [
        true, // .gitconfig always
        app.wizard.shell_config.shell == crate::wizard::ShellChoice::Zsh,
        app.wizard.shell_config.prompt == crate::wizard::PromptChoice::Starship
            || app.wizard.selected_apps.contains("starship"),
        app.wizard.shell_config.multiplexer == Some(crate::wizard::MultiplexerChoice::Tmux)
            || app.wizard.selected_apps.contains("tmux"),
        true, // .editorconfig always
    ]
    .iter()
    .filter(|&&b| b)
    .count();

    text.push(Line::from(vec![
        Span::styled("    ", Style::default()),
        Span::styled(
            format!("{}", config_count),
            Style::default()
                .fg(Theme::LAVENDER)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(
            " config files generated",
            Style::default().fg(Theme::SUBTEXT0),
        ),
    ]));
    text.push(Line::from(""));

    // ─── Divider ────────────────────────────────────────────────
    text.push(Line::from(Span::styled(
        "    ────────────────────────────────────────",
        Style::default().fg(Theme::SURFACE1),
    )));
    text.push(Line::from(""));

    // ─── Next steps ─────────────────────────────────────────────
    text.push(Line::from(Span::styled(
        "    NEXT STEPS",
        Style::default()
            .fg(Theme::LAVENDER)
            .add_modifier(Modifier::BOLD),
    )));
    text.push(Line::from(""));

    text.push(Line::from(vec![
        Span::styled("      ", Style::default()),
        Span::styled("1 ", Style::default().fg(Theme::SURFACE2)),
        Span::styled("exec $SHELL", Style::default().fg(Theme::TEXT)),
        Span::styled("  — reload your shell", HackerTheme::dim()),
    ]));

    let mut step = 2;

    if app.wizard.selected_apps.contains("gh")
        && !app
            .install_log
            .iter()
            .any(|l| l.contains("Already authenticated"))
    {
        text.push(Line::from(vec![
            Span::styled("      ", Style::default()),
            Span::styled(format!("{} ", step), Style::default().fg(Theme::SURFACE2)),
            Span::styled("gh auth login", Style::default().fg(Theme::TEXT)),
            Span::styled("  — authenticate GitHub CLI", HackerTheme::dim()),
        ]));
        step += 1;
    }

    if app.wizard.generate_ssh_key
        && app
            .install_log
            .iter()
            .any(|l| l.contains("SSH key generated"))
    {
        text.push(Line::from(vec![
            Span::styled("      ", Style::default()),
            Span::styled(format!("{} ", step), Style::default().fg(Theme::SURFACE2)),
            Span::styled("ssh -T git@github.com", Style::default().fg(Theme::TEXT)),
            Span::styled("  — test SSH connection", HackerTheme::dim()),
        ]));
        step += 1;
    }

    // Suppress unused variable warning — step is intentionally incremented for future use
    let _ = step;

    text.push(Line::from(""));

    // ─── READY. — the sign-off ──────────────────────────────────
    text.push(Line::from(Span::styled(
        "    ────────────────────────────────────────",
        Style::default().fg(Theme::SURFACE1),
    )));
    text.push(Line::from(""));
    text.push(Line::from(Span::styled(
        "    READY.",
        Style::default()
            .fg(Theme::PHOSPHOR)
            .add_modifier(Modifier::BOLD),
    )));
    text.push(Line::from(Span::styled(
        "    █",
        Style::default().fg(Theme::PHOSPHOR),
    )));
    text.push(Line::from(""));
    text.push(Line::from(Span::styled(
        "    Press ENTER or q to exit",
        Style::default().fg(Theme::SURFACE2),
    )));

    let paragraph = Paragraph::new(text);
    frame.render_widget(paragraph, inner);
}

// ═══════════════════════════════════════════════════════════════════════
//  Shared components
// ═══════════════════════════════════════════════════════════════════════

fn render_header(frame: &mut Frame, area: Rect, title: &str) {
    let block = Block::default()
        .borders(Borders::BOTTOM)
        .border_style(HackerTheme::border_dim())
        .border_set(symbols::border::Set {
            bottom_left: "─",
            bottom_right: "─",
            horizontal_bottom: "─",
            horizontal_top: "─",
            ..symbols::border::PLAIN
        });

    let header_line = Line::from(vec![
        Span::raw("  "),
        Span::styled("LOAD\"*\",8,1", HackerTheme::brand()),
        Span::styled("  ", Style::default()),
        Span::styled(symbols::line::VERTICAL, HackerTheme::border_dim()),
        Span::styled("  ", Style::default()),
        Span::styled(title, HackerTheme::title()),
    ]);

    let inner = block.inner(area);
    frame.render_widget(block, area);
    // Render on second row of the 3-row header area for vertical centering
    if inner.height > 0 {
        let text_area = Rect {
            y: inner.y + inner.height.saturating_sub(1).min(1),
            height: 1,
            ..inner
        };
        frame.render_widget(Paragraph::new(header_line), text_area);
    }
}

fn render_phase_indicator(frame: &mut Frame, area: Rect, current: &WizardPhase) {
    let phases = WizardPhase::all();
    let mut spans: Vec<Span> = Vec::new();

    spans.push(Span::raw("  "));

    for (i, phase) in phases.iter().enumerate() {
        if i > 0 {
            spans.push(Span::styled(" ── ", HackerTheme::border_dim()));
        }

        let (icon, style) = if phase == current {
            (
                "●",
                Style::default()
                    .fg(Theme::BLUE)
                    .add_modifier(Modifier::BOLD),
            )
        } else if phase.index() < current.index() {
            ("●", Style::default().fg(Theme::GREEN))
        } else {
            ("○", HackerTheme::muted())
        };

        spans.push(Span::styled(icon, style));

        // Only show name for current and adjacent phases to save space
        let dist = (phase.index() as i32 - current.index() as i32).unsigned_abs() as usize;
        if dist <= 1 {
            spans.push(Span::styled(format!(" {}", phase.name()), style));
        }
    }

    let line = Line::from(spans);
    frame.render_widget(Paragraph::new(line), area);
}

fn render_footer(frame: &mut Frame, area: Rect, keys: &[(&str, &str)]) {
    let mut spans: Vec<Span> = vec![Span::raw("  ")];

    for (i, (key, action)) in keys.iter().enumerate() {
        if i > 0 {
            spans.push(Span::styled("  ", Style::default()));
        }
        spans.push(Span::styled(*key, HackerTheme::key_hint_key()));
        spans.push(Span::styled(
            format!(" {}", action),
            HackerTheme::key_hint(),
        ));
    }

    let line = Line::from(spans);
    let paragraph = Paragraph::new(line).style(Style::default().bg(Theme::MANTLE));
    frame.render_widget(paragraph, area);
}

fn render_input_field(frame: &mut Frame, area: Rect, label: &str, value: &str, focused: bool) {
    let border_style = if focused {
        HackerTheme::border_focused()
    } else {
        HackerTheme::border()
    };

    let label_style = if focused {
        Style::default()
            .fg(Theme::BLUE)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Theme::OVERLAY1)
    };

    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(ratatui::widgets::BorderType::Rounded)
        .border_style(border_style)
        .title(Span::styled(format!(" {} ", label), label_style));

    let inner = block.inner(area);
    frame.render_widget(block, area);

    let prompt = if focused { "> " } else { "  " };
    let cursor = if focused { "█" } else { "" };

    let text = Paragraph::new(Line::from(vec![
        Span::styled(
            prompt,
            if focused {
                HackerTheme::accent()
            } else {
                HackerTheme::muted()
            },
        ),
        Span::styled(value, HackerTheme::primary()),
        Span::styled(cursor, Style::default().fg(Theme::BLUE)),
    ]));
    frame.render_widget(text, inner);
}

fn render_setup_type_selector(frame: &mut Frame, area: Rect, current: SetupType, focused: bool) {
    let border_style = if focused {
        HackerTheme::border_focused()
    } else {
        HackerTheme::border()
    };

    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(ratatui::widgets::BorderType::Rounded)
        .border_style(border_style)
        .title(Span::styled(
            " SETUP TYPE ",
            if focused {
                Style::default()
                    .fg(Theme::BLUE)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Theme::OVERLAY1)
            },
        ));

    let inner = block.inner(area);
    frame.render_widget(block, area);

    let types = SetupType::all();
    let mut spans: Vec<Span> = Vec::new();

    for t in types {
        let style = if *t == current {
            Style::default()
                .fg(Theme::GREEN)
                .add_modifier(Modifier::BOLD)
        } else {
            HackerTheme::dim()
        };

        let indicator = if *t == current { "●" } else { "○" };
        spans.push(Span::styled(
            format!(" {} {} {} ", indicator, t.icon(), t.name()),
            style,
        ));
    }

    let line1 = Line::from(spans);
    let line2 = Line::from(Span::styled(
        format!("  {}", current.description()),
        Style::default().fg(Theme::SUBTEXT0),
    ));

    let text = Paragraph::new(vec![line1, line2]);
    frame.render_widget(text, inner);
}

fn render_option_selector(
    frame: &mut Frame,
    area: Rect,
    label: &str,
    value: &str,
    description: &str,
    focused: bool,
) {
    let border_style = if focused {
        HackerTheme::border_focused()
    } else {
        HackerTheme::border()
    };

    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(ratatui::widgets::BorderType::Rounded)
        .border_style(border_style)
        .title(Span::styled(
            format!(" {} ", label),
            if focused {
                Style::default()
                    .fg(Theme::BLUE)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Theme::OVERLAY1)
            },
        ));

    let inner = block.inner(area);
    frame.render_widget(block, area);

    let arrow_l = if focused { "◂ " } else { "  " };
    let arrow_r = if focused { " ▸" } else { "  " };

    let lines = vec![
        Line::from(vec![
            Span::styled(arrow_l, Style::default().fg(Theme::BLUE)),
            Span::styled(
                value,
                Style::default()
                    .fg(Theme::GREEN)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(arrow_r, Style::default().fg(Theme::BLUE)),
        ]),
        Line::from(Span::styled(
            description,
            Style::default().fg(Theme::SUBTEXT0),
        )),
    ];

    let text = Paragraph::new(lines);
    frame.render_widget(text, inner);
}

fn render_category_sidebar(
    frame: &mut Frame,
    area: Rect,
    categories: &[Category],
    selected: usize,
) {
    let block = Block::default()
        .borders(Borders::RIGHT)
        .border_style(HackerTheme::border_dim());

    let inner = block.inner(area);
    frame.render_widget(block, area);

    let items: Vec<ListItem> = categories
        .iter()
        .enumerate()
        .map(|(i, cat)| {
            let is_selected = i == selected % categories.len();
            let style = if is_selected {
                HackerTheme::selected()
            } else {
                HackerTheme::primary()
            };
            let prefix = if is_selected { " ▸ " } else { "   " };
            ListItem::new(Line::from(vec![
                Span::styled(
                    prefix,
                    if is_selected {
                        Style::default().fg(Theme::BLUE)
                    } else {
                        Style::default()
                    },
                ),
                Span::styled(format!("{} {}", cat.icon(), cat.name()), style),
            ]))
        })
        .collect();

    let list = List::new(items);
    frame.render_widget(list, inner);
}

fn render_app_list(frame: &mut Frame, area: Rect, category: &Category, app: &App) {
    let apps = catalog::apps_by_category(category);

    let block = Block::default()
        .borders(Borders::NONE)
        .style(Style::default());

    let inner = block.inner(area);
    frame.render_widget(block, area);

    let items: Vec<ListItem> = apps
        .iter()
        .enumerate()
        .map(|(i, a)| {
            let selected = app.wizard.is_app_selected(a.id);
            let focused = i == app.wizard.cursor_position;

            let checkbox = if selected { "◉" } else { "○" };
            let checkbox_style = if selected {
                Style::default().fg(Theme::GREEN)
            } else {
                HackerTheme::muted()
            };

            let name_style = if focused && selected {
                HackerTheme::selected_accent()
            } else if focused {
                HackerTheme::selected()
            } else if selected {
                Style::default().fg(Theme::GREEN)
            } else {
                HackerTheme::primary()
            };

            let mut spans = vec![
                Span::styled(
                    if focused { " ▸ " } else { "   " },
                    if focused {
                        Style::default().fg(Theme::BLUE)
                    } else {
                        Style::default()
                    },
                ),
                Span::styled(format!("{} ", checkbox), checkbox_style),
                Span::styled(a.name, name_style),
            ];

            if app.wizard.show_details {
                spans.push(Span::styled(
                    format!("  {}", a.description),
                    Style::default().fg(Theme::OVERLAY0),
                ));
            }

            ListItem::new(Line::from(spans))
        })
        .collect();

    let list = List::new(items);
    frame.render_widget(list, inner);
}

// ─── Review helpers ──────────────────────────────────────────────────

fn section_header(title: &str) -> Line<'static> {
    Line::from(vec![
        Span::styled("  ── ", HackerTheme::border_dim()),
        Span::styled(
            title.to_string(),
            Style::default()
                .fg(Theme::LAVENDER)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(
            " ──────────────────────────────────────────",
            HackerTheme::border_dim(),
        ),
    ])
}

fn review_line(label: &str, value: &str) -> Line<'static> {
    Line::from(vec![
        Span::styled(format!("{}  ", label), Style::default().fg(Theme::SUBTEXT0)),
        Span::styled(value.to_string(), HackerTheme::primary()),
    ])
}

// ─── Layout helper ───────────────────────────────────────────────────

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
