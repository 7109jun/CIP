"""Thin wrapper over chrome.notifications.
https://developer.chrome.com/docs/extensions/reference/api/notifications

Requires `notifications = true` under [permissions.chrome] in docs.toml.
"""
from __future__ import annotations
from . import _bridge

_chrome = _bridge.chrome


async def create(notification_id: str, *, title: str, message: str, icon_url: str, type: str = "basic", **kwargs):
    opts = _bridge.to_js_dict(
        type=type, title=title, message=message, iconUrl=icon_url, **kwargs
    )
    return await _chrome.notifications.create(notification_id, opts)


async def clear(notification_id: str):
    return await _chrome.notifications.clear(notification_id)


async def get_all():
    return await _chrome.notifications.getAll()
