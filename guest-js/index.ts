import { invoke } from '@tauri-apps/api/core';

export async function start(): Promise<void> {
  return await invoke("plugin:downloader|spawn_thread");
}

export async function destroy(): Promise<void> {
  return await invoke("plugin:downloader|destroy_thread");
}