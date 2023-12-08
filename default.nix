{ stdenv, pkg-config, SDL2, SDL2_mixer, elfkickers, valgrind, kdeApplications }:

stdenv.mkDerivation rec {
  pname = "ksp2d";
  version = "0.1";

  src = ./.;
  nativeBuildInputs = [ pkg-config elfkickers valgrind kdeApplications.kcachegrind];
  buildInputs = [ SDL2 SDL2_mixer ];

  installPhase = ''
    mkdir -p $out/bin
    cp ksp2d $out/bin
  '';

  meta = with stdenv.lib; {
    description = "desc";
    longDescription = ''
      long desc
    '';
    homepage = "";
    changelog = "";
    license = licenses.mit;
    maintainers = [ "aminion@protonmail.com" ];
    platforms = platforms.all;
  };
}