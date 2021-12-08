import logging

from pyrogram import Client, filters
from pyrogram.types import Message

log: logging.Logger = logging.getLogger(__name__)
__HELP__: str = f"本機器人能協助封鎖所有透過頻道發送的訊息" \
                f"本機器人會將綁定的頻道以及匿名管理員設為白名單" \
                f"使用方式：給予 刪除訊息 (Delete messages) 及 封鎖使用者 (Ban users) 的權限"


@Client.on_message(filters.command("help") & filters.private & ~ filters.forwarded)
async def help(cli: Client, msg: Message) -> None:
    await msg.reply(__HELP__)
