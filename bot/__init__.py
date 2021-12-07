import asyncio
import json
import logging
import os
import platform
import sys
from asyncio import AbstractEventLoop
from datetime import datetime
from typing import Optional, Union

from pyrogram import Client
from pyrogram.errors import ApiIdInvalid, AuthKeyUnregistered
from pyrogram.session import Session
from pyrogram.types import User

from bot.methods import CustomMethods

log: logging.Logger = logging.getLogger(__name__)
__version__: str = "1.0"


class Bot(CustomMethods):
    _instance: Union[None, "Bot"] = None

    me: Optional[User] = None
    version: str = __version__
    device_model: str = f"PC {platform.architecture()[0]}"
    system_version: str = f"{platform.system()} {platform.python_implementation()} {platform.python_version()}"

    def __init__(self):
        test_mode: bool = json.loads(os.getenv("TEST_MODE").lower())
        self.app: Client = Client(
            "test" if test_mode else "bot",
            app_version=self.version,
            device_model=self.device_model,
            api_id=os.getenv("API_ID"),
            api_hash=os.getenv("API_HASH"),
            test_mode=test_mode,
            plugins=None,
            system_version=self.system_version
        )

        self.start_time: datetime = datetime.utcnow()

    def __new__(cls, *args, **kwargs):
        if cls._instance is None:
            cls._instance = super().__new__(cls)
        return cls._instance

    def run(self):
        if self.app.test_mode:
            log.debug("[Bot] Warning: bot is running in test mode!")

        loop: AbstractEventLoop = asyncio.get_event_loop()
        run = loop.run_until_complete

        run(self._pyrogram_testing())

        log.info("[Bot] Loading plugins!")

        self.app.plugins = {
            "enabled": True,
            "root": "plugins",
            "include": [],
            "exclude": []
        }

        log.info("[Bot] Plugins loaded!")

        self.app.run()

    async def _pyrogram_testing(self):
        # Disable notice
        Session.notice_displayed = True
        logging.getLogger("pyrogram").setLevel(logging.WARNING)

        log.debug("[Bot] Initializing pyrogram...")

        try:
            await self.app.start()

        except (ApiIdInvalid, AttributeError):
            log.critical("[Bot] Api ID is invalid")
            sys.exit(1)

        except AuthKeyUnregistered:
            log.critical("[Bot] Session expired!")
            log.critical("      Removing old session and exiting!")
            await self.app.storage.delete()
            exit(1)

        try:
            me: User = await self.app.get_me()

            info_str: str = f"[Bot] {me.first_name}"
            info_str += f" {me.last_name}" if me.last_name else ""
            info_str += f" (@{me.username})" if me.username else ""
            info_str += f" ID: {me.id}"

            log.info(info_str)

            self.me: User = me

        except Exception as e:
            log.exception(e)
            sys.exit(1)

        log.debug("[Bot] Pyrogram initialized successfully!")

        await self.app.stop()
        logging.getLogger("pyrogram").setLevel(logging.INFO)
