{ pkgs, clapfile, ... }:

let
  inherit (import (pkgs.path + "/nixos/lib") { }) runTest;
  toml = pkgs.formats.toml { };
in
runTest {
  hostPkgs = pkgs;
  name = "end-to-end-test-set";

  nodes.machine = {
    imports = [ clapfile.nixosModules.nixos ];
    nixpkgs.overlays = pkgs.overlays;

    clapfile = {
      enable = true;
      command = {
        name = "program";
        run = pkgs.writers.writeNu "program" ''
          1..10 | each { $in + 1 } | first
        '';
      };
    };

    environment = {
      systemPackages = [ pkgs.clapfile ];
      etc = {
        "clapfile/basic.toml".source = toml.generate "basic-example.toml" { run = "echo 'hello-world'"; };

        "clapfile/empty.toml".source = toml.generate "empty-example.toml" {
          name = "empty";
          about = "some description";
        };

        "clapfile/simple-args.toml".source = toml.generate "simple-args" {
          name = "simple-args";
          run = "echo $positional $named";
          args = [
            { id = "positional"; }
            {
              id = "named";
              long = "foo";
            }
          ];
        };

        "clapfile/subcommands.toml".source = toml.generate "subcommands" {
          name = "subcommands";
          subcommands.foo = {
            name = "foo";
            run = "echo 'foo'";
          };
        };
      };
    };
  };

  testScript = ''
    start_all()

    with subtest("Basic example"):
      out = machine.succeed("clapfile run -c /etc/clapfile/basic.toml")
      # assert out == "hello-world\n"

    with subtest("Print help if no command specified"):
      out = machine.fail("clapfile run -c /etc/clapfile/empty.toml")
      assert "Usage: empty" in out, "Expected usage message to be printed"

    with subtest("Simple args"):
      out = machine.succeed("clapfile run -c /etc/clapfile/simple-args.toml -- foo --foo bar")
      assert out == "foo bar\n", "Expected 'foo bar' to be printed"

    with subtest("Subcommands"):
      out = machine.succeed("clapfile run -c /etc/clapfile/subcommands.toml -- foo")
      assert out == "foo\n", "Expected 'foo' to be printed"

    with subtest("Provisioned command"):
      out = machine.succeed("program")
      assert out == "2\n", "Nushell script did not print expected output"
  '';
}
