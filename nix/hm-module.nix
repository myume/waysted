self: {
  config,
  pkgs,
  lib,
  ...
}: let
  inherit (lib.modules) mkIf;
  inherit (lib.types) package;
  inherit (lib.options) mkOption mkEnableOption;

  cfg = config.services.waysted;
in {
  options.services.waysted = {
    enable = mkEnableOption "Waysted, Screentime tracker for wayland";

    package = mkOption {
      description = "The Waysted package";
      type = package;
      default = self.packages.${pkgs.stdenv.hostPlatform.system}.waysted;
    };
  };

  config = mkIf cfg.enable {
    home.packages = [cfg.package];
    systemd.user.services.waysted = {
      Unit = {
        Description = "Waysted";
        After = ["graphical-session-pre.target"];
      };

      Service = {
        Environment = "RUST_LOG=info";
        ExecStart = "${cfg.package}/bin/waysted-daemon";
        Restart = "always";
        RestartSec = "10";
      };

      Install = {
        WantedBy = ["default.target"];
      };
    };
  };
}
