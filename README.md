# bar_calendar
A minimalistic calendar. It prints the most recent event in your list joined with the time difference from now (e.g. "Event in 20 min").
Intended for use in a bar.

# Installation
1. You need cargo. If you don't have it installed already you can do so by following the steps from [The Cargo Book](https://doc.rust-lang.org/cargo/getting-started/installation.html)
2. Run ``cargo install --git https://github.com/DEVensiv/bar_calendar --branch main`` to build the crate in a temporary target directory before installing the binaries in your cargo installation's ``bin`` folder. For more information check out the [cargo install](https://doc.rust-lang.org/cargo/commands/cargo-install.html) Book entry
3. Alternatively to 2. you can clone the repository move into ``bar_calendar/`` in order to run ``cargo install --path .``. This has the same effect as 2. but you are not building in a temporary target directory

# Setup
1. Create a config file in your systems config directory ``<your_config_dir>/bar_calendar/config.conf`` (you can use different file locations by giving the path as command line argument to the program)
2. Start listing your events in the config file from step 1.
3. Set up your bar to periodically run ``bar_calendar`` to update its block

## Examples
### [i3status-rust](https://github.com/greshake/i3status-rust)
Add following block to your config.toml
```
[[block]]
block = "custom"
command = "bar_calendar"
json = true
interval = 60
```

## Platform specific config directories
|Platform|Value|Example|
|--------|-----|-------|
|Linux|$XDG_CONFIG_HOME or $HOME/.config|/home/alice/.config|
|macOS|$HOME/Library/Application Support|/Users/Alice/Library/Application Support|
|Windows|{FOLDERID_RoamingAppData}|C:\Users\Alice\AppData\Roaming|

# Options
Usage: ``bar_calendar [path] [options]``

 ``--json``
    Provides output in json format (default) (e.g. ``{"icon": "calendar", "state": "Idle", "text": "Eventname in 20 min"})``)
    
 ``--no-json``
    Outputs only the event text part (e.g. ``Eventname in 20 min``)
 
 ``<path>``
    Path to an alternative config file

## Examples
  - ``bar_calendar ~/.config/bar_calendar/alternative_config.conf --no-json`` uses ``alternative.conf`` and outputs only the text part
  - ``bar_calendar --json`` outputs the most recent event in json format (this is the default)
  - ``bar_calendar --no-json`` outputs only the text part, uses the default config

# Uninstalling
In case you want to uninstall the bar_calendar you can
1. Run ``cargo uninstall bar_calendar`` to remove the program
2. In case you installed ``cargo`` (with rustup) just for this program and want to uninstall it too you can do so by running ``rustup self uninstall`` which will uninstall ``rustup`` and all its components.
