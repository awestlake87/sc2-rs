use colored::{ColoredString, Colorize};

pub fn info_tag() -> ColoredString {
    "Info".white().bold()
}

pub fn warning_tag() -> ColoredString {
    "Warning ".yellow().bold()
}

pub fn sc2_bug_tag() -> ColoredString {
    "sc2-rs BUG ".red().bold()
}
