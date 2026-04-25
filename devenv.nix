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
  git-hooks.hooks = {
    rustfmt.enable = true;
    clippy.enable = true;
  };
}
