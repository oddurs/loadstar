//! Rendering logic for the wizard UI
//! Strange, artful, hacker-core aesthetics

use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Gauge, List, ListItem, Paragraph, Wrap},
    Frame,
};

use crate::{
    catalog::{self, Category},
    effects::HackerTheme,
    wizard::{SetupType, WizardPhase},
    App,
};

pub fn render_app(frame: &mut Frame, app: &mut App) {
    let size = frame.size();

    // Clear with dark background
    let bg = Block::default().style(Style::default().bg(Color::Rgb(5, 5, 10)));
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

fn render_boot(frame: &mut Frame, app: &mut App, area: Rect) {
    // Matrix rain background
    frame.render_widget(&app.matrix_rain, area);

    // Center content
    let center = centered_rect(60, 50, area);
    frame.render_widget(Clear, center);

    // Boot sequence box
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Rgb(0, 100, 0)))
        .title("[ SYSTEM BOOT ]")
        .title_style(HackerTheme::title());

    let inner = block.inner(center);
    frame.render_widget(block, center);

    // Boot messages
    let mut lines: Vec<Line> = Vec::new();
    lines.push(Line::from(""));

    for (i, (msg, complete)) in app.boot_sequence.messages.iter().enumerate() {
        let status = if *complete {
            Span::styled("[OK]", Style::default().fg(Color::Green))
        } else if i == app.boot_sequence.stage {
            Span::styled(
                format!("[{}]", app.spinner.current()),
                Style::default().fg(Color::Yellow),
            )
        } else {
            Span::styled("[  ]", Style::default().fg(Color::DarkGray))
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
            Span::raw("  "),
            status,
            Span::raw(" "),
            Span::styled(text, HackerTheme::primary()),
        ]));
    }

    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        "  Press any key to skip...",
        Style::default().fg(Color::DarkGray),
    )));

    let paragraph = Paragraph::new(lines);
    frame.render_widget(paragraph, inner);
}

fn render_identity(frame: &mut Frame, app: &mut App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(8), // Header
            Constraint::Length(3), // Phase indicator
            Constraint::Length(2), // System info
            Constraint::Min(0),    // Content
            Constraint::Length(3), // Footer
        ])
        .split(area);

    render_header(frame, chunks[0], "IDENTITY MATRIX");
    render_phase_indicator(frame, chunks[1], &app.wizard.phase);

    // System info bar
    let sys = &app.system;
    let brew_status = if sys.has_homebrew() {
        Span::styled("brew ✓", HackerTheme::success())
    } else {
        Span::styled("brew ✗", HackerTheme::warning())
    };
    let sys_info = Line::from(vec![
        Span::styled("  System: ", HackerTheme::dim()),
        Span::styled(
            format!("{} {} ", sys.os.name(), sys.arch.name()),
            HackerTheme::primary(),
        ),
        Span::styled("│ ", HackerTheme::dim()),
        brew_status,
        Span::styled(" │ ", HackerTheme::dim()),
        Span::styled(format!("~{}", sys.hostname), HackerTheme::dim()),
    ]);
    frame.render_widget(Paragraph::new(sys_info), chunks[2]);

    // Main content
    let content_area = chunks[3];
    let form_area = centered_rect(70, 80, content_area);

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(HackerTheme::border())
        .title("[ NEURAL IDENTITY CONFIGURATION ]")
        .title_style(HackerTheme::title());

    let inner = block.inner(form_area);
    frame.render_widget(block, form_area);

    // Form fields
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

    // Name field
    render_input_field(
        frame,
        field_chunks[0],
        "NAME",
        &app.wizard.identity.name,
        app.wizard.input_field == 0,
    );

    // Email field
    render_input_field(
        frame,
        field_chunks[1],
        "EMAIL",
        &app.wizard.identity.email,
        app.wizard.input_field == 1,
    );

    // GitHub field
    render_input_field(
        frame,
        field_chunks[2],
        "GITHUB",
        &app.wizard.identity.github_username,
        app.wizard.input_field == 2,
    );

    // Setup type selector
    render_setup_type_selector(
        frame,
        field_chunks[3],
        app.wizard.identity.setup_type,
        app.wizard.input_field == 3,
    );

    render_footer(
        frame,
        chunks[4],
        "TAB: Next field | ENTER: Continue | ESC: Back",
    );
}

