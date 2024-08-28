# To learn more about how to use Nix to configure your environment
# see: https://developers.google.com/idx/guides/customize-idx-env
{ pkgs, ... }: {
  channel = "stable-23.11";

  packages = [
    pkgs.rustup
    pkgs.clang
  ];

  # Sets environment variables in the workspace
  env = {};
  idx = {
    # Search for the extensions you want on https://open-vsx.org/ and use "publisher.id"
    extensions = [
      "rust-lang.rust-analyzer"
    ];

    previews = {
      enable = true;
      previews = {};
    };

    workspace = {
      onCreate = {};
      onStart = {};
    };
  };
}
