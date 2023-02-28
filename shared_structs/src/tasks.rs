use serde::Deserialize;
use toml;

#[derive(PartialEq, Eq, Clone, Deserialize, Debug)]
pub struct Assignment {
    pub description: String,
    #[serde(rename = "Task")]
    pub tasks: Vec<Task>,
    pub status: Status,
}

impl Assignment {
    pub fn create_stub() -> Assignment {
        Self {
            description: "This is a stub. If you see this, either the task is \
                still loading or you lost connection with the server".into(),
            status: Status::Current,
            tasks: vec![]
        }
    }

    pub fn from_toml(toml_data: &str) -> Result<Assignment, toml::de::Error> {
        toml::from_str(toml_data)
    }
}

#[derive(Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct Task {
    pub description: String,
    pub info: String,
    pub help: String,
    pub template: String,
    pub status: Status,
}

#[derive(Deserialize, Debug, PartialEq, Eq, Clone)]
pub enum Status {
    Complete,
    Current,
    Locked,
}
