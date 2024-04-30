
mod console_ui;
mod console_command_registry;
pub mod prelude {

    use crate::console_ui;
    use crate::console_command_registry;

    pub use console_ui::ConsoleUIPlugin;

    pub use console_command_registry::CommandRegistry;
    pub use console_command_registry::CommandInput;
    pub use console_command_registry::CommandInputParseError;
    pub use console_command_registry::NextCommandInput;
    pub use console_command_registry::purge_command;

    pub use crate::register_command_system;
}
