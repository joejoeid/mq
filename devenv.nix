{
  pkgs,
  lib,
  config,
  inputs,
  ...
}:

{
  packages = [
    pkgs.pkg-config
    pkgs.notmuch
  ];

  languages.rust.enable = true;

  enterShell = ''
    echo "Development environment loaded."
  '';
}