fn render_shell(frame: &mut Frame, app: &mut App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(8),
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(3),
        ])
        .split(area);

    render_header(frame, chunks[0], "COMMAND INTERFACE");
    render_phase_indicator(frame, chunks[1], &app.wizard.phase);

    let content_area = centered_rect(80, 90, chunks[2]);

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(HackerTheme::border())
        .title("[ SHELL CONFIGURATION ]")
        .title_style(HackerTheme::title());

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

    // Shell choice
    render_option_selector(
        frame,
        option_chunks[0],
        "SHELL",
        app.wizard.shell_config.shell.name(),
        app.wizard.shell_config.shell.description(),
        app.wizard.cursor_position == 0,
    );

    // Prompt choice
    render_option_selector(
        frame,
        option_chunks[1],
        "PROMPT",
        app.wizard.shell_config.prompt.name(),
        app.wizard.shell_config.prompt.description(),
        app.wizard.cursor_position == 1,
    );

    // Terminal choice
    render_option_selector(
        frame,
        option_chunks[2],
        "TERMINAL",
        app.wizard.shell_config.terminal.name(),
        app.wizard.shell_config.terminal.description(),
        app.wizard.cursor_position == 2,
    );

    // Multiplexer choice
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
        chunks[3],
        "↑↓: Navigate | ←→: Change | ENTER: Continue | ESC: Back",
    );
}

fn render_devtools(frame: &mut Frame, app: &mut App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(8),
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(3),
        ])
        .split(area);

    render_header(frame, chunks[0], "DEVELOPMENT ARSENAL");
    render_phase_indicator(frame, chunks[1], &app.wizard.phase);

    let content_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(20), Constraint::Min(0)])
        .split(chunks[2]);

    // Category sidebar
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

    // App list for selected category
    let cat = &categories[app.wizard.scroll_offset % categories.len()];
    render_app_list(frame, content_chunks[1], cat, app);

    render_footer(
        frame,
        chunks[3],
        "TAB: Category | SPACE: Toggle | a: All | n: None | ENTER: Continue",
    );
}

fn render_apps(frame: &mut Frame, app: &mut App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(8),
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(3),
        ])
        .split(area);

    render_header(frame, chunks[0], "SOFTWARE COMPANIONS");
    render_phase_indicator(frame, chunks[1], &app.wizard.phase);

    let content_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(22), Constraint::Min(0)])
        .split(chunks[2]);

    // Category sidebar with all categories
    let categories = Category::all();
    render_category_sidebar(
        frame,
        content_chunks[0],
        categories,
        app.wizard.scroll_offset,
    );

    // App list for selected category
    let cat = &categories[app.wizard.scroll_offset % categories.len()];
    render_app_list(frame, content_chunks[1], cat, app);

    render_footer(
        frame,
        chunks[3],
        "TAB/SHIFT+TAB: Category | SPACE: Toggle | d: Details | ENTER: Continue",
    );
}

