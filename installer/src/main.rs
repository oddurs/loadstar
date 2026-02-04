/*!
# LOAD"*",8,1

A Commodore-flavored machine setup wizard.
Your terminal will never be the same.
*/

use anyhow::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::{
    io::{self, Stdout},
    time::{Duration, Instant},
};

mod ascii_art;
mod catalog;
mod config;
mod effects;
mod executor;
mod github;
mod render;
mod system;
mod wizard;

use effects::{MatrixRain, Spinner, TypeWriter};
use executor::InstallMessage;
use render::render_app;
use system::SystemInfo;
use wizard::{WizardPhase, WizardState};

use std::sync::mpsc;

/// Main application state
pub struct App {
    pub wizard: WizardState,
    pub system: SystemInfo,
    pub matrix_rain: MatrixRain,
    pub spinner: Spinner,
    pub typewriter: Option<TypeWriter>,
    pub boot_sequence: BootSequence,
    pub last_tick: Instant,
    pub tick_rate: Duration,
    pub should_quit: bool,
    pub install_log: Vec<String>,
    pub install_progress: f64,
    pub install_total: usize,
    pub install_completed: usize,
    pub install_succeeded: usize,
    pub install_failed: usize,
    pub install_skipped: usize,
    pub current_package: Option<String>,
    pub is_installing: bool,
    pub install_receiver: Option<mpsc::Receiver<InstallMessage>>,
    pub install_thread: Option<std::thread::JoinHandle<()>>,
    pub error_message: Option<String>,
}

/// Boot sequence state
pub struct BootSequence {
    pub stage: usize,
    pub messages: Vec<(&'static str, bool)>, // (message, complete)
    pub current_typewriter: Option<TypeWriter>,
    pub complete: bool,
}

impl Default for BootSequence {
    fn default() -> Self {
        Self::new()
    }
}

impl BootSequence {
    pub fn new() -> Self {
        let messages = vec![
            ("BIOS CHECK... ", false),
            ("NEURAL INTERFACE ONLINE... ", false),
            ("SCANNING REALITY MATRIX... ", false),
            ("QUANTUM ENTANGLEMENT STABLE... ", false),
            ("CONSCIOUSNESS UPLOAD READY... ", false),
            ("READY.", false),
        ];

        let current_typewriter = Some(TypeWriter::new(messages[0].0));

        Self {
            stage: 0,
            messages,
            current_typewriter,
            complete: false,
        }
    }

    pub fn tick(&mut self) {
        if self.complete {
            return;
        }

        if let Some(tw) = &mut self.current_typewriter {
            tw.tick();

            if tw.complete {
                self.messages[self.stage].1 = true;
                self.stage += 1;

                if self.stage < self.messages.len() {
                    self.current_typewriter = Some(TypeWriter::new(self.messages[self.stage].0));
                } else {
                    self.current_typewriter = None;
                    self.complete = true;
                }
            }
        }
    }

    pub fn skip(&mut self) {
        // Complete all messages
        for msg in &mut self.messages {
            msg.1 = true;
        }
        self.stage = self.messages.len();
        self.current_typewriter = None;
        self.complete = true;
    }
}

impl App {
    pub fn new() -> Result<Self> {
        let system = SystemInfo::detect()?;

        Ok(Self {
            wizard: WizardState::new(),
            system,
            matrix_rain: MatrixRain::new(120, 40),
            spinner: Spinner::braille(),
            typewriter: None,
            boot_sequence: BootSequence::new(),
            last_tick: Instant::now(),
            tick_rate: Duration::from_millis(50),
            should_quit: false,
            install_log: Vec::new(),
            install_progress: 0.0,
            install_total: 0,
            install_completed: 0,
            install_succeeded: 0,
            install_failed: 0,
            install_skipped: 0,
            current_package: None,
            is_installing: false,
            install_receiver: None,
            install_thread: None,
            error_message: None,
        })
    }

    pub fn tick(&mut self) {
        self.matrix_rain.tick();
        self.spinner.tick();

        if let Some(tw) = &mut self.typewriter {
            tw.tick();
        }

        if self.wizard.phase == WizardPhase::Boot {
            self.boot_sequence.tick();

            if self.boot_sequence.complete {
                // Auto-advance after boot
                self.wizard.advance();
            }
        }
    }

