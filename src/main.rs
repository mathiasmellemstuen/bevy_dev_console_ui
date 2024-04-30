use bevy::prelude::*;

use bevy_dev_console_ui::prelude::*;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    app.add_plugins(ConsoleUIPlugin);

    register_command_system!("test", app, Update, test_command);

    app.run();

}
fn test_command() {
    info!("Test command running!");
}