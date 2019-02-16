#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Visibility {
    Visible,
    Invisible,
    Gone,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WindowStartSize {
    Exact(u16, u16),
    Fullscreen,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Menu {
    None,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MessageSeverity {
    Info,
    Warning,
    Alert,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TextContent {
    Plain(String),
    LabelDescription(String, String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImageScalePolicy {
    CropCenter, // TODO variants
    FitCenter,  // TODO variants
                // TODO Tile
}
