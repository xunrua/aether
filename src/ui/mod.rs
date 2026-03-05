//! UI 模块
//!
//! 提供现代毛玻璃悬浮风格的消息模板

mod templates;

pub use templates::{
    GlassTemplate,
    info_card, help_menu, leaderboard, success, error, warning, info, subcommand_list,
};