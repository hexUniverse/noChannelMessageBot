import logging

from .logToChannel import LogToChannel

log: logging.Logger = logging.getLogger(__name__)


class CustomMethods(LogToChannel):
    pass
