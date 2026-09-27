"""Thin wrapper over chrome.webNavigation (query side only; for reacting
to navigation events on the Python side, prefer a content script's
`@chrome.page_load`, which fires once your Python code is actually mounted
and running on the page).
https://developer.chrome.com/docs/extensions/reference/api/webNavigation

Requires `web_navigation = true` under [permissions.chrome] in docs.toml
(mapped to Chrome's "webNavigation" permission).
"""
from __future__ import annotations
from . import _bridge

_chrome = _bridge.chrome


async def get_all_frames(tab_id: int):
    return await _chrome.webNavigation.getAllFrames(_bridge.to_js_dict(tabId=tab_id))


async def get_frame(tab_id: int, frame_id: int):
    return await _chrome.webNavigation.getFrame(
        _bridge.to_js_dict(tabId=tab_id, frameId=frame_id)
    )
