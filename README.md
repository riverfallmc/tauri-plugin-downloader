# Tauri Plugin downloader

```ts
import {
  spawn,
  add
} from "tauri-plugin-downloader";

import type { Downloadable } from "tauri-plugin-downloader";

(async () => {
  // spawns thread for downloader
  await spawn();

  // add download object to download query
  await add({
    id: "",
    name: "",
    url: "",
    save: ""
  })

  // returns status of download
  await status(id);

  const query: Downloadable[] = await getQuery();
})();
```