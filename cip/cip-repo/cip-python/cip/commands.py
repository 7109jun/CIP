"""Thin wrapper over chrome.commands (the *query* side; registering a
handler for a command is done with the `@chrome.command("name")`
decorator in cip.chrome, not here).
https://developer.chrome.com/docs/extensions/reference/api/commands
"""
from __future__ import annotations
from . import _bridge

_chrome = _bridge.chrome


async def get_all():
    return await _chrome.commands.getAll()
