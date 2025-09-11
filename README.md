
# File Organizer

A cross-platform desktop application built with Tauri (Rust) and Vue 3 + TypeScript (Vite) that helps organize files inside a folder by file type and extension. Yeah this is an upgrade or desktop version from [Folder Organizer](https://github.com/AchmadEko003/folder-organizer).

This repository contains:

- A Vue 3 + TypeScript frontend (Vite)
- A Rust backend (Tauri) exposing commands to list and organize files

Key features
- Scan a folder and return a list of files and directories (name, path, size, type)
- Organize files by extension into categorized folders (images, documents, videos, others) and subfolders per extension
- Cross-platform (Linux / macOS / Windows) via Tauri, currently works well on `Windows`

## Quick links

- Frontend entry: `src/main.ts` and `src/App.vue`
- Tauri (Rust) entry: `src-tauri/src/main.rs` and `src-tauri/src/lib.rs`

## Development

1. Install frontend dependencies:

```bash
npm install
```
or
```bash
bun install
```

2. Run tauri development:

```bash
npm run tauri dev
```
or
```bash
bun run tauri dev
```

## Build / Production

1. Build the frontend assets:

```bash
npm run tauri build
```
or
```bash
bun run tauri build
```

Note: platform-specific packaging requires Tauri prerequisites for your OS. See the Tauri docs for details.

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make changes and add tests where appropriate
4. Open a pull request describing your change

## Support / Donate

If you find File Organizer helpful and want to support continued development, you can donate via Saweria for Indonesia.

[Support on Saweria](https://saweria.co/madko003)
