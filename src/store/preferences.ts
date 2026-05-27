// Preferences are managed entirely in Rust via tauri-plugin-store.
// This file is a placeholder for future frontend-side preference reads
// if needed (e.g. switching cat image sets from the UI).

export type CatSet = "cat-set-1" | "cat-set-2";

export interface Preferences {
  intervalMinutes: 30 | 45 | 60;
  paused: boolean;
  catSet: CatSet;
}

export const DEFAULT_PREFERENCES: Preferences = {
  intervalMinutes: 45,
  paused: false,
  catSet: "cat-set-1",
};
