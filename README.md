# Native WebApps

# Description 

Proof of Concept for Shortcut WebApp launching regularly used website in a [tauri](https://tauri.app) (Rust WebView) desktop application.

## Images

![screen-mail-tab](./static/res/screenshot-mail-tab.png)
![screen-calendar-tab](./static/res/screenshot-calendar-tab.png)
![screen-proton-view](./static/res/screenshot-proton-view.png)



## Developing

Install [tauri](https://tauri.app)

Install dependencies with `npm install` (or `pnpm install` or `yarn` / `bun`), start a development server:


```bash
cargo tauri dev

# use --host to expose
cargo tauri dev --host
```
