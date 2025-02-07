import { invoke } from "@tauri-apps/api/core";

export interface DownloadableInitialize {
  name: string,
  url: string,
  save: string;
}

export interface Downloadable
  extends DownloadableInitialize {
  paused: boolean,
  speed: number,
  size: number,
  downloaded: number;
}

export async function download(
  downloadable: DownloadableInitialize
): Promise<void> {
  return await invoke("plugin:downloader|download", { downloadable });
}

export async function pause(): Promise<void> {
  return await invoke("plugin:downloader|pause");
}

export async function unpause(): Promise<void> {
  return await invoke("plugin:downloader|unpause");
}

export async function status(): Promise<Downloadable> {
  return await invoke("plugin:downloader|status");
}

export async function cancel(): Promise<void> {
  return await invoke("plugin:downloader|cancel");
}