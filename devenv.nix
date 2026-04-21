{
  pkgs,
  lib,
  config,
  inputs,
  ...
}:

{
  packages = [ ];

  languages.rust.enable = true;

  git-hooks.hooks.clippy.enable = true;
}
