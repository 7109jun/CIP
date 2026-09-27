"""``from cip import chrome`` — the main CIP entry point.

Decorators here register plain Python callables against real browser
events (see cip/runtime.js + the generated background.js/content.js/
popup.js loaders for how the wiring works). The submodules imported below
(``chrome.tabs``, ``chrome.storage``, ...) provide thin, Pythonic wrappers
over the actual chrome.* extension APIs; what's actually *granted* to the
extension is controlled by ``[permissions.chrome]`` in docs.toml.
"""
from __future__ import annotations

from . import _bridge
from . import tabs
from . import windows
from . import storage
from . import runtime
from . import cookies
from . import scripting
from . import notifications
from . import commands
from . import alarms
from . import web_navigation

__all__ = [
    "startup", "on_installed", "command", "on_tab_created",
    "on_tab_updated", "on_tab_removed", "page_load", "on_message",
    "on_alarm", "persistent_state",
    "tabs", "windows", "storage", "runtime", "cookies", "scripting",
    "notifications", "commands", "alarms", "web_navigation",
]


def startup(fn):
    """Runs `fn()` on chrome.runtime.onStartup (browser launch)."""
    return _bridge.register("startup", None, fn)


def on_installed(fn):
    """Runs `fn(details)` on chrome.runtime.onInstalled (install/update)."""
    return _bridge.register("onInstalled", None, fn)


def command(name: str):
    """Runs the decorated function when the keyboard command `name` fires.

    `name` is collected by `cip check`/`cip build` and written into
    manifest.json's `commands` section automatically -- you don't declare
    it in docs.toml.
    """
    def deco(fn):
        return _bridge.register("command", name, fn)
    return deco


def on_tab_created(fn):
    """Runs `fn(tab)` whenever a new tab is created."""
    return _bridge.register("onTabCreated", None, fn)


def on_tab_updated(fn):
    """Runs `fn(tab_id, change_info, tab)` whenever a tab is updated."""
    return _bridge.register("onTabUpdated", None, fn)


def on_tab_removed(fn):
    """Runs `fn(tab_id, remove_info)` whenever a tab is closed."""
    return _bridge.register("onTabRemoved", None, fn)


def page_load(fn):
    """Content-script only: runs `fn(page)` once the page is loaded.
    `page` has `.url` and `.title`."""
    return _bridge.register("pageLoad", None, fn)


def on_message(fn):
    """Runs `fn(message, sender)` for chrome.runtime.onMessage. If `fn`
    returns (or is async and resolves to) a non-None value, that value is
    sent back to the caller as the response."""
    return _bridge.register("onMessage", None, fn)


def on_alarm(fn):
    """Runs `fn(alarm)` for chrome.alarms.onAlarm."""
    return _bridge.register("onAlarm", None, fn)


def persistent_state(key: str, default=None):
    """Shorthand for `storage.State(key, default)` — a dict that lazily
    loads from chrome.storage.local and auto-saves on mutation, so
    surviving MV3 service-worker restarts needs no manual save/load calls.
    """
    return storage.State(key, default)
