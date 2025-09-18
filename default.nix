let
  pkgs = import <nixpkgs> {};
in pkgs.rustPlatform.buildRustPackage rec {
  name = "keywords";
  src = ./.;
  cargoHash = "sha256-op2bzNF5EWCzTykwANeWT3WDaadpknPvaLAq46Y88yA=";
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
      hash = "sha256-fMy3RoZiMkrywTWsw5eVd6yU/atZWuaP3kwyhwPPXDI=";
      postFetch = /*bash*/ ''
        patch -p1 -d $out < ${./0001-count-ascii.patch}
        patch -p1 -d $out < ${./0002-get_fn_best.patch}
      '';
    };
    stopwords = pkgs.runCommand "stopwords" {
      google10ken = pkgs.fetchurl {
        url = "https://github.com/first20hours/google-10000-english/raw/bdf4c221bc120b0b7f6c3f1eff1cc1abb975f8d8/google-10000-english-no-swears.txt";
        sha256 = "11pd0p6ckixr1b5qvi6qxj389wmzq1k42is1bm9fc2y3397y1cyn";
      };
      entxt = "${yake-rust}/yake_rust/src/stopwords/en.txt";
    } ''
      { cat $google10ken; cat $entxt; } | sort | uniq > $out
    '';
  in /*bash*/ ''
    [[ -e yake-rust ]] || ln -s ${yake-rust} yake-rust
    [[ -e ./src/stopwords.txt ]] || cp -f ${stopwords} ./src/stopwords.txt
  '';
  shellHook = preBuild;
}
