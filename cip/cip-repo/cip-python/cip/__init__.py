"""CIP (Chrome in Python) runtime API.

Typical usage:

    from cip import chrome

    @chrome.startup
    def startup():
        print("Extension started")

    @chrome.command("hello")
    def hello():
        chrome.tabs.create("https://example.com")

See chrome.py for the full decorator list, and tabs.py / storage.py /
runtime.py / cookies.py / scripting.py / notifications.py / commands.py /
alarms.py / web_navigation.py for the per-API wrappers.

This package works two ways:
  - Inside a built CIP extension, running under Pyodide in the browser
    (chrome.* calls hit the real extension APIs).
  - In a plain local `pip install`, for editor support and for
    `cip check`'s Python syntax/import validation (chrome.* calls become
    harmless no-ops via cip._bridge's mock).
"""
from . import chrome  # noqa: F401

__version__ = "0.1.0"
__all__ = ["chrome"]
