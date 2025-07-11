#
# Copyright 2023, Colias Group, LLC
#
# SPDX-License-Identifier: BSD-2-Clause
#

{ lib, stdenv, hostPlatform
, cmake, perl

, crateUtils, crates
, mkTask, seL4Modifications
, defaultRustTargetTriple

, mkInstance

, canSimulate
}:

let
  libcDir = "${stdenv.cc.libc}/${hostPlatform.config}";

in
mkInstance {
  rootTask = mkTask rec {
    rootCrate = crates.tests-root-task-c;

    release = false;

    # layers = [
    #   crateUtils.defaultIntermediateLayer
    #   {
    #     crates = [
    #       "sel4"
    #       "sel4-root-task"
    #     ];
    #     modifications = seL4Modifications;
    #   }
    # ];

    commonModifications = {
      modifyConfig = old: lib.recursiveUpdate old {
        target.${defaultRustTargetTriple.name} = {
          rustflags = (old.target.${defaultRustTargetTriple.name}.rustflags or []) ++ [
            # TODO
            # NOTE: won't work because cross gcc always uses hard-coded --with-ld

            # "-C" "linker-flavor=gcc"
            # "-C" "link-arg=-nostartfiles"
            # "-C" "default-linker-libraries=on"

            # "-Z" "gcc-ld=lld"
            # (or)
            # "-Z" "unstable-options"
            # "-C" "link-self-contained=+linker"
            # (or)
            # "-Z" "unstable-options"
            # "-C" "linker-flavor=gnu-lld-cc"

            # "-Z" "verbose"
          ];
        };
      };
    };

    lastLayerModifications = crateUtils.composeModifications seL4Modifications (crateUtils.elaborateModifications {
      modifyDerivation = drv: drv.overrideAttrs (self: super: {
        # NIX_LDFLAGS_AFTER = [ "-lnosys" ]; # NOTE: appease CMake's compiler test
        # NIX_DEBUG = 2;
      });
      # extraCargoFlags = [ "--verbose" ];
    });
  };

  extraPlatformArgs = lib.optionalAttrs canSimulate  {
    canAutomateSimply = true;
  };
}
