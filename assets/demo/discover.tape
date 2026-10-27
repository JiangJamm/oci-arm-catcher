# Setup GIF: `discover` prints ready-to-paste .env lines.
# Record from the repo root after `cargo build --release`:  vhs assets/demo/discover.tape

Output assets/discover.gif

Set Shell bash
Set FontSize 17
Set Width 1360
Set Height 380
Set Padding 28
Set WindowBar Colorful
Set WindowBarSize 44
Set TypingSpeed 55ms
Set Theme { "name": "deeplab", "background": "#0b0f14", "foreground": "#d7dce2", "cursor": "#d7dce2", "selection": "#2a3340", "black": "#1b2129", "red": "#f0524f", "green": "#3fb950", "yellow": "#d29922", "blue": "#58a6ff", "magenta": "#bc8cff", "cyan": "#39c5cf", "white": "#b1bac4", "brightBlack": "#6e7681", "brightRed": "#ff7b72", "brightGreen": "#56d364", "brightYellow": "#e3b341", "brightBlue": "#79c0ff", "brightMagenta": "#d2a8ff", "brightCyan": "#56d4dd", "brightWhite": "#f0f6fc" }

Hide
Type "source assets/demo/setup.sh && rm .env" Enter
Sleep 1.5s
Show

Sleep 500ms
Type "oci-arm-catcher discover" Sleep 300ms Enter
Sleep 4s
