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

      makeRustToolchain = pkgs:
        pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;

      # Compile using the pinned Rust toolchain. This prevents CI from
      # installing two versions of Rust.
      buildPinnedRustPackage = pkgs:
        let toolchain = makeRustToolchain pkgs;
        in lib.getAttr "buildRustPackage" (pkgs.makeRustPlatform {
          cargo = toolchain;
          rustc = toolchain;
        });

    in {
      overlays = rec {
        default = programs;

        # Add `clapfile` to nixpkgs.
        programs =
          (_: super: { clapfile = self.packages.${super.system}.clapfile; });
      };

      nixosModules = rec {
        default = nixos;

        # Generate a CLI using the NixOS module system.
        clapfile = import ./nix/bare-module.nix;

        # Mount `clapfile` into the NixOS module hierarchy.
        nixos = import ./nix/nixos-module.nix;
      };

      packages = eachSystem (system: pkgs: {
        clapfile = buildPinnedRustPackage pkgs {
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

          meta = {
            description = "A declarative CLI generator";
            homepage = "https://github.com/PsychoLlama/clapfile";
            license = lib.licenses.mit;
          };

          # Generate a `clapfile` executable. Suitable for nix shells.
          passthru.command = config:
            let
              root = lib.evalModules {
                modules = [ ./nix/bare-module.nix config ];
                specialArgs.pkgs = pkgs;
              };

            in root.config.program;
        };
      });

      devShell = eachSystem (system: pkgs:
        pkgs.mkShell {
          packages = [
            (makeRustToolchain pkgs)
            (pkgs.clapfile)
            (pkgs.clapfile.command ({
              command = pkgs.lib.pipe ./clapfile.toml [
                builtins.readFile
                builtins.fromTOML
              ];
            }))
          ];
        });
    };
}
