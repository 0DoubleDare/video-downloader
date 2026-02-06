# video-downloader

Небольшой pet-проект для скачивания видео.

## Stack

- GUI: `iced`
- Async runtime: `tokio`
- Downloader: `yt-dlp`

## Архитектура (MVC)

Код разделен на:

- `src/model.rs` — состояние приложения (Model)
- `src/view.rs` — отрисовка UI (View)
- `src/controller.rs` — обработка сообщений и бизнес-логика (Controller)
- `src/downloader.rs` — инфраструктурная логика скачивания

## NixOS/devShell fix for `cannot find -l:libpthread.a`

Если при сборке под Windows (`x86_64-pc-windows-gnu`) в NixOS вы видите ошибку:

```text
x86_64-w64-mingw32-ld: cannot find -l:libpthread.a
```

в этом репозитории уже добавлена рабочая конфигурация:

- `flake.nix` с mingw toolchain + `mingw_w64_pthreads`;
- `.cargo/config.toml`, где для target `x86_64-pc-windows-gnu`
  отключен `crt-static` (`target-feature=-crt-static`).

### Как использовать

```bash
nix develop
rustup target add x86_64-pc-windows-gnu
cargo build --target x86_64-pc-windows-gnu
```

### Почему это помогает

В некоторых NixOS cross-средах статическая линковка CRT тянет
`-l:libpthread.a`, которого может не быть в текущем mingw runtime.
Динамический CRT (`-crt-static`) убирает это требование и сборка проходит.
