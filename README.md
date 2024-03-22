# Clapfile

Command line apps from config files

## Purpose

`clapfile` turns a set of scripts into a single command line app.

It's designed for declarative environments (Ansible, NixOS) that provision management scripts in several languages (shell, Python, Ruby, etc) and benefit from a single harness to run them.

The advantage is shell completions, help messages, discoverability, and a unified way to manage options and arguments.
