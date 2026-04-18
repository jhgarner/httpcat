{ pkgs ? import <nixpkgs> { } }:

pkgs.rustPlatform.buildRustPackage {
  pname = "httpcat";
  version = "0.1.0";

  src = pkgs.nix-gitignore.gitignoreSource [ ] ./.;

  cargoLock = {
    lockFile = ./Cargo.lock;
  };

  meta = with pkgs.lib; {
    description = "Move data between a web browser and stdin/stdout";
    homepage = "https://github.com/jhgarner/httpcat";
    license = licenses.gpl3Only;
    mainProgram = "httpcat";
    platforms = platforms.unix;
  };
}