fn render_review(frame: &mut Frame, app: &mut App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(8),
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(3),
        ])
        .split(area);

    render_header(frame, chunks[0], "CONFIGURATION REVIEW");
    render_phase_indicator(frame, chunks[1], &app.wizard.phase);

    let content_area = centered_rect(80, 95, chunks[2]);

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(HackerTheme::border())
        .title("[ INSTALLATION MANIFEST ]")
        .title_style(HackerTheme::title());

    let inner = block.inner(content_area);
    frame.render_widget(block, content_area);

    let mut lines: Vec<Line> = Vec::new();

    // Identity section
    lines.push(Line::from(Span::styled(
        "═══ IDENTITY ═══",
        HackerTheme::accent(),
    )));
    lines.push(Line::from(format!("  Name: {}", app.wizard.identity.name)));
    lines.push(Line::from(format!(
        "  Email: {}",
        app.wizard.identity.email
    )));
    lines.push(Line::from(format!(
        "  GitHub: {}",
        app.wizard.identity.github_username
    )));
    lines.push(Line::from(format!(
        "  Setup: {} {}",
        app.wizard.identity.setup_type.icon(),
        app.wizard.identity.setup_type.name()
    )));
    lines.push(Line::from(""));

    // Shell section
    lines.push(Line::from(Span::styled(
        "═══ SHELL ═══",
        HackerTheme::accent(),
    )));
    lines.push(Line::from(format!(
        "  Shell: {}",
        app.wizard.shell_config.shell.name()
    )));
    lines.push(Line::from(format!(
        "  Prompt: {}",
        app.wizard.shell_config.prompt.name()
    )));
    lines.push(Line::from(format!(
        "  Terminal: {}",
        app.wizard.shell_config.terminal.name()
    )));
    lines.push(Line::from(format!(
        "  Multiplexer: {}",
        app.wizard
            .shell_config
            .multiplexer
            .map(|m| m.name())
            .unwrap_or("None")
    )));
    lines.push(Line::from(""));

    // Apps section
    lines.push(Line::from(Span::styled(
        "═══ APPLICATIONS ═══",
        HackerTheme::accent(),
    )));
    lines.push(Line::from(format!(
        "  Selected: {} apps",
        app.wizard.selected_app_count()
    )));
    lines.push(Line::from(format!(
        "  Estimated time: ~{} minutes",
        app.wizard.estimated_install_time()
    )));
    lines.push(Line::from(""));

    // Selected apps by category (brief)
    for cat in Category::all() {
        let selected_apps = app.wizard.get_selected_apps();
        let cat_apps: Vec<_> = selected_apps
            .iter()
            .filter(|a| a.category == *cat)
            .collect();
        if !cat_apps.is_empty() {
            let names: Vec<_> = cat_apps.iter().map(|a| a.name).collect();
            lines.push(Line::from(format!(
                "  {} {}: {}",
                cat.icon(),
                cat.name(),
                names.join(", ")
            )));
        }
    }
    lines.push(Line::from(""));

    // Git & GitHub section
    lines.push(Line::from(Span::styled(
        "═══ GIT & GITHUB ═══",
        HackerTheme::accent(),
    )));
    if !app.wizard.identity.name.is_empty() {
        lines.push(Line::from(format!(
            "  Git identity: {} <{}>",
            app.wizard.identity.name, app.wizard.identity.email
        )));
    }
    if app.wizard.generate_ssh_key {
        lines.push(Line::from("  SSH key: Generate ed25519 (or use existing)"));
    }
    if app.wizard.selected_apps.contains("gh") {
        lines.push(Line::from("  GitHub CLI: Configure auth (post-install)"));
    }
    if app.wizard.setup_git_signing {
        lines.push(Line::from("  GPG signing: Configure (or guide)"));
    }
    if app.wizard.selected_apps.contains("delta") {
        lines.push(Line::from("  Delta: Set as git pager"));
    }
    lines.push(Line::from(""));

    // Config files section
    lines.push(Line::from(Span::styled(
        "═══ CONFIG FILES ═══",
        HackerTheme::accent(),
    )));
    lines.push(Line::from("  ~/.gitconfig"));
    if app.wizard.shell_config.shell == crate::wizard::ShellChoice::Zsh {
        lines.push(Line::from("  ~/.zshrc"));
    }
    if app.wizard.shell_config.prompt == crate::wizard::PromptChoice::Starship
        || app.wizard.selected_apps.contains("starship")
    {
        lines.push(Line::from("  ~/.config/starship.toml"));
    }
    if app.wizard.shell_config.multiplexer == Some(crate::wizard::MultiplexerChoice::Tmux)
        || app.wizard.selected_apps.contains("tmux")
    {
        lines.push(Line::from("  ~/.tmux.conf"));
    }
    lines.push(Line::from("  ~/.editorconfig"));
    lines.push(Line::from(Span::styled(
        "  (existing files backed up with .load-backup)",
        HackerTheme::dim(),
    )));

    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        "Ready to transform your machine?",
        Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD),
    )));

    let paragraph = Paragraph::new(lines)
        .wrap(Wrap { trim: true })
        .scroll((app.wizard.scroll_offset as u16, 0));
    frame.render_widget(paragraph, inner);

    render_footer(
        frame,
        chunks[3],
        "↑↓: Scroll | ENTER/y: Begin Installation | ESC/n: Go Back",
    );
}

