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
        programs =
          (_: super: { clapfile = self.packages.${super.system}.clapfile; });
      };

      packages = eachSystem (system: pkgs: rec {
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

          passthru.command = config:
            let
              options =
                import ./nix/make-command.nix { inherit lib pkgs config; };

            in pkgs.stdenvNoCC.mkDerivation {
              pname = options.name;
              version = options.command.version or "latest";
              buildInputs = [ clapfile pkgs.makeWrapper ];
              phases = [ "buildPhase" ];
              buildPhase = ''
                mkdir -p "$out/bin"
                install -Dm755 "$(command -v clapfile)" "$out/bin/${options.name}"

                # Generate shell completions
                function gen_completions {
                  clapfile completions --config ${options.args.config} "$1"
                }

                mkdir -p "$out/share/bash-completion/completions"
                mkdir -p "$out/share/fish/vendor_completions.d"
                mkdir -p "$out/share/zsh/site-functions"

                gen_completions bash > "$out/share/bash-completion/completions/${options.name}"
                gen_completions fish > "$out/share/fish/vendor_completions.d/${options.name}.fish"
                gen_completions zsh >  "$out/share/zsh/site-functions/_${options.name}"

                wrapProgram "$out/bin/${options.name}" --add-flags "run ${
                  lib.cli.toGNUCommandLineShell { } options.args
                } --"
              '';
            };
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
