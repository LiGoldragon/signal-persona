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
            cargoLock = {
              lockFile = ./Cargo.lock;
              outputHashes = {
                "nota-codec-0.1.0" = "sha256-8VwneAUq1+kur+o8uuvV8lxz8p3alFuT3EYMJsaQznc=";
                "nota-derive-0.1.0" = "sha256-se8zZsYzYlIJr75Q+i88k0EfUkRA/cEFafozBKfmlHY=";
                "signal-core-0.1.0" = "sha256-OiaNRHSkCf8tbTn50q10HVjMdfZ3BaJAgQUHIdYp2BQ=";
              };
            };
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
