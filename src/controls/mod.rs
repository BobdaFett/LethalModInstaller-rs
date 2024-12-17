pub mod install_control;
pub mod repair_control;
pub mod config_control;
pub mod uninstall_control;

pub use self::{
    install_control::*,
    repair_control::*,
    config_control::*,
    uninstall_control::*,
};
