{ config, lib, pkgs, ... }:

with lib; let
  cfg = config.services.dnsbot;
  toBool = var: if var then "true" else "false";
  dnsbot = pkgs.callPackage ./package.nix { };
in
{
  options.services.dnsbot = {
    enable = mkEnableOption "Enable dnsbot";

    passwordEnvFile = mkOption {
      type = types.path;
      example = "PASSWORD=p4ssW0rD_";
      description = "Path to password env file";
    };

    homeserver = mkOption {
      type = types.str;
      example = "https://matrix.org/";
      description = "Matrix homeserver URL";
    };

    username = mkOption {
      type = types.str;
      example = "dnsbot";
      description = "Matrix username";
    };

    datadir = mkOption {
      type = types.path;
      default = "/var/lib/dnsbot";
      description = "Data directory";
    };

    package = mkOption {
      type = types.package;
      default = dnsbot;
      description = "dnsbot package to use";
    };
  };

  config.systemd.services = mkIf cfg.enable {
    dnsbot = {
      enable = true;
      description = "DNS resolver in matrix";
      environment = {
        HOMESERVER = cfg.homeserver;
        USERNAME = cfg.username;
      };
      unitConfig = {
        Type = "simple";
      };
      serviceConfig = {
        User = "dnsbot";
        Group = "dnsbot";
        WorkingDirectory = cfg.datadir;
        ExecStart = "${cfg.package}/bin/dnsbot";
        Restart = "on-failure";
        RestartSec = "1s";
        EnvironmentFile = cfg.passwordEnvFile;
      };
      wantedBy = [ "multi-user.target" ];
    };
  };

  config.users = mkIf cfg.enable {
    users.dnsbot = {
      isSystemUser = true;
      description = "dnsbot user";
      home = cfg.datadir;
      createHome = true;
      group = "dnsbot";
    };

    groups.dnsbot = { };
  };
}
