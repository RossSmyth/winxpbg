# winxpbg

Windows XP as your desktop background.

This embeds a GTK-based webview to use https://winxp.vercel.app/ as your desktop background.

Only works on Wayland with gtk-layer-shell.

## Nix

1. Import the repo as an overlay:

```nix
let
  nixpkgs = import nixpkgs {
    inherit system;
    overlays = [
      (import winxpbg)
    ];
  };
in
...
```
2. Launch it with your desktop environment or window manager, for example with Niri:

```nix
{
  programs.niri.settings.spawn-at-startup = [
    { command = [ (lib.getExe pkgs.winxpbg) ]; }
  ];
}
```
