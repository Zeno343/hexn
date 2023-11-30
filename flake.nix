{
  description = "A demo engine in Rust";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";

    craneLib.url = "github:ipetkov/crane";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, craneLib, rust-overlay }:
  let 
    system = "x86_64-linux";
    pkgs = import nixpkgs {
      inherit system;
      overlays = [ (import rust-overlay) ];
    };

    crane = (craneLib.mkLib pkgs).overrideToolchain pkgs.rust-bin.nightly.latest.default;
    hexenSrc = with pkgs; rec {
      src = crane.path ./.;
      cargoTestExtraArgs = "--all-features";

      LIBCLANG_PATH = "${llvmPackages_15.libclang.lib}/lib";
      BINDGEN_EXTRA_CLANG_ARGS =
        (builtins.map (lib: ''-I"${lib.dev}/include"'') buildInputs);

      buildInputs = [
        SDL2
        libGL
      ];
      nativeBuildInputs = [
        pkg-config
        llvmPackages_15.libcxxClang
      ];
    };

    ci = let
      smartRelease = with pkgs; crane.buildPackage {
        src = fetchFromGitHub {
          owner = "Byron";
          repo = "cargo-smart-release";
	  rev = "82c39db";
	  sha256 = "sha256-3aS0/2A4O+IDpFXsCup3OlhlvWuiMf0M1USJqnjAR7w=";
        };

        buildInputs = [ openssl ];
        nativeBuildInputs = [ cmake pkg-config ];
      };
    in crane.buildDepsOnly (hexenSrc // {
      cargoArtifacts = [ hexenDeps smartRelease ];
    });


    hexenDeps = crane.buildDepsOnly hexenSrc;
    hexen = crane.buildPackage (hexenSrc // {
      cargoArtifacts = hexenDeps;
    });
  in {
    packages.${system} = {
      default = hexen;
      inherit ci;
    };

    devShells.${system}.default = crane.devShell (hexenSrc // {
      name = "hexen-ci";
      cargoArtifacts = ci;
    });
  };
}
