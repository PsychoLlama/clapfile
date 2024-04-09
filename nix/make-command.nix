{ lib, pkgs, config }:

lib.getAttr "config" (lib.evalModules {
  modules = [ ./module.nix config ];
  specialArgs.pkgs = pkgs;
})
