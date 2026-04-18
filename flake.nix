{
  description = "Nix packaging for httpcat";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
        httpcat = pkgs.callPackage ./default.nix { };
      in
      {
        packages = {
          default = httpcat;
          httpcat = httpcat;
        };

        apps.default = flake-utils.lib.mkApp {
          drv = httpcat;
        };
      });
}