fn render_install(frame: &mut Frame, app: &mut App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(8), // Header
            Constraint::Length(3), // Phase indicator
            Constraint::Length(3), // Status line (current package + counters)
            Constraint::Length(3), // Progress bar
            Constraint::Min(0),    // Log
            Constraint::Length(3), // Footer
        ])
        .split(area);

    render_header(frame, chunks[0], "REALITY MODIFICATION");
    render_phase_indicator(frame, chunks[1], &app.wizard.phase);

    // Status line — current package + counters
    let status_area = centered_rect(80, 100, chunks[2]);
    let current = app.current_package.as_deref().unwrap_or("Preparing...");
    let elapsed_str = if let Some(started) = app.package_started_at {
        let secs = started.elapsed().as_secs();
        if secs >= 60 {
            format!(" {}m{:02}s", secs / 60, secs % 60)
        } else {
            format!(" {}s", secs)
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
            format!("{} ", app.spinner.current()),
            Style::default().fg(Color::Yellow),
        ),
        Span::styled(current, HackerTheme::primary()),
        Span::styled(elapsed_str, elapsed_style),
        Span::styled("  │  ", HackerTheme::dim()),
        Span::styled(format!("{}", app.install_succeeded), HackerTheme::success()),
        Span::styled(" ok", HackerTheme::dim()),
        Span::styled("  ", HackerTheme::dim()),
        Span::styled(format!("{}", app.install_skipped), HackerTheme::warning()),
        Span::styled(" skip", HackerTheme::dim()),
        Span::styled("  ", HackerTheme::dim()),
        Span::styled(format!("{}", app.install_failed), HackerTheme::error()),
        Span::styled(" fail", HackerTheme::dim()),
        Span::styled("  │  ", HackerTheme::dim()),
        Span::styled(
            format!("{}/{}", app.install_completed, app.install_total),
            HackerTheme::accent(),
        ),
    ]);
    let status_paragraph = Paragraph::new(status_line);
    frame.render_widget(status_paragraph, status_area);

    // Progress bar
    let progress_area = centered_rect(80, 100, chunks[3]);
    let pct = (app.install_progress as u16).min(100);
    let gauge = Gauge::default()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(HackerTheme::border()),
        )
        .gauge_style(Style::default().fg(Color::Green).bg(Color::Rgb(20, 20, 30)))
        .percent(pct)
        .label(format!("{:.0}%", app.install_progress));
    frame.render_widget(gauge, progress_area);

    // Log output
    let log_area = centered_rect(80, 100, chunks[4]);
    let log_block = Block::default()
        .borders(Borders::ALL)
        .border_style(HackerTheme::border())
        .title("[ INSTALLATION LOG ]")
        .title_style(HackerTheme::title());

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
                HackerTheme::success()
            } else if s.starts_with("[FAIL]") || s.starts_with("[FATAL]") || s.contains("[ERROR]") {
                HackerTheme::error()
            } else if s.starts_with("[SKIP]") || s.contains("[WARN]") {
                HackerTheme::warning()
            } else if s.starts_with("[PHASE]") {
                HackerTheme::accent()
            } else if s.starts_with("[DONE]") {
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD)
            } else if s.starts_with("  ") {
                HackerTheme::dim()
            } else {
                HackerTheme::primary()
            };
            Line::from(Span::styled(s.as_str(), style))
        })
        .collect();

    let log_paragraph = Paragraph::new(log_lines);
    frame.render_widget(log_paragraph, log_inner);

    render_footer(
        frame,
        chunks[5],
        "Installation in progress... Ctrl+C to abort.",
    );
}

