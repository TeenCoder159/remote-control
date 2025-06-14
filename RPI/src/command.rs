use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    error::Error,
    process::{Child, Command},
};

#[derive(Deserialize, Serialize, Debug)]
pub enum Status {
    On,
    Off,
}

#[derive(Deserialize, Serialize)]
struct Program {
    name: String,
    location: String,
    start_command: String,
    stop_command: String,
    status: Status,
    env_var: Option<HashMap<String, String>>,
}

impl Program {
    // pub fn get_programs() -> Vec<Program> {}
    pub fn init_env(&self) -> Option<HashMap<String, String>> {
        if self.env_var == None {
            return None;
        }
        let mut env_variables = HashMap::new();

        for (key, value) in self.env_var.clone().unwrap() {
            env_variables.insert(key, value);
        }
        Some(env_variables)
    }
    pub fn start(&self) -> std::io::Result<Child> {
        let env_vars = self.init_env().unwrap_or(HashMap::new());
        Command::new(self.start_command.clone())
            .envs(env_vars)
            .current_dir(self.location.clone())
            .spawn()
    }

    pub fn stop(&self, child: &mut Child) -> Result<(), Box<dyn Error>> {
        child.kill()?;
        Ok(())
    }
}

// impl Status {
//     pub fn bool_to_status(status: bool) -> Self {
//         match status {
//             true => Status::On,
//             false => Status::Off,
//         }
//     }

//     pub fn status_to_bool(&self) -> bool {
//         match self {
//             Self::On => true,
//             Self::Off => false,
//         }
//     }

//     pub fn stringify(&self) -> &'static str {
//         match self {
//             Self::On => "On",
//             Self::Off => "Off",
//         }
//     }
// }
