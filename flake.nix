{
  description = "Development environment";

  inputs.rust-overlay = {
    url = "github:oxalica/rust-overlay";
    inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs = { self, nixpkgs, rust-overlay }:
    let
      inherit (nixpkgs) lib;

      overlays = [ (import rust-overlay) self.overlays.programs ];

      systems =
        [ "aarch64-linux" "aarch64-darwin" "x86_64-darwin" "x86_64-linux" ];

      eachSystem = lib.flip lib.mapAttrs (lib.genAttrs systems
        (system: import nixpkgs { inherit system overlays; }));

    in {
      overlays = rec {
        default = programs;
        programs =
          (_: super: { clapfile = self.packages.${super.system}.clapfile; });
      };

      packages = eachSystem (system: pkgs: rec {
        clapfile = pkgs.rustPlatform.buildRustPackage {
          pname = "clapfile";
          cargoLock.lockFile = ./Cargo.lock;

          version = pkgs.lib.pipe ./Cargo.toml [
            builtins.readFile
            builtins.fromTOML
            (manifest: manifest.package.version)
          ];

          src = let fs = lib.fileset;
          in fs.toSource {
            root = ./.;
            fileset = fs.unions [
              (fs.fileFilter (f: f.hasExt "rs") ./src)
              ./Cargo.lock
              ./Cargo.toml
            ];
          };

          passthru.wrapper = config:
            pkgs.stdenvNoCC.mkDerivation {
              name = config.name;
              buildInputs = [ clapfile pkgs.makeWrapper ];
              phases = [ "buildPhase" ];
              buildPhase = ''
                mkdir -p "$out/bin"
                install -Dm755 "$(command -v clapfile)" "$out/bin/${config.name}"

                # TODO: Generate shell completions.

                wrapProgram "$out/bin/${config.name}" --add-flags "run --config ${
                  (pkgs.formats.yaml { }).generate "${config.name}.yml" config
                } --"
              '';
            };
        };
      });

      devShell = eachSystem (system: pkgs:
        pkgs.mkShell {
          packages = [
            (pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml)
            (pkgs.clapfile)
            (pkgs.clapfile.wrapper (pkgs.lib.pipe ./clapfile.toml [
              builtins.readFile
              builtins.fromTOML
            ]))
          ];
        });
    };
}
