import logging

import pyrogram
from pyrogram import Client, filters
from pyrogram.types import Message

import main

log: logging.Logger = logging.getLogger(__name__)


@Client.on_message(filters.group & ~ filters.service)
async def ban_channel_message(cli: Client, msg: Message) -> None:
    me: pyrogram.types.User = await cli.get_me()
    chat: pyrogram.types.Chat = await cli.get_chat(msg.chat.id)

    white_list: list = [msg.chat.id, chat.linked_chat.id]

    # allow: linked_chat, anonymous_admin
    if msg.sender_chat and msg.sender_chat.id not in white_list:
        # check permission, leave if no permission
        permission: pyrogram.types.ChatMember = await cli.get_chat_member(msg.chat.id, me.id)
        if not (permission.can_delete_messages and permission.can_restrict_members):
            # permission not allowed
            await cli.leave_chat(msg.chat.id)
            await main.bot.log_to_channel(cli, f"Leaving chat <code>{msg.chat.id}</code> "
                                               f"cause {permission.can_delete_messages} "
                                               f"and {permission.can_restrict_members}")
            return

        await msg.delete()
        await msg.reply("Channel message is not allowed!")
        await cli.kick_chat_member(msg.chat.id, msg.sender_chat.id)
