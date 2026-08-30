{ pkgs, ... }:

{
  languages.rust.enable = true;

  packages = [
    pkgs.cargo-deny
    pkgs.cargo-license
  ];
}
