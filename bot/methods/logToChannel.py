import logging
import os

from pyrogram import Client

log: logging.Logger = logging.getLogger(__name__)


class LogToChannel:
    async def log_to_channel(self, cli: Client, text: str):
        await cli.send_message(os.getenv("TG_CHANNEL"), text, reply_to_message_id=os.getenv("TG_TOPIC"))
