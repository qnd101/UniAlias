# UniAlias
Quickly insert Unicode characters anywhere by typing aliases!

## File Structure
File structure is typical of a Tauri project, with an additional `dataset` folder.

- `src-tauri` contains the backend Rust project.
- `frontend` contains a vanilla Vite project.
- `dataset` contains data for alias-to-unicode mapping. For more info, refer to the documentation inside.

## Usage
1) Type the alias in the textbox. You can navigate through autocompletions using `Tab`, `Up`, and `Down`. 
2) Select a completion with `Enter`. This will close the window and simulate an insert of the corresponding Unicode character. Press `Esc` if you want to close the window without selecting a completion. 
3) The program continues to run in the background after the window is gone. You can reopen the window by clicking its icon in the system tray, or simply by the hotkey `Shift+Alt+U`
4) To stop the program, first right-click its icon in the system tray. This will show the menu strip, containing `Exit`.

## Datasets
The program reads all of its alias data from the appdata folder. Specifically, it reads through every csv file under
```
<appdata>/com.qnd101.unialias.app/dataset/
```

In Windows, `<appdata> = $env:APPDATA`. If you are using a different OS, search which folder tauri uses to store application data.

#### Formatting
Each line in the .csv file should look like: 
```csv
<alias>,<unicode>
```

(ex. `alpha,α`)

Note that,
- `<alias>` should consist only of ASCII characters, excluding whitespace. 
- `<unicode>` should be a single UTF-8 character.
- You may put comment lines starting with `#`. They are ignored when parsing.
- Excess spacing doesn't matter. (mostly)

## Setup
```bash
git clone https://github.com/qnd101/UniAlias
cd UniAlias
npm install
```

#### Debug
```bash
npm run tauri dev
```
#### Build
```bash
npm run tauri build
```
This will create executable files under `src-tauri/target/release`

## Notes
Please open an issue if you find something wrong!