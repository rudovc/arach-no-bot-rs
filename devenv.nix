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
    pkgs.gdb
    pkgs.lldb
    pkgs.mold
    pkgs.zstd
  ] ++ pkgs.lib.optionals (pkgs.stdenv.hostPlatform.system == "x86_64-linux") [ pkgs.nnd ];
}
