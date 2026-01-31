use chrono::{DateTime, Local};
use tokio::sync::mpsc;
use uuid::Uuid;
use sqlx::PgPool;
use std::sync::Arc;

use crate::api::deepseek::{ChatMessage, DeepSeekClient};
use crate::config::Config;
use crate::auth::service::AuthService;
use crate::db::{CreateUserRequest, LoginRequest};

#[derive(Debug, Clone)]
pub struct Message {
    pub id: Uuid,
    pub role: MessageRole,
    pub content: String,
    pub timestamp: DateTime<Local>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum MessageRole {
    User,
    Assistant,
    System,
    Error,
}

impl Message {
    pub fn user(content: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            role: MessageRole::User,
            content,
            timestamp: Local::now(),
        }
    }

    pub fn assistant(content: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            role: MessageRole::Assistant,
            content,
            timestamp: Local::now(),
        }
    }

    pub fn system(content: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            role: MessageRole::System,
            content,
            timestamp: Local::now(),
        }
    }

    pub fn error(content: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            role: MessageRole::Error,
            content,
            timestamp: Local::now(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum InputMode {
    Normal,
    Editing,
}

#[derive(Debug, Clone)]
pub enum SlashCommand {
    Login { email: String, password: String },
    Register { email: String, username: String, password: String },
    Logout,
    Upgrade,
    Help,
    Quit,
    Clear,
    Status,
    Unknown(String),
}

impl SlashCommand {
    pub fn parse(input: &str) -> Option<Self> {
        let input = input.trim();
        if !input.starts_with('/') {
            return None;
        }

        let parts: Vec<&str> = input[1..].split_whitespace().collect();
        if parts.is_empty() {
            return None;
        }

        let cmd = parts[0].to_lowercase();
        Some(match cmd.as_str() {
            "login" => {
                if parts.len() >= 3 {
                    SlashCommand::Login {
                        email: parts[1].to_string(),
                        password: parts[2].to_string(),
                    }
                } else {
                    SlashCommand::Unknown("login <email> <password>".to_string())
                }
            }
            "register" => {
                if parts.len() >= 4 {
                    SlashCommand::Register {
                        email: parts[1].to_string(),
                        username: parts[2].to_string(),
                        password: parts[3].to_string(),
                    }
                } else {
                    SlashCommand::Unknown("register <email> <username> <password>".to_string())
                }
            }
            "logout" => SlashCommand::Logout,
            "upgrade" => SlashCommand::Upgrade,
            "help" | "h" | "?" => SlashCommand::Help,
            "quit" | "q" | "exit" => SlashCommand::Quit,
            "clear" | "cls" => SlashCommand::Clear,
            "status" => SlashCommand::Status,
            other => SlashCommand::Unknown(other.to_string()),
        })
    }
}

pub struct App {
    pub messages: Vec<Message>,
    pub input: String,
    pub input_mode: InputMode,
    pub scroll_offset: usize,
    pub user_email: Option<String>,
    pub user_tier: String,
    pub is_connected: bool,
    pub should_quit: bool,
    pub is_loading: bool,
    pub ai_client: DeepSeekClient,
    pub ai_response_rx: Option<mpsc::Receiver<Result<String, String>>>,
    pub auth_response_rx: Option<mpsc::Receiver<Result<(String, String, String), String>>>,
    pub conversation_history: Vec<ChatMessage>,
    pub show_exit_animation: bool,
    pub exit_animation_frame: usize,
    pub config: Config,
    pub auth_service: Option<Arc<AuthService>>,
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

impl App {
    pub fn new() -> Self {
        // Load or create configuration
        let config = Config::load().unwrap_or_else(|e| {
            eprintln!("Warning: Failed to load config: {}. Using defaults.", e);
            Config::default()
        });
        
        // Initialize auth service if DATABASE_URL is available
        let auth_service = std::env::var("DATABASE_URL")
            .ok()
            .and_then(|url| {
                tokio::task::block_in_place(|| {
                    tokio::runtime::Handle::current().block_on(async {
                        PgPool::connect(&url).await.ok()
                    })
                })
            })
            .and_then(|pool| AuthService::new(pool).ok())
            .map(Arc::new);
        
        if auth_service.is_none() {
            eprintln!("⚠️  Warning: Database not available. Auth features disabled.");
        }
        
        // Initialize AI client with config
        let ai_client = if let Some(api_key) = config.get_ai_api_key() {
            DeepSeekClient::new(api_key)
        } else {
            DeepSeekClient::with_default_key()
        };
        
        // Extract user info from config
        let (user_email, user_tier) = if let Some(ref user) = config.user {
            (Some(user.email.clone()), user.tier.clone())
        } else {
            (None, "free".to_string())
        };
        
        let mut app = Self {
            messages: Vec::new(),
            input: String::new(),
            input_mode: InputMode::Normal,
            scroll_offset: 0,
            user_email,
            user_tier,
            is_connected: true,
            should_quit: false,
            is_loading: false,
            ai_client,
            ai_response_rx: None,
            auth_response_rx: None,
            conversation_history: vec![DeepSeekClient::get_system_prompt()],
            show_exit_animation: false,
            exit_animation_frame: 0,
            config,
            auth_service,
        };

        // Check if first run
        let is_first_run = !Config::exists();
        
        // Welcome message
        let welcome_msg = if is_first_run {
            format!(
                r#"
╔═══════════════════════════════════════════════════════════════════╗
║                                                                   ║
║   ██████╗ ██╗  ██╗██╗   ██╗██████╗                               ║
║  ██╔═══██╗██║  ██║██║   ██║██╔══██╗                              ║
║  ██║   ██║███████║██║   ██║██████╔╝                              ║
║  ██║▄▄ ██║██╔══██║██║   ██║██╔══██╗                              ║
║  ╚██████╔╝██║  ██║╚██████╔╝██████╔╝                              ║
║   ╚══▀▀═╝ ╚═╝  ╚═╝ ╚═════╝ ╚═════╝                               ║
║                                                                   ║
║   Quantum Computing + AI                                          ║
║                                                                   ║
╚═══════════════════════════════════════════════════════════════════╝

Welcome to QHub! 🎉 First time setup detected.

Configuration saved to: {}

To get started:
  • Set your AI API key:   export CLOUDFLARE_AI_TOKEN=your_key
  • Use /help to see all commands
  • Start chatting to generate quantum circuits!

Example: "create a bell state circuit"
"#,
                Config::config_path().map(|p| p.display().to_string()).unwrap_or_else(|_| "~/.qhub/config.toml".to_string())
            )
        } else {
            r#"
╔═══════════════════════════════════════════════════════════════════╗
║                                                                   ║
║   ██████╗ ██╗  ██╗██╗   ██╗██████╗                               ║
║  ██╔═══██╗██║  ██║██║   ██║██╔══██╗                              ║
║  ██║   ██║███████║██║   ██║██████╔╝                              ║
║  ██║▄▄ ██║██╔══██║██║   ██║██╔══██╗                              ║
║  ╚██████╔╝██║  ██║╚██████╔╝██████╔╝                              ║
║   ╚══▀▀═╝ ╚═╝  ╚═╝ ╚═════╝ ╚═════╝                               ║
║                                                                   ║
║   Quantum Computing + AI                                          ║
║                                                                   ║
╚═══════════════════════════════════════════════════════════════════╝

Welcome back to QHub!

Commands:
  /login     - Log in to your account
  /register  - Create a new account
  /upgrade   - Upgrade your plan
  /status    - Show configuration status
  /help      - Show help
  /quit      - Exit QHub

Describe what quantum computation you'd like to perform, and I'll generate the code for you.
"#.to_string()
        };
        
        app.messages.push(Message::system(welcome_msg));

        app
    }

    pub fn submit_input(&mut self) {
        let input = self.input.trim().to_string();
        if input.is_empty() || self.is_loading {
            return;
        }

        // Check for slash commands
        if let Some(cmd) = SlashCommand::parse(&input) {
            self.handle_slash_command(cmd);
        } else {
            // Regular message to AI
            self.messages.push(Message::user(input.clone()));
            
            // Add to conversation history
            self.conversation_history.push(ChatMessage {
                role: "user".to_string(),
                content: input.clone(),
            });
            
            // Keep conversation history manageable (last 20 messages + system prompt)
            // This prevents token overflow and keeps context relevant
            if self.conversation_history.len() > 21 {
                // Keep system prompt (first message) and last 20 messages
                let system_prompt = self.conversation_history[0].clone();
                let recent_messages: Vec<_> = self.conversation_history
                    .iter()
                    .skip(self.conversation_history.len() - 20)
                    .cloned()
                    .collect();
                
                self.conversation_history = vec![system_prompt];
                self.conversation_history.extend(recent_messages);
            }
            
            // Start async AI request
            self.is_loading = true;
            let (tx, rx) = mpsc::channel(1);
            self.ai_response_rx = Some(rx);
            
            let client = self.ai_client.clone();
            let history = self.conversation_history.clone();
            
            tokio::spawn(async move {
                let result = client.chat(history).await;
                let _ = tx.send(result.map_err(|e| e.to_string())).await;
            });
        }

        self.input.clear();
        self.scroll_to_bottom();
    }
    
    pub fn check_ai_response(&mut self) {
        if let Some(ref mut rx) = self.ai_response_rx {
            match rx.try_recv() {
                Ok(Ok(response)) => {
                    self.conversation_history.push(ChatMessage {
                        role: "assistant".to_string(),
                        content: response.clone(),
                    });
                    self.messages.push(Message::assistant(response));
                    self.is_loading = false;
                    self.ai_response_rx = None;
                    self.scroll_to_bottom();
                }
                Ok(Err(error)) => {
                    // User-friendly error messages
                    let friendly_error = if error.contains("timeout") {
                        "Request timed out. The AI service might be busy. Please try again.".to_string()
                    } else if error.contains("429") {
                        "Rate limit reached. Please wait a moment before trying again.".to_string()
                    } else if error.contains("401") || error.contains("403") {
                        "Authentication failed. Please check your API key in CLOUDFLARE_AI_TOKEN environment variable.".to_string()
                    } else if error.contains("network") || error.contains("connection") {
                        "Network error. Please check your internet connection.".to_string()
                    } else {
                        format!("AI service error: {}", error)
                    };
                    
                    self.messages.push(Message::error(friendly_error));
                    self.is_loading = false;
                    self.ai_response_rx = None;
                    self.scroll_to_bottom();
                }
                Err(mpsc::error::TryRecvError::Empty) => {
                    // Still waiting
                }
                Err(mpsc::error::TryRecvError::Disconnected) => {
                    self.messages.push(Message::error(
                        "AI request failed unexpectedly. Please try again.".to_string()
                    ));
                    self.is_loading = false;
                    self.ai_response_rx = None;
                }
            }
        }
    }

    pub fn check_auth_response(&mut self) {
        if let Some(ref mut rx) = self.auth_response_rx {
            match rx.try_recv() {
                Ok(Ok((token, email, tier))) => {
                    // Save to config
                    self.config.user = Some(crate::config::settings::UserConfig {
                        email: email.clone(),
                        token: Some(token),
                        tier: tier.clone(),
                    });
                    
                    if let Err(e) = self.config.save() {
                        self.messages.push(Message::error(
                            format!("Failed to save config: {}", e)
                        ));
                    } else {
                        self.user_email = Some(email.clone());
                        self.user_tier = tier.clone();
                        self.messages.push(Message::system(
                            format!("✓ Logged in successfully as {} ({})", email, tier)
                        ));
                    }
                    
                    self.is_loading = false;
                    self.auth_response_rx = None;
                    self.scroll_to_bottom();
                }
                Ok(Err(error)) => {
                    let friendly_error = if error.contains("already registered") {
                        "Email is already registered. Try logging in instead.".to_string()
                    } else if error.contains("Invalid email or password") {
                        "Invalid email or password. Please try again.".to_string()
                    } else if error.contains("Invalid email format") {
                        "Invalid email format. Please use a valid email address.".to_string()
                    } else if error.contains("deactivated") {
                        "Account is deactivated. Contact support for assistance.".to_string()
                    } else {
                        format!("Authentication error: {}", error)
                    };
                    
                    self.messages.push(Message::error(friendly_error));
                    self.is_loading = false;
                    self.auth_response_rx = None;
                    self.scroll_to_bottom();
                }
                Err(mpsc::error::TryRecvError::Empty) => {
                    // Still waiting
                }
                Err(mpsc::error::TryRecvError::Disconnected) => {
                    self.messages.push(Message::error(
                        "Authentication request failed. Please try again.".to_string()
                    ));
                    self.is_loading = false;
                    self.auth_response_rx = None;
                }
            }
        }
    }

    fn handle_slash_command(&mut self, cmd: SlashCommand) {
        match cmd {
            SlashCommand::Login { email, password } => {
                if self.auth_service.is_none() {
                    self.messages.push(Message::error(
                        "Authentication service unavailable. Check DATABASE_URL.".to_string()
                    ));
                    self.input.clear();
                    return;
                }
                
                self.messages.push(Message::system("🔄 Logging in...".to_string()));
                self.is_loading = true;
                
                let auth_service = Arc::clone(self.auth_service.as_ref().unwrap());
                let (tx, rx) = mpsc::channel(1);
                self.auth_response_rx = Some(rx);
                
                tokio::spawn(async move {
                    let result = auth_service.login(LoginRequest {
                        email: email.clone(),
                        password,
                    }).await;
                    
                    let response = match result {
                        Ok(auth_resp) => Ok((auth_resp.token, auth_resp.user.email, auth_resp.user.tier)),
                        Err(e) => Err(e.to_string()),
                    };
                    let _ = tx.send(response).await;
                });
            }
            SlashCommand::Register { email, username, password } => {
                if self.auth_service.is_none() {
                    self.messages.push(Message::error(
                        "Authentication service unavailable. Check DATABASE_URL.".to_string()
                    ));
                    self.input.clear();
                    return;
                }
                
                self.messages.push(Message::system("🔄 Creating account...".to_string()));
                self.is_loading = true;
                
                let auth_service = Arc::clone(self.auth_service.as_ref().unwrap());
                let (tx, rx) = mpsc::channel(1);
                self.auth_response_rx = Some(rx);
                
                tokio::spawn(async move {
                    let result = auth_service.register(CreateUserRequest {
                        email: email.clone(),
                        username: Some(username),
                        password,
                    }).await;
                    
                    let response = match result {
                        Ok(auth_resp) => Ok((auth_resp.token, auth_resp.user.email, auth_resp.user.tier)),
                        Err(e) => Err(e.to_string()),
                    };
                    let _ = tx.send(response).await;
                });
            }
            SlashCommand::Logout => {
                if let Some(ref mut user_config) = self.config.user {
                    if let Some(token) = user_config.token.take() {
                        if let Some(service) = &self.auth_service {
                            let service = service.clone();
                            tokio::spawn(async move {
                                let _ = service.logout(&token).await;
                            });
                        }
                    }
                }
                
                self.config.user = None;
                self.user_email = None;
                self.user_tier = "free".to_string();
                
                if let Err(e) = self.config.save() {
                    self.messages.push(Message::error(
                        format!("Failed to save config: {}", e)
                    ));
                } else {
                    self.messages.push(Message::system("✓ Logged out successfully".to_string()));
                }
            }
            SlashCommand::Upgrade => {
                self.messages.push(Message::system(
                    "Opening upgrade page in your browser...".to_string()
                ));
                // TODO: Open browser for upgrade
            }
            SlashCommand::Help => {
                self.messages.push(Message::system(
                    r#"
╭──────────────────────────────────────────────────────────────────╮
│                         QHub Commands                            │
├──────────────────────────────────────────────────────────────────┤
│  /login <email> <password>                                       │
│      Log in to your QHub account                                 │
│  /register <email> <username> <password>                         │
│      Create a new account                                        │
│  /logout                                                         │
│      Log out from your account                                   │
│  /upgrade    Upgrade to Pro for more quantum backends            │
│  /status     Show your current account status                    │
│  /clear      Clear the chat history                              │
│  /help       Show this help message                              │
│  /quit       Exit QHub                                           │
├──────────────────────────────────────────────────────────────────┤
│  Keyboard Shortcuts:                                             │
│  Ctrl+C      Exit QHub                                           │
│  Ctrl+Q      Exit QHub                                           │
│  PageUp/Down Scroll through messages                             │
│  Enter       Send message                                        │
╰──────────────────────────────────────────────────────────────────╯
"#.to_string()
                ));
            }
            SlashCommand::Quit => {
                self.show_exit_animation = true;
                self.exit_animation_frame = 0;
            }
            SlashCommand::Clear => {
                self.messages.clear();
                self.messages.push(Message::system("Chat cleared.".to_string()));
            }
            SlashCommand::Status => {
                let config_path = Config::config_path()
                    .map(|p| p.display().to_string())
                    .unwrap_or_else(|_| "unknown".to_string());
                
                let ai_key_status = if self.config.get_ai_api_key().is_some() {
                    "✓ Configured"
                } else {
                    "✗ Not set"
                };
                
                let quantum_key_status = if self.config.get_quantum_api_key().is_some() {
                    "✓ Configured"
                } else {
                    "✗ Not set"
                };
                
                let db_status = if self.auth_service.is_some() {
                    "✓ Connected"
                } else {
                    "✗ Not available"
                };
                
                let status = if let Some(email) = &self.user_email {
                    format!(
                        r#"
╭─────────────────────────────────────────────╮
│ Account Status                              │
├─────────────────────────────────────────────┤
│ Email: {}
│ Tier:  {}
│ Status: {}
├─────────────────────────────────────────────┤
│ Configuration                               │
├─────────────────────────────────────────────┤
│ Config file: {}
│ Database: {}
│ AI Provider: {} ({})
│ Quantum Provider: {} ({})
│ AI Model: {}
╰─────────────────────────────────────────────╯
"#,
                        email,
                        self.user_tier,
                        if self.is_connected { "Connected" } else { "Disconnected" },
                        config_path,
                        db_status,
                        self.config.ai.provider,
                        ai_key_status,
                        self.config.quantum.provider,
                        quantum_key_status,
                        self.config.ai.model,
                    )
                } else {
                    format!(
                        r#"
╭─────────────────────────────────────────────╮
│ Account Status                              │
├─────────────────────────────────────────────┤
│ Not logged in
│ Use /login or /register to get started
├─────────────────────────────────────────────┤
│ Configuration                               │
├─────────────────────────────────────────────┤
│ Config file: {}
│ Database: {}
│ AI Provider: {} ({})
│ Quantum Provider: {} ({})
│ AI Model: {}
╰─────────────────────────────────────────────╯
"#,
                        config_path,
                        db_status,
                        self.config.ai.provider,
                        ai_key_status,
                        self.config.quantum.provider,
                        quantum_key_status,
                        self.config.ai.model,
                    )
                };
                self.messages.push(Message::system(status));
            }
            SlashCommand::Unknown(cmd) => {
                self.messages.push(Message::error(
                    format!("Unknown command or invalid syntax: /{}. Type /help for available commands.", cmd)
                ));
            }
        }
        self.input.clear();
        self.scroll_to_bottom();
    }

    pub fn scroll_up(&mut self) {
        if self.scroll_offset > 0 {
            self.scroll_offset -= 1;
        }
    }

    pub fn scroll_down(&mut self) {
        self.scroll_offset += 1;
    }

    pub fn scroll_to_bottom(&mut self) {
        // Will be calculated properly in UI rendering
        self.scroll_offset = usize::MAX;
    }
}
