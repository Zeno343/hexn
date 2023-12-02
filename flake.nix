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
    crane = (craneLib.mkLib pkgs).overrideToolchain pkgs.rust-bin.nightly.latest.default;
    pkgs = import nixpkgs {
      inherit system;
      overlays = [ (import rust-overlay) ];
    };

    hexen = let
      src = with pkgs; rec {
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
    in rec {
      deps = crane.buildDepsOnly src;

      release = crane.buildPackage (src // {
        cargoArtifacts = deps;
      });

      dev = crane.devShell (src // {
        cargoArtifacts = deps;
        inputsFrom = [ src ];
        packages = [ smartRelease ];
      });
    };
  in {
    packages.${system} = {
      default = hexen.release;
      dev = hexen.dev;
    };

    devShells.${system}.default = hexen.dev;
  };
}
