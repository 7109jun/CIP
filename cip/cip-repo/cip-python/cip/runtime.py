"""Thin wrapper over chrome.runtime.
https://developer.chrome.com/docs/extensions/reference/api/runtime
"""
from __future__ import annotations
from . import _bridge

_chrome = _bridge.chrome


def get_url(path: str) -> str:
    return _chrome.runtime.getURL(path)


def get_manifest():
    return _chrome.runtime.getManifest()


def get_id() -> str:
    return _chrome.runtime.id


async def send_message(message, extension_id: str | None = None):
    if extension_id:
        return await _chrome.runtime.sendMessage(extension_id, message)
    return await _chrome.runtime.sendMessage(message)


async def open_options_page():
    return await _chrome.runtime.openOptionsPage()
