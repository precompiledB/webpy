use serde::Deserialize ;
use toml;

#[derive(Deserialize, Debug)]
pub struct Assignment{
    description: String,
    #[serde(rename = "Task")]
    tasks: Vec<Task>,
    status: Status,
}

impl Assignment {
    pub fn from_toml(toml_data: &str) -> Result<Assignment, toml::de::Error> {
        toml::from_str(toml_data)
    }
}

#[derive(Deserialize, Debug)]
pub struct Task{
    description: String,
    info: String,
    help: String,
    template: String,
    status: Status,
}

#[derive(Deserialize, Debug)]
pub enum Status {
    Complete,
    Current,
    Locked,
}