    pub fn handle_key(&mut self, key: KeyCode, modifiers: KeyModifiers) {
        // Global quit (during install, skip to complete instead of hard exit)
        if modifiers.contains(KeyModifiers::CONTROL) && key == KeyCode::Char('c') {
            if self.is_installing && self.wizard.phase == WizardPhase::Install {
                // Abort install gracefully — drop the receiver so the thread is detached
                self.install_receiver = None;
                self.is_installing = false;
                self.install_log
                    .push("[ABORT] Installation interrupted by user".to_string());
                self.wizard.phase = WizardPhase::Complete;
            } else {
                self.should_quit = true;
            }
            return;
        }

        match self.wizard.phase {
            WizardPhase::Boot => {
                // Any key skips boot sequence
                self.boot_sequence.skip();
            }

            WizardPhase::Identity => {
                self.handle_identity_input(key);
            }

            WizardPhase::Shell => {
                self.handle_shell_input(key);
            }

            WizardPhase::DevTools => {
                self.handle_devtools_input(key);
            }

            WizardPhase::Apps => {
                self.handle_apps_input(key);
            }

            WizardPhase::Review => {
                self.handle_review_input(key);
            }

            WizardPhase::Install => {
                // No input during install
            }

            WizardPhase::Complete => match key {
                KeyCode::Enter | KeyCode::Char('q') => {
                    self.should_quit = true;
                }
                _ => {}
            },
        }
    }

    fn handle_identity_input(&mut self, key: KeyCode) {
        match key {
            KeyCode::Tab | KeyCode::Down => {
                self.wizard.input_field = (self.wizard.input_field + 1) % 4;
            }
            KeyCode::BackTab | KeyCode::Up => {
                self.wizard.input_field = self.wizard.input_field.checked_sub(1).unwrap_or(3);
            }
            KeyCode::Char(c) => match self.wizard.input_field {
                0 => self.wizard.identity.name.push(c),
                1 => self.wizard.identity.email.push(c),
                2 => self.wizard.identity.github_username.push(c),
                _ => {}
            },
            KeyCode::Backspace => match self.wizard.input_field {
                0 => {
                    self.wizard.identity.name.pop();
                }
                1 => {
                    self.wizard.identity.email.pop();
                }
                2 => {
                    self.wizard.identity.github_username.pop();
                }
                _ => {}
            },
            KeyCode::Left | KeyCode::Right => {
                if self.wizard.input_field == 3 {
                    let types = wizard::SetupType::all();
                    let current = types
                        .iter()
                        .position(|t| *t == self.wizard.identity.setup_type)
                        .unwrap_or(0);
                    let new_idx = if key == KeyCode::Right {
                        (current + 1) % types.len()
                    } else {
                        current.checked_sub(1).unwrap_or(types.len() - 1)
                    };
                    self.wizard.identity.setup_type = types[new_idx];
                }
            }
            KeyCode::Enter => {
                if self.wizard.input_field == 3 {
                    self.wizard.advance();
                } else {
                    self.wizard.input_field = (self.wizard.input_field + 1) % 4;
                }
            }
            KeyCode::Esc => {
                self.wizard.go_back();
            }
            _ => {}
        }
    }

    fn handle_shell_input(&mut self, key: KeyCode) {
        let max_items = 4; // shell, prompt, terminal, multiplexer

        match key {
            KeyCode::Up | KeyCode::Char('k') => {
                self.wizard.cursor_position = self.wizard.cursor_position.saturating_sub(1);
            }
            KeyCode::Down | KeyCode::Char('j') => {
                self.wizard.cursor_position = (self.wizard.cursor_position + 1).min(max_items - 1);
            }
            KeyCode::Left | KeyCode::Char('h') => {
                self.cycle_shell_option(false);
            }
            KeyCode::Right | KeyCode::Char('l') => {
                self.cycle_shell_option(true);
            }
            KeyCode::Enter => {
                self.wizard.advance();
            }
            KeyCode::Esc => {
                self.wizard.go_back();
            }
            _ => {}
        }
    }

