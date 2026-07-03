use std::sync::OnceLock;

// 생성 패턴
// ////////////////////////////////////////////////////////
// Factory Method
trait Button {
    fn render(&self) -> String;
}

struct WindowsButton;
impl Button for WindowsButton {
    fn render(&self) -> String {
        "[Windows Button]".to_string()
    }
}

struct HtmlButton;
impl Button for HtmlButton {
    fn render(&self) -> String {
        "<button>HTML Button</button>".to_string()
    }
}

trait Dialog {
    fn create_button(&self) -> Box<dyn Button>;

    fn render_window(&self) -> String {
        let button = self.create_button();
        format!("Dialog rendering with: {}", button.render())
    }
}

struct WindowsDialog;
impl Dialog for WindowsDialog {
    fn create_button(&self) -> Box<dyn Button> {
        Box::new(WindowsButton)
    }
}

struct HtmlDialog;
impl Dialog for HtmlDialog {
    fn create_button(&self) -> Box<dyn Button> {
        Box::new(HtmlButton)
    }
}

// ///////////////////////////////////////////////////////////////////
// Abstract Factory

trait ButtonP {
    fn paint(&self) -> String;
}

trait Checkbox {
    fn paint(&self) -> String;
}

struct MacButton;
impl ButtonP for MacButton {
    fn paint(&self) -> String {
        "Mac Button".to_string()
    }
}

struct MacCheckbox;
impl Checkbox for MacCheckbox {
    fn paint(&self) -> String {
        "Mac Checkbox".to_string()
    }
}

struct WinButton;
impl ButtonP for WinButton {
    fn paint(&self) -> String {
        "Win Button".to_string()
    }
}

struct WinCheckbox;
impl Checkbox for WinCheckbox {
    fn paint(&self) -> String {
        "Win Checkbox".to_string()
    }
}

// 2. 추상 팩토리(Abstract Factory) 인터페이스
trait GuiFactory {
    fn create_button(&self) -> Box<dyn ButtonP>;
    fn create_checkbox(&self) -> Box<dyn Checkbox>;
}

// 3. 구체적인 팩토리(Concrete Factory) 구현
struct WinFactory;
impl GuiFactory for WinFactory {
    fn create_button(&self) -> Box<dyn ButtonP> {
        Box::new(WinButton)
    }

    fn create_checkbox(&self) -> Box<dyn Checkbox> {
        Box::new(WinCheckbox)
    }
    // 이 부분을 구현하세요.
}

struct MacFactory;
impl GuiFactory for MacFactory {
    fn create_button(&self) -> Box<dyn ButtonP> {
        Box::new(MacButton)
    }

    fn create_checkbox(&self) -> Box<dyn Checkbox> {
        Box::new(MacCheckbox)
    }
    // 이 부분을 구현하세요.
}

// ////////////////////////////////////////////////////////
// Builder
#[derive(Debug, PartialEq)]
struct ServerConfig {
    host: String,
    port: u16,
    timeout: u32,
    max_connections: usize,
}

struct ServerConfigBuilder {
    host: Option<String>,
    port: Option<u16>,
    timeout: Option<u32>,
    max_connections: Option<usize>,
}
impl ServerConfigBuilder {
    fn new() -> Self {
        ServerConfigBuilder {
            host: None,
            port: None,
            timeout: None,
            max_connections: None,
        }
    }

    fn host(mut self, host: &str) -> Self {
        self.host = Some(host.to_string());
        self
    }

    fn port(mut self, port: u16) -> Self {
        self.port = Some(port);
        self
    }

    fn timeout(mut self, timeout: u32) -> Self {
        self.timeout = Some(timeout);
        self
    }

    fn max_connections(mut self, max_connections: usize) -> Self {
        self.max_connections = Some(max_connections);
        self
    }

    fn build(self) -> ServerConfig {
        ServerConfig {
            host: self.host.unwrap_or_else(|| "127.0.0.1".to_string()),
            port: self.port.unwrap_or(8080),
            timeout: self.timeout.unwrap_or(30),
            max_connections: self.max_connections.unwrap_or(1_000),
        }
    }
}

// ////////////////////////////////////////////////////////
// Prototype

#[derive(Debug, PartialEq)]
struct Cell {
    name: String,
    dna_sequence: Vec<u8>,
}
impl Cell {
    fn new(name: &str, dna: Vec<u8>) -> Self {
        Cell {
            name: name.to_string(),
            dna_sequence: dna,
        }
    }
}

impl Clone for Cell {
    fn clone(&self) -> Self {
        Self {
            name: format!("{}_clone", self.name),
            dna_sequence: self.dna_sequence.clone(),
        }
    }
}

// ////////////////////////////////////////////////////////
// Singleton

#[derive(Debug, PartialEq)]
struct DatabaseConnection {
    connection_string: String,
}
impl DatabaseConnection {
    fn get_instance() -> &'static Self {
        static INSTANCE: OnceLock<DatabaseConnection> = OnceLock::new();
        INSTANCE.get_or_init(|| DatabaseConnection {
            connection_string: "mysql://localhost:3306/prod".to_string(),
        })
    }

    fn query(&self) -> String {
        format!("Executing query on {}", self.connection_string)
    }
}

#[cfg(test)]
#[path = "creational_test.rs"]
mod creational_test;
