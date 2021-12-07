import html
import logging

from pyrogram import Client, filters
from pyrogram.types import Message

from main import bot

log: logging.Logger = logging.getLogger(__name__)


@Client.on_message(filters.group & filters.new_chat_members)
async def invite(cli: Client, msg: Message) -> None:
    if bot.me.id in [_.id for _ in msg.new_chat_members]:
        await bot.log_to_channel(cli, f"Bot invited to <code>{msg.chat.id}</code> "
                                      f"({html.escape(msg.chat.title)})")
