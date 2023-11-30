{
  description = "A demo engine in Rust";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";

    crane.url = "github:ipetkov/crane";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, crane, rust-overlay }:
  let 
    system = "x86_64-linux";
    pkgs = import nixpkgs {
      inherit system;
      overlays = [ (import rust-overlay) ];
    };

    craneLib = (crane.mkLib pkgs).overrideToolchain pkgs.rust-bin.nightly.latest.default;

    smartRelease = with pkgs; craneLib.buildPackage {
      src = fetchFromGitHub {
        owner = "Byron";
        repo = "cargo-smart-release";
	rev = "82c39db";
	sha256 = "sha256-3aS0/2A4O+IDpFXsCup3OlhlvWuiMf0M1USJqnjAR7w=";
      };

      buildInputs = [ openssl ];
      nativeBuildInputs = [ cmake pkg-config ];
    };

    hexen = with pkgs; craneLib.buildPackage rec {
      pname = "hexen";
      version = "v0.0.1";
      src = craneLib.path ./.;
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
        smartRelease
      ];
    };
  in {
    packages.${system}.default = hexen;

    devShells.${system}.default = craneLib.devShell {
      LIBCLANG_PATH = "${pkgs.llvmPackages_15.libclang.lib}/lib";
      name = "hexen";
      inputsFrom = [ hexen ];
    };
  };
}
