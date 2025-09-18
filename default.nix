let
  pkgs = import <nixpkgs> {};
in pkgs.rustPlatform.buildRustPackage {
  name = "words";
  src = ./.;
  cargoHash = "sha256-qdn68JAE4zEj9V49GMs9TNO1i2/hVNpvDNdFmiJmzcU=";
  nativeBuildInputs = [
    pkgs.pkg-config
  ];
  buildInputs = [
    pkgs.glib
    pkgs.poppler
  ];
  preBuild = let
    yake-rust = pkgs.fetchFromGitHub {
      owner = "quesurifn";
      repo = "yake-rust";
      rev = "92cee51b1d6f13cbab388a9c89accabb282d99e9";
      hash = "sha256-6da5HIQ2D/vfbQUE2RrrK5Eo7DBDxEuM4HrTIoNA2/s=";
      postFetch = /*bash*/ ''
        patch -p1 -d $out < ${./yake-rust.patch}
      '';
    };
  in /*bash*/ ''
    [[ -e yake-rust ]] || ln -s ${yake-rust} yake-rust
  '';
}
