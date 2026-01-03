# Clapfile

Turn config files into command line apps

## Project Status

:no_entry: **Abandoned**

Compilation time and effort to add new features wasn't worth the overhead. I switched to [just](https://github.com/casey/just) and [nuenv](https://github.com/nushell/nu_scripts/blob/1cb6d6c460949b989b7fb1a6d02456a560521366/nu-hooks/nu-hooks/nuenv/hook.nu).

## Purpose

Managed systems usually have maintenance scripts that run manually, like initializing a ZFS pool or unlocking a Vault server. These scripts are written in Python, Bash, Perl, or whatever language was convenient at the time.

Over time scripts become less discoverable. They are scattered across the filesystem, have different interfaces, and different levels of `--help`.

Clapfile unifies scripts into one interface. You define subcommands, the script to execute, the options it accepts, and clapfile wraps them into in a single CLI.

This gets you some advantages:

- **Discoverability**: All management scripts are in one place and discoverable through `--help`.
- **Shell completions**: Generated completions are available for popular shells.
- **Argument validation**: Required arguments are checked before running the script.

## Usage

Create a `clapfile.toml` in your project. This defines the available scripts.

> [!TIP]
> The config file mirrors the [clap](https://clap.rs/) library. If `clap::Command` has a method `.about(h: String)`, then the TOML key is `about = "string"`. However, not all options are supported.

```toml
name = "system"
about = "System administration tasks"

[subcommands.greet]
about = "Says hello"
run = 'echo "Hello, $name"'

args = [
  { id = "name", required = true }
]
```

Then run it with `clapfile`:

```bash
clapfile run --config clapfile.toml -- greet world
# => "Hello, world"
```

The `run` key is a shell command to execute. Any arguments defined are passed through as environment variables.

This is just to get started quickly. The Nix wrapper has all the bells and whistles. It's the recommended way to use clapfile.

## Installation

This package is only available as a Nix flake. To quickly try it, run:

```bash
nix run 'github:PsychoLlama/clapfile'
```

Or programmatically by adding it to your `flake.nix`:

```nix
{
  inputs.clapfile.url = "github:PsychoLlama/clapfile";

  outputs = { self, clapfile }: {
    devShell = eachSystem (system: pkgs: pkgs.mkShell {
      packages = [
        (clapfile.packages.${system}.clapfile.command {
          args.shell = "${pkgs.bash}/bin/bash";
          command = {
            name = "example";
            run = "echo 'Hello, world!'";
          };
        })
      ];
    });
  };
}
```

### Usage with Nix

The project flake exports a `clapfile` derivation. You can add this to a dev shell, or use the `clapfile.command {...}` builder to construct a wrapper program:

```nix
project = clapfile.command {
  args = {
    # Optional: extra options passed to `clapfile run`
  };

  command = {
    name = "project";
    about = "Project task runner";
    subcommands = {
      lint = {
        about = "Lint the project";
        run = pkgs.writers.writeBash "lint-project" ''
          echo "Linting..."
        '';
      };

      test = {
        about = "Run the test suite";
        run = pkgs.writers.writePython3 "run-tests" { } ''
          print("Running tests...")
        '';
      };
    };
  };
}
```

The result is a `project` command you can add to your dev shell or `environment.systemPackages`.

```bash
project --help
```

```
Project task runner

Usage: project [COMMAND]

Commands:
  test  Run the test suite
  lint  Lint the project
  help  Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```

### Usage with NixOS

There is a NixOS module that works similar to the Nix example above.

```nix
{
  # Provides `config.clapfile`.
  imports = [ flake-inputs.clapfile.nixosModules.nixos ];

  # Ensures `pkgs.clapfile` is available.
  nixpkgs.overlays = [ flake-inputs.clapfile.overlays.programs ];

  clapfile = {
    enable = true; # Add the generated program to `environment.systemPackages`.
    args = {
      # Optional command line args passed to `clapfile run`
    };

    command = {
      name = "greet";
      about = "Says hello";
      run = "echo 'Hello, world!'";
    };
  };
}
```

```bash
greet
# => "Hello, world!"
```
