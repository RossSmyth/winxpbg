{ pkgs ? import <nixpkgs> { } }: pkgs.mkShell {
  nativeBuildInputs = [ pkgs.pkg-config ];
  packages = [
    pkgs.gtk4
    pkgs.gtk4-layer-shell
    pkgs.atk
    pkgs.glib
    pkgs.gdk-pixbuf
    pkgs.libadwaita
    pkgs.webkitgtk_6_0

    pkgs.gst_all_1.gstreamer
    pkgs.gst_all_1.gst-plugins-base
    pkgs.gst_all_1.gst-plugins-good
    pkgs.gst_all_1.gst-plugins-bad
  ];
}
