#[derive(Debug)]
pub enum IconType {
    Error,
    Info,
    None,
}

impl std::fmt::Display for IconType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}

#[derive(Debug)]
pub struct MsgBoxCreationError<'a> {
    pub title: &'a str,
    pub content: &'a str,
    pub icon_type: IconType,
}
