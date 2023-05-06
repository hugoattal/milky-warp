# 🌌 Milky Warp

<p align="center"><i>Navigate the galaxy of pixels with Milky Warp!</i></p>

Milky Warp is an open-source tool that displays a magnifier when pressing a shortcut. It's built with Tauri, Vite, Vue, and Typescript.

## Features

- Displays a magnifier when pressing a configurable shortcut
- Supports zoom in and zoom out using the mouse scroll wheel
- Cross-platform: runs on Windows, macOS, and Linux

## Preview

<p align="center"><img src="https://user-images.githubusercontent.com/4563971/236351314-0082007d-e740-47b3-8505-7e79fec0b653.gif" alt="preview"></p>

## Installation

### Windows
Download the latest release from the [releases page](https://github.com/hugoattal/milky-warp/releases)

### macOS
There is no release for macOS yet. You can build the application from source.

### Linux
There is no release for Linux yet. You can build the application from source.

## Build

1. Clone the repository: `git clone https://github.com/hugoattal/milky-warp.git`
2. Install the dependencies: `pnpm install`
3. Build the application: `pnpm run tauri build`
4. Get the executable from the `src-tauri/target` folder

## Usage

1. Press the shortcut key (default: `Ctrl+Alt+A`) to activate the magnifier
2. Use the mouse to move the magnifier around the screen
3. Use the mouse scroll wheel to zoom in and out
4. Click to fix the magnifier position, and click again to release it
5. Press the shortcut key again to deactivate the magnifier

### Experimental

You can also use the shortcut holder. Due to some limitation, it's not as smooth as I would like it to be.
1. Press the shortcut key (default: `Ctrl+Alt+A`) to activate the magnifier
2. Then you can hold and release the hold shortcut (default: `Ctrl+Shit+A`)
3. While the magnifier is hidden using the hold shortcut, you can't click
4. To close this mode, close the magnifier with the normal shortcut (default: `Ctrl+Alt+A`)

## Configuration

You can customize the shortcut key by editing the `src/config.ts` file.

## Contributing

Contributions are welcome!

## Sponsors

- [**Roller Coaster** 🎢](https://roller-coaster.app): Fast track your automations with visual scripting

## License

Milky Warp is released under the [MIT License](LICENSE).
