<p align="center">
    <img src="src/resources/termdot_icon.png" alt="Termdot Grey Icon" />
</p>

<h1 align="center"> Termdot </h1>

<div align="center" >

![Language](https://img.shields.io/badge/Language-Rust-FFF7E9) ![License](https://img.shields.io/badge/License-MIT-B9E0FF) ![Support](https://img.shields.io/badge/Support-Windows-CD97F9)[![Join Discord](https://img.shields.io/discord/1358801061414047774?label=Discord&logo=discord&style=flat)](https://discord.gg/phg7YvSStX)

</div>

**_Terminal built for Godot._**

## Features

- **Runtime command execution console** – interact with the game during runtime.
- **Command history caching and auto-completion** – recall and auto-fill previous inputs.
- **Command scripting with execution status handling** – write reusable scripts and track results.
- **Extensible and easily integrated** – fits smoothly into existing projects.
- **Log and data inspection windows** – view logs and runtime data visually.

<!-- ![Termdot Display](src/resources/termdot_display.gif) -->

![Termdot Image](src/resources/termdot.png)

## Usage

### Installation

1. Copy the `addons/termdot` folder into your project's `addons` directory.
2. In your scene, add a `Termdot` node. This is the main control node for the plugin.
3. Under the `Termdot` node, add `Command` nodes (each `Command` node must be a child of `Termdot`).
4. Write your own command scripts to define behavior.

### Creating a Command Script

Example `Command.gd`:

```gdscript
extends Command

# This method is executed when the command is detected.
# The command is trimmed by spaces, and parameters are passed as `params`.
func _start(params: Array[String]) -> int:
    # Return values:
    # ExecuteStatus.DONE
    # ExecuteStatus.RUNNING

# This method executes when `_start()` returns `ExecuteStatus.RUNNING` and continues
# running until `_running()` itself returns `ExecuteStatus.DONE`.
func _running() -> int:
    var text = AnsiString.new().background_rgb(112, 112, 112).foreground_256(3).italic().append("Hello World").de_italic().clear_style().append("\r\nHello You\r\n")

    # This will print `text` to the terminal. `text` is a special string that can contain
    # control characters and escape sequences for stylized output and cursor control.
    echo(text)

    # Return values:
    # ExecuteStatus.DONE
    # ExecuteStatus.RUNNING
```

## Builtin Functions

| Key              | Function                                       |
| ---------------- | ---------------------------------------------- |
| ↑ / ↓            | History commands select.                       |
| Tab              | Commands list, auto completion.                |
| Control + C      | Interrupt current running command.             |
| Control + Insert | Copy selected text to clipboard from terminal. |
| Shift + Insert   | Paste text from clipboard to terminal.         |

| Command | Function                                                                    |
| ------- | --------------------------------------------------------------------------- |
| version | Show current Termdot version.                                               |
| cls     | Clear entire screen.                                                        |
| log     | Display logs recorded by Termdot.log(), Termdot.warn(), and Termdot.error() |

## License

[MIT license](LICENSE) © Joe Zane
