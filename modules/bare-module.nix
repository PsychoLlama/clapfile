{
  lib,
  pkgs,
  config,
  ...
}:

with lib;

let
  toml = pkgs.formats.toml { };
in
{
  options = {
    package = mkPackageOption pkgs "clapfile" { };

    command = mkOption {
      type = toml.type;
      description = "Config defining the command line interface";
      default = { };
    };

    name = mkOption {
      type = types.str;
      description = "Name of the executable";
      default = config.command.name;
    };

    args = {
      shell = mkOption {
        type = types.str;
        default = "${pkgs.dash}/bin/dash";
        description = "The shell to use when running commands";
      };

      log-level = mkOption {
        type = types.nullOr types.str;
        description = "Clapfile logging verbosity level";
        default = null;
      };

      config = mkOption {
        type = types.path;
        description = "Path to the clapfile";
        default = toml.generate "${config.name}.toml" config.command;
      };
    };

    program = mkOption {
      type = types.package;
      description = "The generated executable";
      internal = true;
      readOnly = true;
      default = pkgs.stdenvNoCC.mkDerivation {
        pname = config.name;
        version = config.command.version or "latest";
        buildInputs = [
          config.package
          pkgs.makeWrapper
        ];
        phases = [ "buildPhase" ];
        buildPhase = ''
          mkdir -p "$out/bin"
          install -Dm755 "$(command -v clapfile)" "$out/bin/${config.name}"

          # Generate shell completions
          function gen_completions {
            clapfile completions --config ${config.args.config} "$1"
          }

          mkdir -p "$out/share/bash-completion/completions"
          mkdir -p "$out/share/fish/vendor_completions.d"
          mkdir -p "$out/share/zsh/site-functions"

          gen_completions bash > "$out/share/bash-completion/completions/${config.name}"
          gen_completions fish > "$out/share/fish/vendor_completions.d/${config.name}.fish"
          gen_completions zsh >  "$out/share/zsh/site-functions/_${config.name}"

          wrapProgram "$out/bin/${config.name}" --add-flags "run ${
            lib.cli.toGNUCommandLineShell { } config.args
          } --"
        '';
      };
    };
  };
}