    fn cycle_shell_option(&mut self, forward: bool) {
        match self.wizard.cursor_position {
            0 => {
                let opts = wizard::ShellChoice::all();
                let current = opts
                    .iter()
                    .position(|o| *o == self.wizard.shell_config.shell)
                    .unwrap_or(0);
                let new_idx = if forward {
                    (current + 1) % opts.len()
                } else {
                    current.checked_sub(1).unwrap_or(opts.len() - 1)
                };
                self.wizard.shell_config.shell = opts[new_idx];
            }
            1 => {
                let opts = wizard::PromptChoice::all();
                let current = opts
                    .iter()
                    .position(|o| *o == self.wizard.shell_config.prompt)
                    .unwrap_or(0);
                let new_idx = if forward {
                    (current + 1) % opts.len()
                } else {
                    current.checked_sub(1).unwrap_or(opts.len() - 1)
                };
                self.wizard.shell_config.prompt = opts[new_idx];
            }
            2 => {
                let opts = wizard::TerminalChoice::all();
                let current = opts
                    .iter()
                    .position(|o| *o == self.wizard.shell_config.terminal)
                    .unwrap_or(0);
                let new_idx = if forward {
                    (current + 1) % opts.len()
                } else {
                    current.checked_sub(1).unwrap_or(opts.len() - 1)
                };
                self.wizard.shell_config.terminal = opts[new_idx];
            }
            3 => {
                let opts = wizard::MultiplexerChoice::all();
                let current_opt = self.wizard.shell_config.multiplexer;
                let current = current_opt
                    .map(|m| opts.iter().position(|o| *o == m).unwrap_or(0))
                    .unwrap_or(opts.len() - 1);
                let new_idx = if forward {
                    (current + 1) % opts.len()
                } else {
                    current.checked_sub(1).unwrap_or(opts.len() - 1)
                };
                self.wizard.shell_config.multiplexer =
                    if opts[new_idx] == wizard::MultiplexerChoice::None {
                        None
                    } else {
                        Some(opts[new_idx])
                    };
            }
            _ => {}
        }
    }

    fn handle_devtools_input(&mut self, key: KeyCode) {
        let categories = &[
            catalog::Category::Language,
            catalog::Category::Editor,
            catalog::Category::Git,
            catalog::Category::Container,
            catalog::Category::Cloud,
        ];

        match key {
            KeyCode::Up | KeyCode::Char('k') => {
                self.wizard.cursor_position = self.wizard.cursor_position.saturating_sub(1);
            }
            KeyCode::Down | KeyCode::Char('j') => {
                let max = self.get_current_list_len();
                self.wizard.cursor_position =
                    (self.wizard.cursor_position + 1).min(max.saturating_sub(1));
            }
            KeyCode::Tab => {
                // Cycle through categories
                self.wizard.scroll_offset = (self.wizard.scroll_offset + 1) % categories.len();
                self.wizard.cursor_position = 0;
            }
            KeyCode::Char(' ') => {
                self.toggle_current_app();
            }
            KeyCode::Enter => {
                self.wizard.advance();
            }
            KeyCode::Esc => {
                self.wizard.go_back();
            }
            KeyCode::Char('a') => {
                // Select all in current category
                self.select_all_in_category();
            }
            KeyCode::Char('n') => {
                // Deselect all in current category
                self.deselect_all_in_category();
            }
            _ => {}
        }
    }

    fn handle_apps_input(&mut self, key: KeyCode) {
        match key {
            KeyCode::Up | KeyCode::Char('k') => {
                self.wizard.cursor_position = self.wizard.cursor_position.saturating_sub(1);
            }
            KeyCode::Down | KeyCode::Char('j') => {
                let max = self.get_current_list_len();
                self.wizard.cursor_position =
                    (self.wizard.cursor_position + 1).min(max.saturating_sub(1));
            }
            KeyCode::Tab => {
                // Cycle through categories
                let categories = catalog::Category::all();
                self.wizard.scroll_offset = (self.wizard.scroll_offset + 1) % categories.len();
                self.wizard.cursor_position = 0;
            }
            KeyCode::BackTab => {
                let categories = catalog::Category::all();
                self.wizard.scroll_offset = self
                    .wizard
                    .scroll_offset
                    .checked_sub(1)
                    .unwrap_or(categories.len() - 1);
                self.wizard.cursor_position = 0;
            }
            KeyCode::Char(' ') => {
                self.toggle_current_app();
            }
            KeyCode::Char('d') => {
                self.wizard.show_details = !self.wizard.show_details;
            }
            KeyCode::Enter => {
                self.wizard.advance();
            }
            KeyCode::Esc => {
                self.wizard.go_back();
            }
            _ => {}
        }
    }

