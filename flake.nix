{
  inputs = {
    naersk.url = "github:nix-community/naersk/master";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, utils, naersk }:
    utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
        naersk-lib = pkgs.callPackage naersk { };
      in
      {
        defaultPackage = naersk-lib.buildPackage ./.;
        devShell = let
					buildInputs = with pkgs; [
						expat
						fontconfig
						freetype
						freetype.dev
						libGL
						pkg-config
						wayland
						libxkbcommon
						# for xorg X11
						# xorg.libX11
						# xorg.libXcursor
						# xorg.libXi
						# xorg.libXrandr
					];
				in with pkgs; mkShell  {
					inherit buildInputs;
          nativeBuildInputs = [ cargo rustc rustfmt pre-commit rustPackages.clippy rust-analyzer ];

          RUST_SRC_PATH = rustPlatform.rustLibSrc;

					LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath buildInputs;
        };
      }
    );
}
