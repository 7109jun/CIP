"""Thin wrapper over chrome.scripting.
https://developer.chrome.com/docs/extensions/reference/api/scripting

Requires `scripting = true` under [permissions.chrome] in docs.toml.
Note: chrome.scripting.executeScript injects *JavaScript* functions/files,
not Python -- CIP does not attempt to inject the Pyodide runtime into
arbitrary pages this way. Use a declared content script (docs.toml
`extension.content`) to run Python against matched pages instead.
"""
from __future__ import annotations
from . import _bridge

_chrome = _bridge.chrome


async def insert_css(tab_id: int, css: str):
    return await _chrome.scripting.insertCSS(
        _bridge.to_js_dict(target={"tabId": tab_id}, css=css)
    )


async def remove_css(tab_id: int, css: str):
    return await _chrome.scripting.removeCSS(
        _bridge.to_js_dict(target={"tabId": tab_id}, css=css)
    )
