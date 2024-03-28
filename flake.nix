{
  description = "Development environment";

  inputs.rust-overlay = {
    url = "github:oxalica/rust-overlay";
    inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs = { self, nixpkgs, rust-overlay }:
    let
      inherit (nixpkgs) lib;

      overlays = [ (import rust-overlay) ];

      systems =
        [ "aarch64-linux" "aarch64-darwin" "x86_64-darwin" "x86_64-linux" ];

      eachSystem = lib.flip lib.mapAttrs (lib.genAttrs systems
        (system: import nixpkgs { inherit system overlays; }));

    in {
      packages = eachSystem (system: pkgs: {
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
        };
      });

      devShell = eachSystem (system: pkgs:
        pkgs.mkShell {
          packages =
            [ (pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml) ];
        });
    };
}
