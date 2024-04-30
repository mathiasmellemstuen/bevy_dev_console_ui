use bevy::prelude::*;
use std::str::FromStr;

#[derive(Clone)]
pub struct CommandInput {
    pub command: String,
    pub arguments : Vec<String>
}

pub struct CommandInputParseError;

impl FromStr for CommandInput {
    type Err = CommandInputParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts: Vec<String> = vec![];

        for subs in s.split_whitespace() {
            parts.push(subs.to_string());
        }

        match parts.is_empty() {
            true =>  Err(CommandInputParseError),
            false => {
                let cmd = parts[0].clone();

                // This will always be true since we are matching for the emptiness
                parts.remove(0);

                Ok(Self {
                    command:cmd,
                    arguments:parts
                })
            }
        }
    }
}
#[derive(Resource)]
pub struct NextCommandInput(pub Option<CommandInput>);

impl Default for NextCommandInput {
    fn default() -> Self {
        NextCommandInput(None)
    }
}

#[derive(Resource)]
pub struct CommandRegistry(pub Vec<String>);

impl Default for CommandRegistry {
    fn default() -> Self {
        CommandRegistry(vec![])
    }
}

pub fn purge_command(mut next_command_input: ResMut<NextCommandInput>) {
    next_command_input.0 = None;
}

#[macro_export]
macro_rules! register_command_system {
    ($command:tt, $app:ident, $schedule:ident, $system:ident) => {
        {
            fn cond(next_command_input: Res<NextCommandInput>) -> bool {

                match &next_command_input.0 {
                    Some(ci) => {
                        $command == ci.command
                    },
                    None => false
                }
            }

            $app.add_systems($schedule, ($system, purge_command).run_if(cond));


            // We store the registered command in the registry
            fn add_to_command_registry(mut command_registry : ResMut<CommandRegistry>) {
                command_registry.0.push($command.to_string());
            }

            $app.add_systems(Startup, add_to_command_registry);
        }
    };
}