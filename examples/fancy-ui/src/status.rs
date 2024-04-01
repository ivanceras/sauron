use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
pub enum Status {
    Error,
    Success,
    Info,
    Warning,
}

impl Status {
    pub fn class_name(&self) -> &'static str {
        match self {
            Status::Error => "error",
            Status::Success => "success",
            Status::Info => "info",
            Status::Warning => "warning",
        }
    }
}

impl FromStr for Status {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "error" => Ok(Status::Error),
            "success" => Ok(Status::Success),
            "info" => Ok(Status::Info),
            "warning" => Ok(Status::Warning),
            _ => Err(()),
        }
    }
}
