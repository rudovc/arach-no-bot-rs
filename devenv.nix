{pkgs, ...}: {
  languages.rust = {
    enable = true;
    channel = "nightly";
    mold.enable = true;
  };

  env.LDFLAGS = "";

  packages = [
    pkgs.gitleaks
    pkgs.bacon
    pkgs.cargo-deny
    pkgs.cargo-license
    pkgs.cargo-make
    pkgs.zstd
  ];
}
