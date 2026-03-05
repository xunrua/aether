//! 毛玻璃风格消息模板
//!
//! 现代毛玻璃悬浮风（Modern Glassmorphism）
//! 特点：半透明背景、背景模糊效果、圆角边框、柔和阴影、图标驱动

/// 毛玻璃风格模板
pub struct GlassTemplate;

impl GlassTemplate {
    /// 基础样式 CSS
    const BASE_STYLE: &'static str = r#"
        background: linear-gradient(135deg, rgba(45, 55, 72, 0.85) 0%, rgba(26, 32, 44, 0.9) 100%);
        border-radius: 12px;
        padding: 16px 20px;
        margin: 8px 0;
        box-shadow: 0 8px 32px rgba(0, 0, 0, 0.3), inset 0 1px 0 rgba(255, 255, 255, 0.1);
        border: 1px solid rgba(255, 255, 255, 0.1);
        backdrop-filter: blur(10px);
        -webkit-backdrop-filter: blur(10px);
        font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, sans-serif;
        color: #e2e8f0;
    "#;

    /// 标题样式
    const TITLE_STYLE: &'static str = r#"
        font-size: 16px;
        font-weight: 600;
        color: #f7fafc;
        margin-bottom: 12px;
        display: flex;
        align-items: center;
        gap: 8px;
    "#;

    /// 内容样式
    const CONTENT_STYLE: &'static str = r#"
        font-size: 14px;
        line-height: 1.6;
        color: #cbd5e0;
    "#;

    /// 分割线样式
    const DIVIDER_STYLE: &'static str = r#"
        height: 1px;
        background: linear-gradient(90deg, transparent, rgba(255, 255, 255, 0.15), transparent);
        margin: 12px 0;
    "#;

    /// 列表项样式
    const ITEM_STYLE: &'static str = r#"
        padding: 8px 12px;
        margin: 4px 0;
        background: rgba(255, 255, 255, 0.05);
        border-radius: 8px;
        border-left: 3px solid rgba(99, 179, 237, 0.6);
    "#;

    /// 图标样式
    const ICON_STYLE: &'static str = r#"
        font-size: 18px;
    "#;

