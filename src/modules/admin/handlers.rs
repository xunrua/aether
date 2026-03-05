//! Admin 命令处理器

use anyhow::Result;
use async_trait::async_trait;
use matrix_sdk::ruma::events::room::message::RoomMessageEventContent;

use crate::command::{CommandContext, CommandHandler, Permission};

/// Bot 信息命令处理器
pub struct BotInfoHandler;

#[async_trait]
impl CommandHandler for BotInfoHandler {
    fn name(&self) -> &str {
        "bot"
    }

    fn description(&self) -> &str {
        "Bot 管理命令"
    }

    fn usage(&self) -> &str {
        "bot info - 查看 Bot 信息"
    }

    fn permission(&self) -> Permission {
        Permission::Anyone
    }

    async fn execute(&self, ctx: &CommandContext<'_>) -> Result<()> {
        let sub = ctx.sub_command();

        match sub {
            Some("info") => self.handle_info(ctx).await,
            Some("ping") => self.handle_ping(ctx).await,
            _ => {
                let help = "可用子命令:\n\
                    • info - 查看 Bot 信息\n\
                    • ping - 测试响应延迟";
                send_text(&ctx.room, help).await
            }
        }
    }
}

impl BotInfoHandler {
    async fn handle_info(&self, ctx: &CommandContext<'_>) -> Result<()> {
        let user_id = ctx.client.user_id()
            .map(|u| u.to_string())
            .unwrap_or_else(|| "未知".to_string());

        let device_id = ctx.client.device_id()
            .map(|d| d.to_string())
            .unwrap_or_else(|| "未知".to_string());

        let rooms_count = ctx.client.joined_rooms().len();

        let info = format!(
            "**Bot 信息**\n\
            • 用户 ID: {}\n\
            • 设备 ID: {}\n\
            • 已加入房间: {} 个",
            user_id, device_id, rooms_count
        );

        send_text(&ctx.room, &info).await
    }

    async fn handle_ping(&self, ctx: &CommandContext<'_>) -> Result<()> {
        send_text(&ctx.room, "🏓 Pong!").await
    }
}

/// Bot 离开房间命令处理器
pub struct BotLeaveHandler;

#[async_trait]
impl CommandHandler for BotLeaveHandler {
    fn name(&self) -> &str {
        "leave"
    }

    fn description(&self) -> &str {
        "让 Bot 离开当前房间"
    }

    fn usage(&self) -> &str {
        "leave"
    }

    fn permission(&self) -> Permission {
        Permission::RoomMod
    }

    async fn execute(&self, ctx: &CommandContext<'_>) -> Result<()> {
        let room_id = ctx.room_id();

        // 发送告别消息
        send_text(&ctx.room, &format!("再见！正在离开房间 {} ...", room_id)).await?;

        // 离开房间
        ctx.room.leave().await?;

        Ok(())
    }
}

/// Bot Ping 命令处理器
pub struct BotPingHandler;

#[async_trait]
impl CommandHandler for BotPingHandler {
    fn name(&self) -> &str {
        "ping"
    }

    fn description(&self) -> &str {
        "测试 Bot 响应"
    }

    fn usage(&self) -> &str {
        "ping"
    }

    fn permission(&self) -> Permission {
        Permission::Anyone
    }

    async fn execute(&self, ctx: &CommandContext<'_>) -> Result<()> {
        send_text(&ctx.room, "🏓 Pong!").await
    }
}

/// 发送文本消息
async fn send_text(room: &matrix_sdk::Room, text: &str) -> Result<()> {
    room.send(RoomMessageEventContent::text_plain(text)).await?;
    Ok(())
}