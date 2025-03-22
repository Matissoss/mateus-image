<div align=center>
    <h1>mateus-image</h1>
</div>

---

# About Project

`mateus-image` is simple CLI program for image manipulation using [*filters*](src/filters). 
Originally it was made for colorscheme named ashen, but now it defaultly supports [my colorscheme](https://github.com/Matissoss/colorscheme)

# Examples

Example images changed by `mateus-image` can be found in [my other repo](https://github.com/Matissoss/mateus-wallpaper)

# Install

`mateus-image` is available through:
- **cargo** : `cargo install mateus-image`
- **precompiled bins** : download from `Releases` section
- **from source** : requires `cargo`, `python3` (optional) and `git` (optional) ; use [`build.py`](build.py) or `cargo build --release`

# Usage

```
# mateus-image help

mateus-image VERSION
===================
USAGE:
mateus-image [METHOD] [FLAGS]
-------------------
METHODs:
help
config	    - use help method along with config method to
	      get more info
median
stalinsort
mean
pixel
binary      -   chooses 2 most often colors and changes 
                rest to be one of them
ascii       -   returns image as string in printable ascii
		letters
monochrome  -	makes image monochrome
extras-1    -	makes image darker
-------------------
FLAGs:
--param/-p=[VALUE]	- 	param is used to customize filters;
				used by: 
				median, stalinsort, mean, pixel
				filters
--input/-i=[PATH]	-	specifies path to image that program
				should use
--output/-o=[PATH]	-	specifies path where output image should be 
				saved
--color/-c=[COLORS]	-	makes program use colors specified by this
				flag. Colors must be split by ',' like in .csv format
				and saved in hex format (ex. "#F5F5F5", "#000")
-------------------
made by matissoss <matissossgamedev@proton.me>
licensed under MIT
```

# Configuration

`mateus-image` supports configuration since version `v1.3.0` on systems that have `~/.config`, so on systems that aren't windows. Check [CONFIG.md](CONFIG.md) for more info.

# Credits

made by Matissoss [matissossgamedev@proton.me]

licensed under MIT License
