{
  description = "Persona binary wire contract.";

  inputs = {
    nixpkgs.url = "github:LiGoldragon/nixpkgs?ref=main";
  };

  outputs =
    { self, nixpkgs }:
    let
      systems = [ "x86_64-linux" "aarch64-linux" ];
      forSystems = function: nixpkgs.lib.genAttrs systems (system: function system nixpkgs.legacyPackages.${system});
    in
    {
      packages = forSystems (
        system: pkgs:
        {
          default = pkgs.rustPlatform.buildRustPackage {
            pname = "signal-persona";
            version = "0.1.0";
            src = ./.;
            cargoLock.lockFile = ./Cargo.lock;
          };
        }
      );

      checks = forSystems (
        system: pkgs:
        {
          default = self.packages.${system}.default;
        }
      );

      devShells = forSystems (
        system: pkgs:
        {
          default = pkgs.mkShell {
            packages = [
              pkgs.cargo
              pkgs.clippy
              pkgs.rust-analyzer
              pkgs.rustc
              pkgs.rustfmt
            ];
          };
        }
      );

      formatter = forSystems (system: pkgs: pkgs.nixfmt);
    };
}

