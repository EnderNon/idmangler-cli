# idmangler-cli
Basically this generates fake items by loading from a json. Effectively a wrapper for [zatzou's idmangler-lib](https://github.com/Zatzou/idmangler-lib).

Any bugs? DM endernon on discord.


## How to get the program
### Option 1: prebuilt
Go to the [releases tab](https://git.frfrnocap.men/endernon/idmangler-cli/releases)
### Option 2: compile yourself
requirements: rust, cargo  
use `cargo run --release` to build and run.  
### Option 3: get from crates.io
Requirements: rust-lang  
run `cargo install idmangler-cli`.

## Usage

### 1) Setting up
- #### 1a) Linux:
  - Open your preferred terminal emulator, and make sure the working directory is the directory with idmangler-cli extracted inside.
  - Otherwise, if it's the wrong directory, run `cd PATH_TO_IDMANGLER_CLI_DIR` where `PATH_TO_IDMANGLER_CLI_DIR` is where idmangler-cli is stored.
  - Run `chmod +x idmangler-cli` because by default it doesn't have executable permissions.
- #### 1b) Windows:
  - Open the folder where idmangler-cli is extracted to in Windows File Explorer. 
  - Then, click the empty space in the top bar (it shows the path e.g. "This PC > Downloads") and type `cmd` and press enter.
  - This should open a Command Prompt.
  - At the start of the line displayed, it should say the path to the folder where File Explorer is open (e.g. "This PC > Downloads").
### 2) Getting necessary information files / Updating the necessary information files
- These files are from wynntils, and are data files necessary to the operation of the program. 
- There are two methods of getting these information files.
- #### 1) Automatic download
  - Windows: run `idmangler-cli.exe --download all`
  - Linux: run `idmangler-cli --download all`
- #### 2) Manual download
  - Move the program to some folder along with config.json . Download these three files and place them next to the program:  
    - https://raw.githubusercontent.com/Wynntils/Static-Storage/main/Reference/id_keys.json  
    - https://raw.githubusercontent.com/Wynntils/Static-Storage/main/Data-Storage/shiny_stats.json
- ### 3) Actually generating an item
- Read through the provided `config.md` document. You can get the web version [HERE](https://git.frfrnocap.men/endernon/idmangler-cli/src/branch/main/config.md).
- Once you have read through it, use one of the provided json files to create your own json.
- Run the program with it.
  - Windows: run `idmangler-cli.exe --config CONFIG_FILE_PATH` 
  - Linux: run `idmangler-cli --config CONFIG_FILE_PATH`
  - Where CONFIG_FILE_PATH is the actual path to the file.
- Now, what happens is there should be an output line that has the encoded file.
- Copy the whole line and paste it into minecraft chat or whatever.