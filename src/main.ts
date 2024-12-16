import { invoke } from '@tauri-apps/api/tauri';
import arg from 'arg';

let kioskEl: HTMLIFrameElement | null;

async function getArgs(): Promise<string[]> {
  return await invoke('get_args');
}

async function parseArgs() {
  // Get command line arguments.
  return arg(
    {
      '--width': String,
      '--height': String,
      '--url': String,
      '--hide-cursor': Boolean,
      '-w': '--width',
      '-h': '--height'
    },
    {
      argv: (await getArgs()).slice(1)
    }
  );
}

window.addEventListener('DOMContentLoaded', async () => {
  // Get command line arguments.
  const args = await parseArgs();

  kioskEl = document.querySelector('#kiosk');

  if (kioskEl) {
    if (args['--hide-cursor']) {
      // Workaround for https://github.com/tauri-apps/tauri/issues/10231
      document.body.requestPointerLock();
    }

    kioskEl.src =  args['--url'] ?? args['_'][0];

    if (args['--width']) {
      kioskEl.style.width = args['--width'];
    }

    if (args['--height']) {
      kioskEl.style.height = args['--height'];
    }
  }
});
