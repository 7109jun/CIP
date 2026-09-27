// CIP runtime bootstrap.
// Loads a locally-vendored Pyodide (never from a CDN: Chrome Web Store /
// Edge Add-ons policy forbids remotely-hosted executable code), mounts the
// project's Python sources into the Pyodide virtual filesystem, installs
// declared pip dependencies via micropip, then imports the entry module for
// whichever context this file was loaded from (background/content/popup).
//
// Python-side decorators (cip.chrome.startup / .command / .on_tab_created /
// .page_load / ...) register themselves into `self.__cip_registry` (via
// cip._bridge.register, exposed to Python as a JS callable). The per-context
// loader files (background.js / content.js / popup.js) read that registry
// and wire it to the real chrome.* event listeners.

self.__cip_registry = {
  startup: [],
  onInstalled: [],
  commands: {},
  onTabCreated: [],
  onTabUpdated: [],
  onTabRemoved: [],
  pageLoad: [],
  onMessage: [],
  onAlarm: [],
};

function __cip_register(kind, key, fn) {
  const reg = self.__cip_registry;
  if (kind === "command") {
    reg.commands[key] = fn;
  } else if (kind in reg) {
    reg[kind].push(fn);
  } else {
    console.warn("[CIP] unknown registry kind:", kind);
  }
}
self.__cip_register = __cip_register;

let __cip_pyodide_promise = null;

/**
 * cip/pyodide/pyodide.js (which defines the global `loadPyodide`) must
 * already be loaded before this file runs. Background service workers do
 * that via importScripts(); content scripts and popup.html do it by
 * listing cip/pyodide/pyodide.js before cip/runtime.js in the manifest /
 * <script> tags. This file never fetches remote code itself.
 *
 * @param {string[]} pyFiles - relative paths of .py files to mount, e.g.
 *   ["main.py", "background.py", "cip/__init__.py", "cip/chrome.py", ...]
 * @param {string} entryModule - the module to `import` after mounting,
 *   e.g. "background" (from background.py at the project root).
 * @param {string[]} dependencies - pip package specs from requirements.txt,
 *   installed at runtime via micropip (PyPI pure-python wheels, or WASM
 *   builds of C-extensions when available).
 */
async function cipInit(pyFiles, entryModule, dependencies) {
  if (!__cip_pyodide_promise) {
    __cip_pyodide_promise = loadPyodide({ indexURL: "./cip/pyodide/" });
  }
  const pyodide = await __cip_pyodide_promise;

  // Mount project sources under /cip_project and add to sys.path once.
  if (!self.__cip_mounted) {
    self.__cip_mounted = true;
    pyodide.FS.mkdirTree("/cip_project");
    const resolveUrl = (rel) =>
      typeof chrome !== "undefined" && chrome.runtime && chrome.runtime.getURL
        ? chrome.runtime.getURL(rel)
        : rel;
    for (const rel of pyFiles) {
      const resp = await fetch(resolveUrl(rel));
      const text = await resp.text();
      const full = "/cip_project/" + rel;
      const dir = full.substring(0, full.lastIndexOf("/"));
      pyodide.FS.mkdirTree(dir);
      pyodide.FS.writeFile(full, text);
    }
    pyodide.runPython(
      "import sys\n" +
      "for p in ('/cip_project/cip_pkg', '/cip_project/cip_pkg/project'):\n" +
      "    if p not in sys.path:\n" +
      "        sys.path.insert(0, p)\n"
    );

    if (dependencies && dependencies.length > 0) {
      await pyodide.loadPackage("micropip");
      const micropip = pyodide.pyimport("micropip");
      for (const dep of dependencies) {
        // A local wheel bundled by `cip build` (recommended, and required
        // for Web Store / Add-ons Store compliance: no remotely hosted
        // code). A bare package name here means vendoring failed at build
        // time and this will try to resolve over the network instead --
        // `cip build` already prints a warning for that case.
        const target = dep.endsWith(".whl") ? resolveUrl(dep) : dep;
        try {
          await micropip.install(target);
        } catch (e) {
          console.error("[CIP] failed to install dependency via micropip:", dep, e);
        }
      }
    }
    // (No extra wiring needed here: self.__cip_register was already
    // installed at the top of this file, before any Python code ran, and
    // Pyodide's `js` module reflects that same global scope.)
  }

  await pyodide.runPythonAsync(`import ${entryModule}`);
  return pyodide;
}
