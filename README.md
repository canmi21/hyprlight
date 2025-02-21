# hyprlight

**hyprlight** is a brightness control utility for Hyprland, allowing you to adjust the screen brightness via the command line. You can easily integrate it into your Hyprland setup to adjust brightness on the fly.

## Installation

You can install **hyprlight** from the AUR (Arch User Repository) using `yay` or any other AUR helper:

```bash
yay -S hyprlight
```

Alternatively, you can manually clone the AUR repository and build it:

```bash
git clone https://aur.archlinux.org/hyprlight.git
cd hyprlight
makepkg -si
```

## Dependencies

- `glibc`
- `brightnessctl`
- `notify-daemon`
- `cargo` (for building the package)

## Configuration

Once installed, you can configure **hyprlight** by adding it to your Hyprland configuration or using it directly from the terminal. To start **hyprlight** automatically when your session starts, you can add the following line to your `hyprland.conf`:

```ini
exec-once = hyprlight
```

## Usage

Once running, **hyprlight** allows you to control the brightness of your screen. You can use the following commands:

```bash
Usage: hyprlight [OPTIONS] [STEP] <COMMAND>

Commands:
  i        Increase brightness
  d        Decrease brightness
  help     Print this message or the help of the given subcommand(s)

Arguments:
  [STEP]   [default: 5]    The brightness adjustment step

Options:
  -n, --notify     Enable notifications
  -h, --help       Print help
  -V, --version    Print version

Example:
  hyprlight i 10    # Increase brightness by 10%
  hyprlight d       # Decrease brightness by default step (5%)
  hyprlight i 5 -n  # Increase brightness by 5% with notification
```

### Notes on Notifications:

- **hyprlight** can show notifications about brightness changes.
- If you notice that your notifications lack icons, you'll need to install **HyprDE** for full notification support. HyprDE provides the necessary icons for notifications. You can find more information on how to set it up here: [HyprDE GitHub Repository](https://github.com/HyDE-Project/HyDE).

## Links

- [AUR Package](https://aur.archlinux.org/packages/hyprlight)
- [GitHub Repository](https://github.com/canmi21/hyprlight)
