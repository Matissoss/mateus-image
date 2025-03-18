# Configuration on Windows

I don't officially support config on **Windows** as it has non-XDG-compatible paths. Only way to configure is with `--colors`/`-c` flag/s.

# Configuration On Linux

Configuration on Linux on the other hand is easy as it has XDG-compatible paths. Config can be ussually found in `.config/mateus-image` directory. Create `conf.ini` file.

Example **conf.ini**

```
# Comment
# 
# colors = #FFFFFF, #000000
# colors_path = colorscheme.csv
```

- `colors` define colors in `csv` format
- `colors_path` define where color scheme is stored it MUST BE IN `.config/mateus-image` DIRECTORY, otherwise it won't work

>![WARNING]
> Due to some bug in `src/config.rs`, if you use `colors_path` file must end with `,` at the end. This will be fixed soon
