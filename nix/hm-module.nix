self: {
  config,
  pkgs,
  lib,
  ...
}: let
  inherit (lib.modules) mkIf;
  inherit (lib.types) package enum;
  inherit (lib.options) mkOption mkEnableOption;

  cfg = config.services.waysted;
in {
  options.services.waysted = {
    enable = mkEnableOption "Waysted, Screentime tracker for wayland";

    compositor = mkOption {
      description = "The wayland compositor";
      type = enum ["niri" "hyprland"];
    };

    package = mkOption {
      description = "The Waysted package";
      type = package;
      default = self.packages.${pkgs.stdenv.hostPlatform.system}.waysted.overrideAttrs (finalAttrs: prevAttrs: {
        cargoBuildFlags = ["--features=${cfg.compositor}"];
      });
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
