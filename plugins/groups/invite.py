import html
import logging

from pyrogram import Client, filters
from pyrogram.types import Message

from main import bot

log: logging.Logger = logging.getLogger(__name__)
__INFO__: str = "請給予刪除訊息及封鎖使用者的權限，本機器人不清除匿名管理員以及連結頻道的發言"

@Client.on_message(filters.group & filters.new_chat_members)
async def invite(cli: Client, msg: Message) -> None:
    if bot.me.id in [_.id for _ in msg.new_chat_members]:
        await msg.reply(__INFO__)
        await bot.log_to_channel(cli, f"Bot invited to <code>{msg.chat.id}</code> "
                                      f"({html.escape(msg.chat.title)}) "
                                      f"by user <code>{msg.from_user.id}</code>: "
                                      f"<a href='tg://user?id={msg.from_user.id}'>"
                                      f"{html.escape(msg.from_user.first_name) if msg.from_user.first_name else ''}"
                                      f"{' ' + html.escape(msg.from_user.last_name) if msg.from_user.last_name else ''}"
                                      f"</a>")