fn render_complete(frame: &mut Frame, app: &mut App, area: Rect) {
    // Matrix rain background
    frame.render_widget(&app.matrix_rain, area);

    let center = centered_rect(70, 70, area);
    frame.render_widget(Clear, center);

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Green))
        .title("[ TRANSFORMATION COMPLETE ]")
        .title_style(
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
        );

    let inner = block.inner(center);
    frame.render_widget(block, center);

    let has_failures = app.install_failed > 0;

    let mut text = vec![
        Line::from(""),
        Line::from(Span::styled(
            "    ██████╗ ██████╗ ███╗   ███╗██████╗ ██╗     ███████╗████████╗███████╗",
            Style::default().fg(Color::Green),
        )),
        Line::from(Span::styled(
            "   ██╔════╝██╔═══██╗████╗ ████║██╔══██╗██║     ██╔════╝╚══██╔══╝██╔════╝",
            Style::default().fg(Color::Green),
        )),
        Line::from(Span::styled(
            "   ██║     ██║   ██║██╔████╔██║██████╔╝██║     █████╗     ██║   █████╗  ",
            Style::default().fg(Color::LightGreen),
        )),
        Line::from(Span::styled(
            "   ██║     ██║   ██║██║╚██╔╝██║██╔═══╝ ██║     ██╔══╝     ██║   ██╔══╝  ",
            Style::default().fg(Color::LightGreen),
        )),
        Line::from(Span::styled(
            "   ╚██████╗╚██████╔╝██║ ╚═╝ ██║██║     ███████╗███████╗   ██║   ███████╗",
            Style::default().fg(Color::White),
        )),
        Line::from(Span::styled(
            "    ╚═════╝ ╚═════╝ ╚═╝     ╚═╝╚═╝     ╚══════╝╚══════╝   ╚═╝   ╚══════╝",
            Style::default().fg(Color::White),
        )),
        Line::from(""),
    ];

    // Install summary
    text.push(Line::from(vec![
        Span::styled("  Installed: ", HackerTheme::dim()),
        Span::styled(format!("{}", app.install_succeeded), HackerTheme::success()),
        Span::styled("  Skipped: ", HackerTheme::dim()),
        Span::styled(format!("{}", app.install_skipped), HackerTheme::warning()),
        Span::styled("  Failed: ", HackerTheme::dim()),
        Span::styled(
            format!("{}", app.install_failed),
            if has_failures {
                HackerTheme::error()
            } else {
                HackerTheme::dim()
            },
        ),
    ]));
    text.push(Line::from(""));

    if has_failures {
        text.push(Line::from(Span::styled(
            "Some packages failed to install. Check the log above.",
            HackerTheme::warning(),
        )));
        text.push(Line::from(""));
    }

    text.push(Line::from(Span::styled(
        "Your machine has been transformed.",
        Style::default().fg(Color::Cyan),
    )));
    text.push(Line::from(""));
    text.push(Line::from("Next steps:"));
    text.push(Line::from(Span::styled(
        "  • Restart your terminal: exec $SHELL",
        HackerTheme::dim(),
    )));

    // Dynamic next steps based on what was set up
    if app.wizard.selected_apps.contains("gh")
        && !app
            .install_log
            .iter()
            .any(|l| l.contains("Already authenticated"))
    {
        text.push(Line::from(Span::styled(
            "  • Authenticate GitHub CLI: gh auth login",
            HackerTheme::dim(),
        )));
    }

    if app.wizard.selected_apps.contains("starship") {
        text.push(Line::from(Span::styled(
            "  • Verify prompt: which starship",
            HackerTheme::dim(),
        )));
    }

    text.push(Line::from(""));
    text.push(Line::from(Span::styled(
        "Welcome to the other side.",
        Style::default()
            .fg(Color::Magenta)
            .add_modifier(Modifier::ITALIC),
    )));
    text.push(Line::from(""));
    text.push(Line::from(Span::styled(
        "Press ENTER to exit...",
        Style::default().fg(Color::DarkGray),
    )));

    let paragraph = Paragraph::new(text).alignment(Alignment::Center);
    frame.render_widget(paragraph, inner);
}

// Helper rendering functions

