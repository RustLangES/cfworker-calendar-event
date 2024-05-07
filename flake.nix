{
  description = "Calendar worker";
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    fenix.url = "github:nix-community/fenix";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = {
    self,
    nixpkgs,
    flake-utils,
    ...
  } @ inputs:
    flake-utils.lib.eachSystem (flake-utils.lib.defaultSystems) (
      system: let
        calendarBundle = import ./. {
          inherit system;
          pkgs = nixpkgs.legacyPackages.${system};
          fenix = inputs.fenix.packages;
        };
      in {
        inherit (calendarBundle) devShells;
      }
    );
}
