````md
# png-to-terminal

Render PNG images directly in the terminal using 24-bit TrueColor.

This tool draws real image pixels in the terminal using Unicode half-block characters (`▀`).
It is **not ASCII art**.

---

## Requirements

- A terminal with TrueColor (24-bit) support  
  - Windows Terminal  
  - WezTerm  
  - Kitty  
  - Alacritty  

> Classic PowerShell console is **not supported**.

---

## Install

```bash
cargo install png-to-terminal
````

---

## Usage

```bash
png-to-terminal image.png
```

Optional width:

```bash
png-to-terminal image.png 40
```

---

## How it works

* Each terminal character represents two vertical pixels
* Foreground color = top pixel
* Background color = bottom pixel
* Uses ANSI TrueColor escape sequences

---

## License

MIT

```
```
