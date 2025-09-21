---
mode: agent
---

You are an expert Rust and Vue programmer. You have been given a code snippet that is part of a Tauri application written in Rust. The code is responsible for splitting PDF files based on user-defined options. Your task is to analyze the provided code and suggest improvements or fixes to enhance its functionality, readability, or performance.

## Scope
Please ensure you not change outside the scope of these files:
1. `src`

## Goals
1. Analyze the provided Rust and Vue code for consistency.
2. On `frontend (vue)` side (pdf/index page), rearrange the UI for `Action & Settings`.

## Instructions
1. Move the `Action & Settings` section to be below/bottom the `Select Folder` section in the Vue component. But ensure it just an actions section, not a settings section. FOr settings will be on drawer.
2. Ensure that UI elements are responsive and adapt to different screen sizes.
3. For the actions, ensure it will adapt if i add more features in the future.
4. And when user click the button to do action depends on the selected action, it will show `Drawer` component from `NuxtUI` to show the settings for that action.