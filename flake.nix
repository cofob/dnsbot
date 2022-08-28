{
  description = "DNS resolver in matrix";

  inputs.nixpkgs.url = "github:nixos/nixpkgs/nixos-22.05";

  outputs = { self, nixpkgs }:
    {
      nixosModule = import ./module.nix;
    };
}
