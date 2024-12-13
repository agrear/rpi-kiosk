import { invoke } from "@tauri-apps/api/tauri";

let kioskEl: HTMLIFrameElement | null;

async function getUrl(): Promise<string> {
  return await invoke('get_url');
}

window.addEventListener("DOMContentLoaded", async () => {
  kioskEl = document.querySelector('#kiosk');

  if (kioskEl) {
    kioskEl.src = await getUrl();
  }
});
