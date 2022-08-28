{ lib, fetchFromGitHub, rustPlatform, pkg-config, openssl }:

rustPlatform.buildRustPackage rec {
  pname = "dnsbot";
  version = "0.1.0";

  src = ./.;

  cargoSha256 = "sha256-bxVmp9HJ5Q07dALUjjHIcstEmzPGyfwGmypK+dMVLP0=";

  nativeBuildInputs = [ pkg-config ];
  buildInputs = [ openssl ];

  meta = with lib; {
    description = "DNS resolver in matrix";
    homepage = "https://git.frsqr.xyz/cofob/dnsbot";
    license = licenses.gpl3;
  };
}
