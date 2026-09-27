"""Thin wrapper over chrome.windows.
https://developer.chrome.com/docs/extensions/reference/api/windows

Requires `windows = true` under [permissions.chrome] in docs.toml.
"""
from __future__ import annotations
from . import _bridge

_chrome = _bridge.chrome


async def create(**kwargs):
    """`await windows.create(url="https://example.com", type="popup")`"""
    return await _chrome.windows.create(_bridge.to_js_dict(**kwargs))


async def get(window_id: int, **kwargs):
    return await _chrome.windows.get(window_id, _bridge.to_js_dict(**kwargs))


async def get_current(**kwargs):
    return await _chrome.windows.getCurrent(_bridge.to_js_dict(**kwargs))


async def get_all(**kwargs):
    return await _chrome.windows.getAll(_bridge.to_js_dict(**kwargs))


async def update(window_id: int, **kwargs):
    return await _chrome.windows.update(window_id, _bridge.to_js_dict(**kwargs))


async def remove(window_id: int):
    return await _chrome.windows.remove(window_id)