fn render_header(frame: &mut Frame, area: Rect, title: &str) {
    let header_text = vec![
        Line::from(Span::styled(
            "╔═══════════════════════════════════════════════════════════════════════════╗",
            HackerTheme::border(),
        )),
        Line::from(vec![
            Span::styled("║  ", HackerTheme::border()),
            Span::styled(
                "◉ LOAD\"*\",8,1",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(" │ ", HackerTheme::dim()),
            Span::styled(title, HackerTheme::accent()),
            Span::raw(" ".repeat(50_usize.saturating_sub(title.len()))),
            Span::styled("║", HackerTheme::border()),
        ]),
        Line::from(Span::styled(
            "╚═══════════════════════════════════════════════════════════════════════════╝",
            HackerTheme::border(),
        )),
    ];

    let header = Paragraph::new(header_text).alignment(Alignment::Center);
    frame.render_widget(header, area);
}

fn render_phase_indicator(frame: &mut Frame, area: Rect, current: &WizardPhase) {
    let phases = WizardPhase::all();
    let mut spans: Vec<Span> = Vec::new();

    spans.push(Span::raw("  "));

    for (i, phase) in phases.iter().enumerate() {
        if i > 0 {
            spans.push(Span::styled(" ─── ", HackerTheme::dim()));
        }

        let (icon, style) = if phase == current {
            ("◉", HackerTheme::accent())
        } else if phase.index() < current.index() {
            ("✓", HackerTheme::success())
        } else {
            ("○", HackerTheme::dim())
        };

        spans.push(Span::styled(icon, style));
        spans.push(Span::styled(format!(" {}", phase.name()), style));
    }

    let line = Line::from(spans);
    let paragraph = Paragraph::new(line).alignment(Alignment::Center);
    frame.render_widget(paragraph, area);
}

fn render_footer(frame: &mut Frame, area: Rect, help_text: &str) {
    let footer = Paragraph::new(Line::from(vec![
        Span::styled("  ", HackerTheme::dim()),
        Span::styled(help_text, HackerTheme::dim()),
    ]));

    let block = Block::default()
        .borders(Borders::TOP)
        .border_style(HackerTheme::border());

    frame.render_widget(block, area);
    frame.render_widget(footer, area);
}

fn render_input_field(frame: &mut Frame, area: Rect, label: &str, value: &str, focused: bool) {
    let style = if focused {
        Style::default().fg(Color::Cyan)
    } else {
        HackerTheme::dim()
    };

    let border_style = if focused {
        Style::default().fg(Color::Cyan)
    } else {
        HackerTheme::border()
    };

    let cursor = if focused { "█" } else { "" };

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(border_style)
        .title(format!("[ {} ]", label))
        .title_style(style);

    let inner = block.inner(area);
    frame.render_widget(block, area);

    let text = Paragraph::new(format!("{}{}", value, cursor)).style(HackerTheme::primary());
    frame.render_widget(text, inner);
}

fn render_setup_type_selector(frame: &mut Frame, area: Rect, current: SetupType, focused: bool) {
    let border_style = if focused {
        Style::default().fg(Color::Cyan)
    } else {
        HackerTheme::border()
    };

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(border_style)
        .title("[ SETUP TYPE ]")
        .title_style(if focused {
            HackerTheme::accent()
        } else {
            HackerTheme::dim()
        });

    let inner = block.inner(area);
    frame.render_widget(block, area);

    let types = SetupType::all();
    let mut spans: Vec<Span> = Vec::new();

    for t in types {
        let style = if *t == current {
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD)
        } else {
            HackerTheme::dim()
        };

        spans.push(Span::styled(format!(" {} {} ", t.icon(), t.name()), style));
        spans.push(Span::raw(" "));
    }

    let line1 = Line::from(spans);
    let line2 = Line::from(Span::styled(
        format!("  {}", current.description()),
        HackerTheme::dim(),
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
        Style::default().fg(Color::Cyan)
    } else {
        HackerTheme::border()
    };

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(border_style)
        .title(format!("[ {} ]", label))
        .title_style(if focused {
            HackerTheme::accent()
        } else {
            HackerTheme::dim()
        });

    let inner = block.inner(area);
    frame.render_widget(block, area);

    let arrow = if focused { "◀ " } else { "  " };
    let arrow_r = if focused { " ▶" } else { "  " };

    let lines = vec![
        Line::from(vec![
            Span::styled(arrow, HackerTheme::accent()),
            Span::styled(
                value,
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(arrow_r, HackerTheme::accent()),
        ]),
        Line::from(Span::styled(description, HackerTheme::dim())),
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
        .borders(Borders::ALL)
        .border_style(HackerTheme::border())
        .title("[ CATEGORIES ]")
        .title_style(HackerTheme::title());

    let inner = block.inner(area);
    frame.render_widget(block, area);

    let items: Vec<ListItem> = categories
        .iter()
        .enumerate()
        .map(|(i, cat)| {
            let style = if i == selected % categories.len() {
                HackerTheme::selected()
            } else {
                HackerTheme::primary()
            };
            ListItem::new(format!("{} {}", cat.icon(), cat.name())).style(style)
        })
        .collect();

    let list = List::new(items);
    frame.render_widget(list, inner);
}

fn render_app_list(frame: &mut Frame, area: Rect, category: &Category, app: &App) {
    let apps = catalog::apps_by_category(category);

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(HackerTheme::border())
        .title(format!("[ {} ]", category.name().to_uppercase()))
        .title_style(HackerTheme::title());

    let inner = block.inner(area);
    frame.render_widget(block, area);

    let items: Vec<ListItem> = apps
        .iter()
        .enumerate()
        .map(|(i, a)| {
            let selected = app.wizard.is_app_selected(a.id);
            let focused = i == app.wizard.cursor_position;

            let checkbox = if selected { "[✓]" } else { "[ ]" };
            let style = if focused {
                HackerTheme::selected()
            } else if selected {
                Style::default().fg(Color::Green)
            } else {
                HackerTheme::primary()
            };

            let content = if app.wizard.show_details {
                format!("{} {} - {}", checkbox, a.name, a.description)
            } else {
                format!("{} {}", checkbox, a.name)
            };

            ListItem::new(content).style(style)
        })
        .collect();

    let list = List::new(items);
    frame.render_widget(list, inner);
}

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
