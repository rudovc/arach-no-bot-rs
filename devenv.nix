{pkgs, ...}: {
  languages.rust.enable = true;

  packages = [
    pkgs.bacon
    pkgs.cargo-deny
    pkgs.cargo-license
    pkgs.cargo-make
  ];
}
