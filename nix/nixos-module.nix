{ lib, pkgs, config, ... }:

{
  # Ensure this is being used on a NixOS system.
  _class = "nixos";

  options.clapfile = lib.mkOption {
    description = "Create a command line app declaratively";

    type = lib.types.submoduleWith {
      specialArgs.pkgs = pkgs;
      modules = [
        (import ./bare-module.nix)
        { options.enable = lib.mkEnableOption "Add to system packages"; }
      ];
    };
  };

  config = lib.mkIf config.clapfile.enable {
    environment.systemPackages = [ config.clapfile.program ];
  };
}
