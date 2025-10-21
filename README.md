# tauri-plugin-system-fonts

> This plugin only works on tauri v2, if you need the v1 plugin, feel free to submit a PR!

Support getting all fonts installed on your system.

![](https://github.com/user-attachments/assets/7a1a6cfd-68f7-4fd3-a4ad-ed1f7a18ae6d)

## Platform Support

| Platform | Supported |
| -------- | --------- |
| Windows  | ✅        |
| macOS    | ✅        |
| Linux    | ✅        |
| Android  | ❌        |
| iOS      | ❌        |

## Install

```shell
cargo add tauri-plugin-system-fonts
```

You can install the JavaScript Guest bindings using your preferred JavaScript package manager:

```shell
pnpm add tauri-plugin-system-fonts-api
```

## Usage

`src-tauri/src/lib.rs`

```diff
pub fn run() {
    tauri::Builder::default()
+       .plugin(tauri_plugin_system_fonts::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

`src-tauri/capabilities/default.json`

```diff
{
    ...
    "permissions": [
        ...
+       "system-fonts:default"
    ]
}
```

Afterwards all the plugin's APIs are available through the JavaScript guest bindings:

```ts
import { getSystemFonts } from "tauri-plugin-system-fonts-api";

const fonts = await getSystemFonts();
```

## Methods

| Method           | Description                            |
| ---------------- | -------------------------------------- |
| `getSystemFonts` | Get all fonts installed on the system. |

## Example

```shell
git clone https://github.com/ayangweb/tauri-plugin-system-fonts.git
```

```shell
pnpm install

pnpm build

cd examples/tauri-app

pnpm install

pnpm tauri dev
```

## Thanks

- Use [fontdb](https://github.com/RazrFalcon/fontdb) to get all fonts installed on your system.

## Who's Use It

- [EcoPaste](https://github.com/EcoPasteHub/EcoPaste) - Open source cross-platform clipboard management tool.
