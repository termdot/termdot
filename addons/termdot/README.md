<p align="center">
    <img src="https://raw.githubusercontent.com/termdot/termdot/master/src/resources/termdot_icon.png" alt="Termdot Grey Icon" />
</p>

<h1 align="center"> Termdot </h1>

<div align="center" >

![Language](https://img.shields.io/badge/Language-Rust-FFF7E9) ![License](https://img.shields.io/badge/License-MIT-B9E0FF) ![Support](https://img.shields.io/badge/Support-Windows-CD97F9)[![Join Discord](https://img.shields.io/discord/1358801061414047774?label=Discord&logo=discord&style=flat)](https://discord.gg/phg7YvSStX)

</div>

**_Terminal built for Godot, Improve your workflow throughout the whole game development cycle._**

## Features

- **Local shell support** – interact with Cmd, PowerShell etc.
- **Game runtime command execution console** – Behavior your own command and interact with the game during runtime.
- **Log and data inspection windows** – view logs and runtime data visually.
- **Capture Godot's standard output and errors with timestamps automatically** - inspect error data on release export.
- **Extensible and easily integrated** – fits smoothly into existing projects.

<!-- ![Termdot Display](src/resources/termdot_display.gif) -->

![Termdot Image](https://raw.githubusercontent.com/termdot/termdot/master/src/resources/termdot.png)
![Termdot Log Capture](https://raw.githubusercontent.com/termdot/termdot/master/src/resources/log_capture.png)

## Usage

### Installation

1. Search **Termdot** in Godot's Asset Lib to install, or just copy the `addons/termdot` folder into your project's `addons` directory.
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
    #
    # NOTICE: `echo` will `queue_free()` the AnsiString automatically.
    echo(text)

    # Return values:
    # ExecuteStatus.DONE
    # ExecuteStatus.RUNNING
```

### Use in release export

- Ensure the plugin folder (addons/termdot) is in the same directory as your game executable;
- Alternatively, copy the files (shell.dll, winpty.dll, termdot.exe) from the plugin folder (addons/termdot) to the same directory as your game executable.

## Builtin Functions

| Key              | Function                                       |
| ---------------- | ---------------------------------------------- |
| ↑ / ↓            | History commands select.                       |
| Tab              | Commands list, auto completion.                |
| Control + C      | Interrupt current running command.             |
| Control + Insert | Copy selected text to clipboard from terminal. |
| Shift + Insert   | Paste text from clipboard to terminal.         |

| Command | Function                                                                                                                                                    |
| ------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------- |
| version | Show current Termdot version.                                                                                                                               |
| cls     | Clear entire screen.                                                                                                                                        |
| log     | Termdot will capture Godot's standard output and errors automatically, and also display logs recorded by Termdot.log(), Termdot.warn(), and Termdot.error() |

## Nodes Details

---

### Termdot

Main Godot node for plugin status management, and interactive with users.

#### Exported Fields

- **`host_name` (`String`)**:

  - **Description**: The host name displayed in the terminal prompt. It appears in the format `host_name> `, allowing users to customize the terminal's identity.
  - **Default Value**: `"termdot"`
  - **Usage**: Customize the name that appears in the terminal prompt to suit your project or plugin’s theme.

- **`command_ticks_per_second` (`int`)**:

  - **Description**: Controls the frequency at which commands are executed, measured in ticks per second.
  - **Range**: 1 to 60 (inclusive)
  - **Default Value**: `60`
  - **Usage**: Adjust this setting to control how often commands are processed, providing flexibility for more or less frequent updates.

- **`auto_output_captures` (`bool`)**:
  - **Description**: Capture Godot's standard output and errors automatically or not.
  - **Default Value**: `true`
  - **Usage**: If true, Termdot will capture Godot's standard output and errors automatically with timestamp.

These fields allow for a high degree of customization in how the plugin behaves within your Godot project, enabling tailored interaction with the external terminal.

#### Functions

- **`info(log: String)`**:

  - **static function**.
  - **Description**: Logs an informational message at the `INFO` log level. This log message is displayed when the internal `log` command is executed.
  - **Usage**: Call this method to log general informational messages.

- **`warn(log: String)`**:

  - **static function**.
  - **Description**: Logs a warning message at the `WARN` log level. This log message is displayed when the internal `log` command is executed.
  - **Usage**: Call this method to log warnings that require attention but are not critical.

- **`error(log: String)`**:
  - **static function**.
  - **Description**: Logs an error message at the `ERROR` log level. This log message is displayed when the internal `log` command is executed.
  - **Usage**: Call this method to log errors or critical issues that need immediate attention.

---

### Command

- **`command_name` (`String`)**:
  - **Description**: The name of the command being registered. If the name matches an internal command, it will be ignored. If the name matches a previous command, the previous one will be overwritten.
  - **Usage**: Set the command's name to customize how the command is identified and handled by the system.

#### Functions

- **`_start(params: Array[String]) -> int`**:

  - **Description**: This method is called when the command is detected. It processes the command by trimming spaces, and the parameters are passed as the `params` argument.
  - **Return Value**: `ExecuteStatus.DONE` or `ExecuteStatus.RUNNING`
  - **Usage**: Override this method to define what happens when the command is executed. Return `DONE` when the execution is complete or `RUNNING` if it needs to keep running.

- **`_running() -> int`**:

  - **Description**: This method is called when the command is in the running state (i.e., after `_start()` returns `RUNNING`). It continues running until it returns `DONE`.
  - **Return Value**: `ExecuteStatus.DONE` or `ExecuteStatus.RUNNING`
  - **Usage**: Override this method to define the behavior of the command while it is running. Return `DONE` when the operation is complete.

- **`_interrupting()`**:

  - **Description**: This method is executed when an interrupt signal (e.g., `Control+C`) is received. By default, it does nothing.
  - **Usage**: Override this method if you want to handle interruptions during command execution.

- **`get_terminal_size() -> Vector2i`**:

  - **Description**: Retrieves the current terminal size as a `(cols, rows)` tuple.
  - **Usage**: Call this method to get the dimensions of the terminal window.

- **`get_cursor_position() -> Vector2i`**:

  - **Description**: Retrieves the current cursor position in the terminal as a `(cols, rows)` tuple. The origin point of the cursor is `(1, 1)`.
  - **Usage**: Call this method to get the cursor’s position in the terminal.

- **`echo(text: AnsiString)`**:
  - **Description**: Sends a text message to the terminal, effectively echoing it.
  - **Usage**: Call this method to send an `AnsiString` to the terminal for output.
  - **Notice** `echo` will `queue_free()` the AnsiString automatically.

These fields and functions provide essential functionality for managing commands within a terminal environment, enabling customization, execution, and interaction within the Godot engine.

---

### AnsiString

The `AnsiString` struct is designed for building styled text using [Ansi Escape Code Sequences](https://gist.github.com/Joezeo/ce688cf42636376650ead73266256336) for terminal applications. It allows you to style text with various attributes such as colors and text effects.

#### Functions

- **Text Styling**:
  - Change the **foreground** and **background** colors using the 256-color palette or RGB values.
  - Set or reset text **style modes** such as **bold**, **underline**, **italic**, **blinking**, and **strikethrough**.
- **Cursor Manipulation**:
  - **Move**, **save**, and **restore** the cursor position, allowing you to manipulate text positioning in the terminal dynamically.

This class provides flexible control over text styling and cursor management, ideal for creating interactive terminal applications with colorful, styled text.

---

### Color256

The `Color256` class defines a set of constants representing standard basic colors commonly used in terminal applications. These constants correspond to the 16 basic color indices in the terminal’s 256-color palette.

#### Constants

- **Basic Colors**: Defines the standard terminal colors such as black, red, green, yellow, blue, magenta, cyan, and white.
- **Bright Colors**: Defines the bright variants of the standard colors, such as bright black, bright red, bright green, etc.

These constants provide a simple way to reference and use terminal colors in your application, making it easier to manage text styling and color formatting.

---

## License

[MIT license](LICENSE) © Joe Zane
