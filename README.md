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

# Configuration
In the config file every line represents an event. The lines have to be formatted as one of the following: 
1. ``%:z %Y "m" %b %d	%R <Label>	[<Duration>]`` or 	(e.g. ``+02:00 2021 m Jun 27	12:00 Example1	m10``)
2. ``%:z %Y "w" %W %a	%R <Label>	[<Duration>]``		(e.g. ``+02:00 2021 w	05 Mon	21:00 Example2	h1``)
3. Lines starting with ``#`` as well as empty lines are ingnored in the parsing of events.

|``%`` specifier|Example|Description|Wildcard options|
|---------------|-------|-----------|----------------|
|``%:z``|``+02+00``, ``+00:00``|Time zone offset from UTC (+00:00 is UTC)|Wildcards are not allowed for the timezone|
|``%Y``|``2020``, ``2021``, ``*``|The full proleptic Gregorian year, zero-padded to 4 digits|Can be ``*`` -> every year|
|``%b``|``Jan``, ``Dec``, ``*``|Abbreviated month name. 3 letters.|Can be ``*`` -> every month|
|``%d``|``01``, ``24``, ``*``|Day number (01--31), zero-padded to 2 digits.| Can be ``*`` -> every day of the month|
|``%W``|``01``, ``15``, ``*``|Calendar week (week starting with monday)|Can be ``*`` -> every week of the year|
|``%a``|``Mon``, ``Fri``, ``*``|Abbreviated weekday name. 3 letters.|Can be ``*`` -> every day of the week|
|``%R``|``03:00``, ``17:30``, ``*:30``, ``12:*``|24 hour-minute format|Hour part can be ``*`` -> every hour of the day at given minute,<br/>Minute part can be ``*`` -> every minute of the hour,<br/>Both can be ``*`` (``*:*``) -> every minute of the day|

Durations are provided as follows ``<Fmt><Num>`` (e.g. s10 -> 10 seconds) where <Fmt> is one of ``s, m, h, d`` and num any positive integer (32 bit uint)

|Fmt|Meaning|
|---|-------|
|s|Seconds|
|m|Minutes|
|h|Hours|
|d|Days|

All parts are seperated by any ammount/type of whitespace you want to use -> you can format your config file to look organized using tabs and spaces.

You decide whether the date you put in is parsed in 1. "month_mode" or 2. "week_mode" by setting the 3rd parameter either to 1.``m`` or 2.``w`` which changes the parsing pattern as shown above.

You may combine any number of wildcard options with eachother.
## Examples
- ``+02:00 2021 m Jun * 19:00 daily`` -> every day of June at 19:00 in 2021 (no duration defaults to 0 seconds)
- ``+02:00 * m Jun * 19:00 daily m10`` -> every day of June at 19:00 each year (for 10 minutes)
- ``+02:00 * m * * 19:00 daily h3`` -> every day at 19:00 of every month each year (for 3h)

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