    fn handle_review_input(&mut self, key: KeyCode) {
        match key {
            KeyCode::Enter | KeyCode::Char('y') => {
                self.start_installation();
                self.wizard.advance();
            }
            KeyCode::Esc | KeyCode::Char('n') => {
                self.wizard.go_back();
            }
            KeyCode::Up | KeyCode::Char('k') => {
                self.wizard.scroll_offset = self.wizard.scroll_offset.saturating_sub(1);
            }
            KeyCode::Down | KeyCode::Char('j') => {
                self.wizard.scroll_offset += 1;
            }
            _ => {}
        }
    }

    fn get_current_list_len(&self) -> usize {
        match self.wizard.phase {
            WizardPhase::DevTools => {
                let categories = &[
                    catalog::Category::Language,
                    catalog::Category::Editor,
                    catalog::Category::Git,
                    catalog::Category::Container,
                    catalog::Category::Cloud,
                ];
                let cat = &categories[self.wizard.scroll_offset % categories.len()];
                catalog::apps_by_category(cat).len()
            }
            WizardPhase::Apps => {
                let categories = catalog::Category::all();
                let cat = &categories[self.wizard.scroll_offset % categories.len()];
                catalog::apps_by_category(cat).len()
            }
            _ => 0,
        }
    }

    fn toggle_current_app(&mut self) {
        let app_id = self.get_current_app_id();
        if let Some(id) = app_id {
            self.wizard.toggle_app(&id);
        }
    }

    fn get_current_app_id(&self) -> Option<String> {
        let categories = match self.wizard.phase {
            WizardPhase::DevTools => vec![
                catalog::Category::Language,
                catalog::Category::Editor,
                catalog::Category::Git,
                catalog::Category::Container,
                catalog::Category::Cloud,
            ],
            WizardPhase::Apps => catalog::Category::all().to_vec(),
            _ => return None,
        };

        let cat_idx = self.wizard.scroll_offset % categories.len();
        let cat = &categories[cat_idx];

        let apps = catalog::apps_by_category(cat);
        apps.get(self.wizard.cursor_position)
            .map(|a| a.id.to_string())
    }

    fn select_all_in_category(&mut self) {
        let categories = &[
            catalog::Category::Language,
            catalog::Category::Editor,
            catalog::Category::Git,
            catalog::Category::Container,
            catalog::Category::Cloud,
        ];
        let cat = &categories[self.wizard.scroll_offset % categories.len()];
        for app in catalog::apps_by_category(cat) {
            self.wizard.selected_apps.insert(app.id.to_string());
        }
    }

    fn deselect_all_in_category(&mut self) {
        let categories = &[
            catalog::Category::Language,
            catalog::Category::Editor,
            catalog::Category::Git,
            catalog::Category::Container,
            catalog::Category::Cloud,
        ];
        let cat = &categories[self.wizard.scroll_offset % categories.len()];
        for app in catalog::apps_by_category(cat) {
            self.wizard.selected_apps.remove(app.id);
        }
    }

