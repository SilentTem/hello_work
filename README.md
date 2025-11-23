# Hello Work

## FAQ

### The UI elements are too large under X11

Either set `Xft.dpi: 96` in `~/.Xresources`, or set `WINIT_X11_SCALE_FACTOR=1`. See [this](https://github.com/rust-windowing/winit/issues/2231).
