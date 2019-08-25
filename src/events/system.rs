/// Events related to general System things.
///
use super::AnyEvent;
use crate::core::Prop;

/// Unused
///
#[derive(Debug, Clone, Copy)]
pub enum LogEvent {
    Message(&'static str),
}
impl AnyEvent for LogEvent {}


/// Unused
///
#[derive(Debug, Clone, Copy)]
pub enum DebugEvent {
    Tween(u32, Prop),
}

impl AnyEvent for DebugEvent {}