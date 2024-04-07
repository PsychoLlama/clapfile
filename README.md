# Clapfile

Turn config files into command line apps

## Project Status

:scientist: **Experimental**

I wrote this for [my home lab](https://github.com/PsychoLlama/home-lab/) (a managed NixOS cluster). It works, it's useful, but I'm still playing with the concept. I might abandon it later.

Documentation is kind of thrown together.

## Purpose

Managed systems usually have maintenance scripts that run manually, like initializing a ZFS pool or updating a database schema. These scripts are written in Python, Bash, Perl, or whatever language was convenient at the time.

Over time scripts become less discoverable. They are scattered across the filesystem, have different interfaces, and different levels of `--help`.

Clapfile is like a task runner. It combines them under a single interface. You define subcommands, the script to execute, the options it accepts, and clapfile wraps them into in a single command.

This buys you some advantages:

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

This is just to get started quickly. The Nix wrapper has all the bells and whistles. It's the "recommended" way to use clapfile.

```nix
clapfile.command {
    # Optional: extra options passed to `clapfile run`
} {
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
}
```

This provisions a `project` command you can add to your dev shell or `environment.systemPackages`.

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

## Installation

This is only available as a Nix flake for now.

```nix
{
  inputs.clapfile.url = "github:PsychoLlama/clapfile";

  outputs = { self, clapfile }: {
    devShell = eachSystem (system: pkgs: pkgs.mkShell {
      packages = [
        (clapfile.packages.${system}.clapfile.command { } {
          name = "example";
        })
      ];
    });
  };
}
```
