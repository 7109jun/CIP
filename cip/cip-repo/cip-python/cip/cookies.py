"""Thin wrapper over chrome.cookies.
https://developer.chrome.com/docs/extensions/reference/api/cookies

Requires `cookies = true` under [permissions.chrome] in docs.toml, plus
host access to the relevant site(s) under [permissions].webpages.
"""
from __future__ import annotations
from . import _bridge

_chrome = _bridge.chrome


async def get(url: str, name: str, **kwargs):
    return await _chrome.cookies.get(_bridge.to_js_dict(url=url, name=name, **kwargs))


async def get_all(**kwargs):
    return await _chrome.cookies.getAll(_bridge.to_js_dict(**kwargs))


async def set(url: str, name: str, value: str, **kwargs):
    return await _chrome.cookies.set(
        _bridge.to_js_dict(url=url, name=name, value=value, **kwargs)
    )


async def remove(url: str, name: str):
    return await _chrome.cookies.remove(_bridge.to_js_dict(url=url, name=name))
