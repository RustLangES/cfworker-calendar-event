let
  inherit
    (builtins)
    currentSystem
    fromJSON
    readFile
    ;
  getFlake = name:
    with (fromJSON (readFile ./flake.lock)).nodes.${name}.locked; {
      inherit rev;
      outPath = fetchTarball {
        url = "https://github.com/${owner}/${repo}/archive/${rev}.tar.gz";
        sha256 = narHash;
      };
    };
in
  {
    system ? currentSystem,
    pkgs ? import (getFlake "nixpkgs") {localSystem = {inherit system;};},
    lib ? pkgs.lib,
    fenix,
    ...
  }: let
    # fenix: rustup replacement for reproducible builds
    toolchain = fenix.${system}.fromToolchainFile {
      file = ./rust-toolchain.toml;
      sha256 = "sha256-6eN/GKzjVSjEhGO9FhWObkRFaE1Jf+uqMSdQnb8lcB4=";
    };

    # buildInputs for Examples
    buildInputs = with pkgs; [
      openssl
    ];

  in {
    # `nix develop`
    devShells.default = pkgs.mkShell {
      packages = with pkgs;
        [
          toolchain
          pkg-config
          worker-build
          nodePackages.wrangler
          binaryen
        ]
        ++ buildInputs;
      LD_LIBRARY_PATH = lib.makeLibraryPath buildInputs;
    };
  }
