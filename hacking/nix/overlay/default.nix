#
# Copyright 2023, Colias Group, LLC
#
# SPDX-License-Identifier: BSD-2-Clause
#

self: super: with self;

let
  scopeName = "this";
in

assert !(super ? scopeName);

{

  "${scopeName}" = makeScopeWithSplicing' rec {
    otherSplices = generateSplicesForMkScope scopeName;
    f = self: callPackage ../scope {} self // {
      __dontMashWhenSplicingChildren = true;
      inherit otherSplices; # for child spliced scopes
    };
  };

  # Add Python packages needed by the seL4 ecosystem
  pythonPackagesExtensions = super.pythonPackagesExtensions ++ [
    (callPackage ./python-overrides.nix {})
  ];

}
