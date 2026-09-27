# CIP (Chrome in Python)

Python으로 Chrome / Edge 확장 프로그램을 개발하는 시스템. 새 언어가 아니라
기존 Python 문법 + pip 생태계를 그대로 쓰고, Chrome Extension API를
Python에서 호출할 수 있게 해준다.

## 구조

```
.
├── Cargo.toml / Cargo.lock   # CIP CLI (Rust)
├── src/                      # CLI 소스
├── templates/                # 빌드시 확장에 박히는 JS 로더 템플릿
├── tests/                    # cargo test
├── cip-python/                # `from cip import chrome` Python API 패키지
│                               (cip build가 매 빌드마다 자동 vendoring)
└── .github/workflows/build.yml
```

## 아키텍처 요약

- **실행 방식**: 확장 안에 **Pyodide**(CPython → WASM)를 로컬로 내장.
  numpy 등 실제 C-extension이 WASM 빌드로 존재하는 유일한 실전 옵션이라
  RustPython 대신 Pyodide를 선택.
- **pip**: `requirements.txt`가 진실의 소스. `cip build`가 빌드 시점에
  `pip download`로 wheel을 로컬에 받아 vendoring하고, 브라우저에서는
  micropip이 그 로컬 wheel을 설치 — 원격 코드 실행 없음 (Chrome Web
  Store / Edge Add-ons 정책 준수).
- **MV3 service worker 상태**: `chrome.persistent_state(key, default)`가
  dict-like 객체를 반환하고, 값을 대입하면 `chrome.storage.local`에
  자동 저장됨.

## 로컬 빌드

```bash
cargo test --locked
cargo build --release --locked
./target/release/cip init hello      # (Windows: cip.exe)
cd hello
../target/release/cip fetch-runtime  # Pyodide 코어 런타임 최초 1회
../target/release/cip build
```

## CI

`.github/workflows/build.yml`이 push/PR마다 **windows-latest**에서
`cargo test` + `cargo build --release`를 돌리고, `cip.exe`가 실제로
`version` / `help` / `init` / `check`를 수행하는지 smoke test까지 한 뒤
아티팩트로 업로드한다. 태그 푸시 시에는 릴리즈에 `cip.exe`를 첨부한다.

## 알려진 제한

- numpy 같은 C-extension 패키지는 PyPI에 wasm wheel이 있는 경우만
  지원한다 (모든 pip 패키지가 되는 건 아님).
- `cip fetch-runtime`은 기본 Pyodide 0.26.2를 받는다. 다른 버전을 쓰면
  micropip 번들 wheel 파일명이 달라질 수 있어 `src/fetch_runtime.rs`의
  `CORE_MEMBERS`를 확인해야 할 수 있다.
- 실제 Chrome/Edge 브라우저에 로드해서 UI까지 테스트한 적은 없다
  (Node.js + Pyodide로 데코레이터 등록·이벤트 전달·상태 저장 로직만
  검증함).
