"""Low-level bridge between CIP's Python API and the browser.

Inside Pyodide, ``js`` exposes real browser globals (``chrome.*``,
``window``, ``document``, ...) with automatic JS<->Python type conversion,
including calling JS functions that return Promises as awaitables. Outside
the browser -- e.g. a developer's local venv used only for editor/type
support, or the ``python3 -m py_compile`` syntax check that ``cip check``
runs -- there is no ``js`` module, so this falls back to an inert mock that
lets ``import cip`` (and decorator application) succeed without a real
Chrome runtime.
"""
from __future__ import annotations

try:
    import js  # type: ignore
    from pyodide.ffi import create_proxy, to_js as _pyodide_to_js  # type: ignore
    from js import Object as _JSObject  # type: ignore

    IN_BROWSER = True
    chrome = js.chrome
except ImportError:
    IN_BROWSER = False

    class _MockCall:
        def __call__(self, *args, **kwargs):
            return None

        def __getattr__(self, _name):
            return _MockCall()

        def __await__(self):
            async def _noop():
                return None

            return _noop().__await__()

    class _MockChrome:
        def __getattr__(self, _name):
            return _MockCall()

    chrome = _MockChrome()  # type: ignore


_REGISTRY_KINDS = (
    "startup",
    "onInstalled",
    "command",
    "onTabCreated",
    "onTabUpdated",
    "onTabRemoved",
    "pageLoad",
    "onMessage",
    "onAlarm",
)

# Only used outside the browser (local dev / tests), so decorators still
# work and registered handlers can be introspected without a real runtime.
local_registry: dict = {k: [] for k in _REGISTRY_KINDS}


def register(kind: str, key, fn):
    """Registers `fn` against the JS-side event registry in cip/runtime.js.

    `key` is the command name when kind == "command"; ignored otherwise.
    Returns `fn` unchanged so this can be used directly as a decorator.
    """
    if IN_BROWSER:
        # self.__cip_register is installed by cip/runtime.js at load time,
        # before any Python code runs; `js` reflects that same global scope.
        # create_proxy: without it, Pyodide auto-destroys the JS-side proxy
        # for `fn` right after this call returns, so a later browser event
        # trying to call it would fail -- these handlers must outlive the
        # registration call itself.
        js.__cip_register(kind, key, create_proxy(fn))
    else:
        local_registry.setdefault(kind, []).append((key, fn))
    return fn


def to_js(value):
    """Converts a Python value (dict/list/primitive) into a *real* JS
    value -- a plain object (not a Map) for dicts, a real Array for
    lists/tuples -- suitable for passing into an actual chrome.* call.

    This matters, not just style: Pyodide's default argument conversion
    when calling a JsProxy method leaves Python containers wrapped as
    PyProxy objects (and even pyodide.ffi.to_js's own default turns dicts
    into JS Maps, not plain objects), and Chrome's real extension APIs
    read plain object properties (`options.url`) -- they can't read a Map
    or a bare PyProxy.
    """
    if not IN_BROWSER:
        return value
    return _pyodide_to_js(value, dict_converter=_JSObject.fromEntries)


def to_js_dict(**kwargs):
    """Drops kwargs whose value is None (so callers get Chrome's own
    defaults for anything unset), then converts the rest with `to_js`."""
    d = {k: v for k, v in kwargs.items() if v is not None}
    return to_js(d)
