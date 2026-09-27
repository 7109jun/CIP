"""Thin wrapper over chrome.tabs.
https://developer.chrome.com/docs/extensions/reference/api/tabs

Requires `tabs = true` under [permissions.chrome] in docs.toml.
"""
from __future__ import annotations
from . import _bridge

_chrome = _bridge.chrome


def create(url: str, *, active=None, pinned=None, index=None, window_id=None):
    """Opens a new tab. Fire-and-forget (matches `chrome.tabs.create(...)`
    called without a callback in JS); use `create_async` if you need the
    created Tab object back."""
    opts = _bridge.to_js_dict(
        url=url, active=active, pinned=pinned, index=index, windowId=window_id
    )
    return _chrome.tabs.create(opts)


async def create_async(url: str, **kwargs):
    opts = _bridge.to_js_dict(url=url, **kwargs)
    return await _chrome.tabs.create(opts)


async def query(**kwargs):
    """`await tabs.query(active=True, currentWindow=True)`"""
    return await _chrome.tabs.query(_bridge.to_js_dict(**kwargs))


async def update(tab_id: int, **kwargs):
    return await _chrome.tabs.update(tab_id, _bridge.to_js_dict(**kwargs))


async def remove(tab_id: int):
    return await _chrome.tabs.remove(tab_id)


async def get(tab_id: int):
    return await _chrome.tabs.get(tab_id)


async def send_message(tab_id: int, message):
    return await _chrome.tabs.sendMessage(tab_id, message)


async def reload(tab_id: int, *, bypass_cache=None):
    return await _chrome.tabs.reload(tab_id, _bridge.to_js_dict(bypassCache=bypass_cache))