    fn start_installation(&mut self) {
        self.is_installing = true;
        self.install_progress = 0.0;
        self.install_completed = 0;
        self.install_log.clear();
        self.install_log
            .push("[INIT] Starting installation sequence...".to_string());

        let selected_apps = self.wizard.get_selected_apps();
        self.install_total = selected_apps.len();

        // Clone what the thread needs (system and wizard state are not Send,
        // so we extract the data we need)
        let system = self.system.clone();
        let wizard_clone = self.wizard.clone();
        let apps: Vec<&'static catalog::App> = selected_apps;

        let (tx, rx) = mpsc::channel();
        self.install_receiver = Some(rx);

        let handle = std::thread::spawn(move || {
            // Phase 1: Install packages
            let _summary = executor::run_install(&system, apps, &tx);

            // Phase 2: Git & GitHub setup (SSH keys, git config, gh auth)
            github::setup_github(&wizard_clone, &system, &tx);

            // Phase 3: Generate config files
            config::generate_configs(&wizard_clone, &system, &tx);
        });

        self.install_thread = Some(handle);
    }
}

fn main() -> Result<()> {
    // Initialize terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app
    let mut app = App::new()?;

    // Main loop
    let result = run_app(&mut terminal, &mut app);

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    // Handle result
    if let Err(e) = result {
        eprintln!("Error: {}", e);
        return Err(e);
    }

    // Print completion message
    if app.wizard.phase == WizardPhase::Complete {
        println!(
            "\n{}",
            include_str!("../assets/complete.txt").trim_matches('\n')
        );
    }

    Ok(())
}

fn run_app(terminal: &mut Terminal<CrosstermBackend<Stdout>>, app: &mut App) -> Result<()> {
    loop {
        // Draw
        terminal.draw(|frame| render_app(frame, app))?;

        // Handle input with timeout for animations
        if event::poll(app.tick_rate)? {
            if let Event::Key(key) = event::read()? {
                app.handle_key(key.code, key.modifiers);
            }
        }

        // Update animations
        if app.last_tick.elapsed() >= app.tick_rate {
            app.tick();
            app.last_tick = Instant::now();
        }

        // Process install messages from the background thread
        if app.is_installing && app.wizard.phase == WizardPhase::Install {
            // Drain all pending messages from the install thread
            if let Some(rx) = &app.install_receiver {
                while let Ok(msg) = rx.try_recv() {
                    match msg {
                        InstallMessage::PhaseStart { phase } => {
                            app.install_log.push(format!("[PHASE] ═══ {} ═══", phase));
                        }
                        InstallMessage::PackageStart { name, method } => {
                            app.current_package = Some(name.clone());
                            app.install_log
                                .push(format!("[INSTALL] {} ({})", name, method));
                        }
                        InstallMessage::PackageSuccess { name, duration_ms } => {
                            app.current_package = None;
                            app.install_log.push(format!(
                                "[OK] {} ({:.1}s)",
                                name,
                                duration_ms as f64 / 1000.0
                            ));
                            app.install_completed += 1;
                            app.install_succeeded += 1;
                        }
                        InstallMessage::PackageSkipped { name, reason } => {
                            app.current_package = None;
                            app.install_log
                                .push(format!("[SKIP] {} — {}", name, reason));
                            app.install_completed += 1;
                            app.install_skipped += 1;
                        }
                        InstallMessage::PackageFailed { name, error } => {
                            app.current_package = None;
                            app.install_log.push(format!("[FAIL] {} — {}", name, error));
                            app.install_completed += 1;
                            app.install_failed += 1;
                        }
                        InstallMessage::Log(line) => {
                            app.install_log.push(line);
                        }
                        InstallMessage::Progress { completed, total } => {
                            app.install_completed = completed;
                            app.install_total = total;
                        }
                        InstallMessage::Done {
                            succeeded,
                            failed,
                            skipped,
                        } => {
                            app.install_log.push(format!(
                                "[DONE] {} succeeded, {} failed, {} skipped",
                                succeeded, failed, skipped
                            ));
                        }
                        InstallMessage::FatalError(err) => {
                            app.install_log.push(format!("[FATAL] {}", err));
                            app.error_message = Some(err);
                        }
                    }
                }
            }

            // Update progress percentage
            if app.install_total > 0 {
                app.install_progress =
                    (app.install_completed as f64 / app.install_total as f64) * 100.0;
            }

            // Check if the install thread has finished
            let thread_done = app
                .install_thread
                .as_ref()
                .map(|h| h.is_finished())
                .unwrap_or(false);

            if thread_done {
                // Join the thread
                if let Some(handle) = app.install_thread.take() {
                    let _ = handle.join();
                }
                app.install_receiver = None;
                app.is_installing = false;
                app.install_progress = 100.0;
                app.install_log
                    .push("[COMPLETE] Installation finished!".to_string());
                app.wizard.advance();
            }
        }

        // Check for quit
        if app.should_quit {
            break;
        }
    }

    Ok(())
}