    /// 构建带样式的 div
    fn styled_div(content: &str, style: &str) -> String {
        format!(r#"<div style="{}">{}</div>"#, style.replace('\n', " "), content)
    }

    /// 信息卡片模板
    ///
    /// # Arguments
    /// * `icon` - 图标 emoji
    /// * `title` - 标题
    /// * `items` - 信息项列表 [(label, value), ...]
    pub fn info_card(icon: &str, title: &str, items: &[(impl AsRef<str>, impl AsRef<str>)]) -> String {
        let items_html = items.iter().map(|(label, value)| {
            format!(
                r#"<div style="display: flex; justify-content: space-between; align-items: center; padding: 6px 0;">
                    <span style="color: #a0aec0; font-size: 13px;">{}</span>
                    <span style="color: #e2e8f0; font-weight: 500;">{}</span>
                </div>"#,
                label.as_ref(),
                value.as_ref()
            )
        }).collect::<Vec<_>>().join("\n");

        format!(
            r#"<div style="{}">
                <div style="{}"><span style="{}">{}</span> {}</div>
                <div style="{}"></div>
                <div style="{}">{}</div>
            </div>"#,
            Self::BASE_STYLE.replace('\n', " "),
            Self::TITLE_STYLE.replace('\n', " "),
            Self::ICON_STYLE.replace('\n', " "),
            icon, title,
            Self::DIVIDER_STYLE.replace('\n', " "),
            Self::CONTENT_STYLE.replace('\n', " "),
            items_html
        )
    }

    /// 帮助菜单模板
    ///
    /// # Arguments
    /// * `commands` - 命令列表 [(icon, name, description), ...]
    pub fn help_menu(commands: &[(impl AsRef<str>, impl AsRef<str>, impl AsRef<str>)]) -> String {
        let commands_html = commands.iter().map(|(icon, name, desc)| {
            format!(
                r#"<div style="{}">
                    <span style="{}">{}</span>
                    <code style="background: rgba(99, 179, 237, 0.2); color: #63b3ed; padding: 2px 8px; border-radius: 4px; font-size: 13px; margin-left: 8px;">{}</code>
                    <span style="color: #a0aec0; font-size: 13px; margin-left: 12px;">{}</span>
                </div>"#,
                Self::ITEM_STYLE.replace('\n', " "),
                Self::ICON_STYLE.replace('\n', " "),
                icon.as_ref(),
                name.as_ref(),
                desc.as_ref()
            )
        }).collect::<Vec<_>>().join("\n");

        format!(
            r#"<div style="{}">
                <div style="{}"><span style="{}">📜</span> 命令帮助</div>
                <div style="{}"></div>
                <div style="{}">{}</div>
            </div>"#,
            Self::BASE_STYLE.replace('\n', " "),
            Self::TITLE_STYLE.replace('\n', " "),
            Self::ICON_STYLE.replace('\n', " "),
            Self::DIVIDER_STYLE.replace('\n', " "),
            Self::CONTENT_STYLE.replace('\n', " "),
            commands_html
        )
    }

    /// 排行榜/列表模板
    ///
    /// # Arguments
    /// * `icon` - 图标
    /// * `title` - 标题
    /// * `headers` - 表头
    /// * `rows` - 数据行
    pub fn leaderboard(
        icon: &str,
        title: &str,
        headers: &[impl AsRef<str>],
        rows: &[Vec<impl AsRef<str>>]
    ) -> String {
        let header_html = headers.iter().map(|h| {
            format!(r#"<th style="text-align: left; padding: 8px; color: #a0aec0; font-weight: 500; border-bottom: 1px solid rgba(255, 255, 255, 0.1);">{}</th>"#, h.as_ref())
        }).collect::<Vec<_>>().join("");

        let rows_html = rows.iter().enumerate().map(|(i, row)| {
            let bg = if i % 2 == 0 { "rgba(255, 255, 255, 0.02)" } else { "rgba(255, 255, 255, 0.05)" };
            let cells = row.iter().map(|cell| {
                format!(r#"<td style="padding: 8px;">{}</td>"#, cell.as_ref())
            }).collect::<Vec<_>>().join("");
            format!(r#"<tr style="background: {};">{}</tr>"#, bg, cells)
        }).collect::<Vec<_>>().join("");

        format!(
            r#"<div style="{}">
                <div style="{}"><span style="{}">{}</span> {}</div>
                <div style="{}"></div>
                <table style="width: 100%; border-collapse: collapse;">
                    <thead><tr>{}</tr></thead>
                    <tbody style="font-size: 13px;">{}</tbody>
                </table>
            </div>"#,
            Self::BASE_STYLE.replace('\n', " "),
            Self::TITLE_STYLE.replace('\n', " "),
            Self::ICON_STYLE.replace('\n', " "),
            icon, title,
            Self::DIVIDER_STYLE.replace('\n', " "),
            header_html,
            rows_html
        )
    }

    /// 状态反馈模板
    ///
    /// # Arguments
    /// * `status` - 状态类型 (success, error, warning, info)
    /// * `icon` - 图标
    /// * `message` - 消息内容
    pub fn status(status: &str, icon: &str, message: &str) -> String {
        let (border_color, bg_glow) = match status {
            "success" => ("rgba(72, 187, 120, 0.6)", "rgba(72, 187, 120, 0.1)"),
            "error" => ("rgba(245, 101, 101, 0.6)", "rgba(245, 101, 101, 0.1)"),
            "warning" => ("rgba(236, 201, 75, 0.6)", "rgba(236, 201, 75, 0.1)"),
            _ => ("rgba(99, 179, 237, 0.6)", "rgba(99, 179, 237, 0.1)"),
        };

        format!(
            r#"<div style="{} border-left: 4px solid {}; box-shadow: 0 8px 32px rgba(0, 0, 0, 0.3), inset 0 1px 0 rgba(255, 255, 255, 0.1), 0 0 20px {};">
                <span style="{}">{}</span>
                <span style="margin-left: 8px;">{}</span>
            </div>"#,
            Self::BASE_STYLE.replace('\n', " "),
            border_color,
            bg_glow,
            Self::ICON_STYLE.replace('\n', " "),
            icon,
            message
        )
    }

    /// 子命令列表模板
    ///
    /// # Arguments
    /// * `icon` - 图标
    /// * `title` - 标题
    /// * `subcommands` - 子命令列表 [(name, description), ...]
    pub fn subcommand_list(
        icon: &str,
        title: &str,
        subcommands: &[(impl AsRef<str>, impl AsRef<str>)]
    ) -> String {
        let items_html = subcommands.iter().map(|(name, desc)| {
            format!(
                r#"<div style="{}">
                    <code style="background: rgba(99, 179, 237, 0.2); color: #63b3ed; padding: 3px 10px; border-radius: 4px; font-size: 13px;">{}</code>
                    <span style="color: #a0aec0; font-size: 13px; margin-left: 12px;">{}</span>
                </div>"#,
                Self::ITEM_STYLE.replace('\n', " "),
                name.as_ref(),
                desc.as_ref()
            )
        }).collect::<Vec<_>>().join("\n");

        format!(
            r#"<div style="{}">
                <div style="{}"><span style="{}">{}</span> {}</div>
                <div style="{}"></div>
                <div style="{}">{}</div>
            </div>"#,
            Self::BASE_STYLE.replace('\n', " "),
            Self::TITLE_STYLE.replace('\n', " "),
            Self::ICON_STYLE.replace('\n', " "),
            icon, title,
            Self::DIVIDER_STYLE.replace('\n', " "),
            Self::CONTENT_STYLE.replace('\n', " "),
            items_html
        )
    }
}

/// 快捷函数：创建信息卡片
pub fn info_card(icon: &str, title: &str, items: &[(impl AsRef<str>, impl AsRef<str>)]) -> String {
    GlassTemplate::info_card(icon, title, items)
}

/// 快捷函数：创建帮助菜单
pub fn help_menu(commands: &[(impl AsRef<str>, impl AsRef<str>, impl AsRef<str>)]) -> String {
    GlassTemplate::help_menu(commands)
}

/// 快捷函数：创建排行榜
pub fn leaderboard(
    icon: &str,
    title: &str,
    headers: &[impl AsRef<str>],
    rows: &[Vec<impl AsRef<str>>]
) -> String {
    GlassTemplate::leaderboard(icon, title, headers, rows)
}

/// 快捷函数：创建成功状态
pub fn success(icon: &str, message: &str) -> String {
    GlassTemplate::status("success", icon, message)
}

/// 快捷函数：创建错误状态
pub fn error(icon: &str, message: &str) -> String {
    GlassTemplate::status("error", icon, message)
}

/// 快捷函数：创建警告状态
pub fn warning(icon: &str, message: &str) -> String {
    GlassTemplate::status("warning", icon, message)
}

/// 快捷函数：创建信息状态
pub fn info(icon: &str, message: &str) -> String {
    GlassTemplate::status("info", icon, message)
}

/// 快捷函数：创建子命令列表
pub fn subcommand_list(
    icon: &str,
    title: &str,
    subcommands: &[(impl AsRef<str>, impl AsRef<str>)]
) -> String {
    GlassTemplate::subcommand_list(icon, title, subcommands)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_info_card() {
        let items = vec![
            ("用户 ID", "@bot:example.com"),
            ("设备 ID", "DEVICE123"),
            ("已加入房间", "5 个"),
        ];
        let html = info_card("🤖", "Bot 信息", &items);
        assert!(html.contains("Bot 信息"));
        assert!(html.contains("@bot:example.com"));
        assert!(html.contains("background:"));
    }

    #[test]
    fn test_help_menu() {
        let commands = vec![
            ("📖", "!help", "查看帮助"),
            ("ℹ️", "!bot info", "Bot 信息"),
        ];
        let html = help_menu(&commands);
        assert!(html.contains("命令帮助"));
        assert!(html.contains("!help"));
    }

    #[test]
    fn test_status() {
        let html = success("✅", "操作成功");
        assert!(html.contains("操作成功"));
        assert!(html.contains("rgba(72, 187, 120"));

        let html = error("❌", "操作失败");
        assert!(html.contains("rgba(245, 101, 101"));
    }
}