# Tauri + Vue 3 + TypeScript

## CDP Desktop application plan

The product and contributor documentation for the Composers' Desktop Project
application is split into six documents. Read them in this order:

1. [Product roadmap](docs/app/00-product-roadmap.md)
2. [Experience and visual design](docs/app/01-experience-and-visual-design.md)
3. [Process catalog contract](docs/app/02-process-catalog-contract.md)
4. [Desktop runtime and delivery](docs/app/03-desktop-runtime-and-delivery.md)
5. [Adding processes](docs/app/04-adding-processes.md)
6. [Cross-platform builds](docs/app/05-cross-platform-builds.md)

The first release described by these documents is an Intel macOS vertical
slice. Its application and catalog contracts are intentionally platform-neutral
so that Apple Silicon, Windows, Linux, and broader CDP process coverage can be
added without redesigning the UI or IPC boundary.

To stage the Intel macOS sidecars for local development, run
`bash scripts/stage-cdp-binaries.sh`. The script copies and validates the five
required executables from `cdpr8/_cdp/_cdprogs/` into `src-tauri/binaries/`
with Tauri's `-x86_64-apple-darwin` suffix. Redistribution rights, code signing,
and notarization remain release gates; this repository does not sign or
distribute these binaries.

This template should help get you started developing with Vue 3 and TypeScript in Vite. The template uses Vue 3 `<script setup>` SFCs, check out the [script setup docs](https://v3.vuejs.org/api/sfc-script-setup.html#sfc-script-setup) to learn more.

## Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/) + [Volar](https://marketplace.visualstudio.com/items?itemName=Vue.volar) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

## Type Support For `.vue` Imports in TS

Since TypeScript cannot handle type information for `.vue` imports, they are shimmed to be a generic Vue component type by default. In most cases this is fine if you don't really care about component prop types outside of templates. However, if you wish to get actual prop types in `.vue` imports (for example to get props validation when using manual `h(...)` calls), you can enable Volar's Take Over mode by following these steps:

1. Run `Extensions: Show Built-in Extensions` from VS Code's command palette, look for `TypeScript and JavaScript Language Features`, then right click and select `Disable (Workspace)`. By default, Take Over mode will enable itself if the default TypeScript extension is disabled.
2. Reload the VS Code window by running `Developer: Reload Window` from the command palette.

You can learn more about Take Over mode [here](https://github.com/johnsoncodehk/volar/discussions/471).
