{
  description = "video-downloader dev shell";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
        mingw = pkgs.pkgsCross.mingwW64;
      in {
        devShells.default = pkgs.mkShell {
          packages = with pkgs; [
            rustup
            pkg-config
            openssl

            # Windows GNU cross toolchain:
            mingw.buildPackages.gcc
            mingw.windows.mingw_w64_pthreads
          ];

          shellHook = ''
            export RUST_BACKTRACE=1
            echo "Rust dev shell is ready"
            echo "Linux build:   cargo build"
            echo "Windows build: cargo build --target x86_64-pc-windows-gnu"
          '';
        };
      });
}
