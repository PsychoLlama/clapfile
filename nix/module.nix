{ lib, pkgs, config, ... }:

with lib;

let toml = pkgs.formats.toml { };

in {
  options = {
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
  };
}
