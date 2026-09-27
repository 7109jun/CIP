"""chrome.storage wrapper, plus State: a dict-like object that lazily loads
from chrome.storage.local and automatically persists mutations back.

MV3 background scripts run as a non-persistent service worker: it can be
killed and its top-level code re-run at any time, so any Python-side global
variable is lost between wake-ups unless it's explicitly saved somewhere.
`State` exists so that "somewhere" doesn't require the developer to write
any explicit save/load plumbing -- assigning into a State object schedules
its own save.

Requires `storage = true` under [permissions.chrome] in docs.toml.
"""
from __future__ import annotations
import asyncio
from . import _bridge

_chrome = _bridge.chrome


async def get(keys=None):
    conv = _bridge.to_js(keys) if keys is not None else None
    return await _chrome.storage.local.get(conv)


async def set(items: dict):
    return await _chrome.storage.local.set(_bridge.to_js(items))


async def remove(keys):
    return await _chrome.storage.local.remove(_bridge.to_js(keys))


async def clear():
    return await _chrome.storage.local.clear()


class State(dict):
    """`state = storage.State("counter", default={"count": 0})`

    Call `await state.load()` once (typically at the top of a handler) to
    populate it from chrome.storage.local; every `state[x] = y` after that
    schedules an automatic save back to chrome.storage.local. No explicit
    `.save()` call is needed for the common case.
    """

    def __init__(self, key: str, default=None):
        super().__init__(default or {})
        self._key = key
        self._loaded = False
        self._save_task = None

    async def load(self) -> "State":
        if self._loaded:
            return self
        result = await get([self._key])
        stored = None
        if result is not None:
            try:
                stored = result[self._key]
            except (KeyError, TypeError, AttributeError):
                stored = None
        if stored:
            dict.clear(self)
            dict.update(self, dict(stored))
        self._loaded = True
        return self

    async def save(self):
        await set({self._key: dict(self)})

    def __setitem__(self, k, v):
        dict.__setitem__(self, k, v)
        self._schedule_save()

    def __delitem__(self, k):
        dict.__delitem__(self, k)
        self._schedule_save()

    def _schedule_save(self):
        if not _bridge.IN_BROWSER:
            return
        try:
            self._save_task = asyncio.ensure_future(self.save())
        except RuntimeError:
            # No running event loop (e.g. module import time) -- the next
            # mutation inside a handler will schedule successfully instead.
            pass
