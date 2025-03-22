# Configuration

## Linux/UNIX-like

To configure `mateus-image` on Linux there are 2 methods: through flags (temporary) or through config file.

Config file can be found in following directory: `~/.config/mateus-image` (uses `std::env::home_dir()`) named `conf.ini`

**Example conf.ini file**
```
; Comment
;   colors = [CSV_STRING]   : overrides default colorscheme
;   colors_path = [PATH]    : overrides default colorscheme with file found in: `~/.config/mateus-image`.
colors_path = colors.csv
```

**Example colors.csv file**
```
#000000,#00FF00
```

## Windows

There are no plans for officially supporting Windows through `conf.ini` file, because Windows isn't compatible with XDG paths.
Only way is through flags.
