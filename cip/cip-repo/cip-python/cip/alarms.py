"""Thin wrapper over chrome.alarms (the *set/query* side; handling a
firing alarm is done with the `@chrome.on_alarm` decorator in cip.chrome).
https://developer.chrome.com/docs/extensions/reference/api/alarms

Requires `alarms = true` under [permissions.chrome] in docs.toml.
"""
from __future__ import annotations
from . import _bridge

_chrome = _bridge.chrome


async def create(name: str, *, when=None, delay_in_minutes=None, period_in_minutes=None):
    opts = _bridge.to_js_dict(
        when=when, delayInMinutes=delay_in_minutes, periodInMinutes=period_in_minutes
    )
    return await _chrome.alarms.create(name, opts)


async def get(name: str):
    return await _chrome.alarms.get(name)


async def get_all():
    return await _chrome.alarms.getAll()


async def clear(name: str):
    return await _chrome.alarms.clear(name)


async def clear_all():
    return await _chrome.alarms.clearAll()
