#
# Copyright 2023, Colias Group, LLC
#
# SPDX-License-Identifier: BSD-2-Clause
#

{ lib
}:

let
  fetchGit =
    { url, rev
    , local ? null
    , useLocal ? false
    , andThen ? ""
    }:

    assert useLocal -> local != null;

    let
      remote = builtins.fetchGit rec {
        inherit url rev;
        ref = mkKeepRef rev;
      };
      base = if useLocal then (lib.cleanSource local) else remote;
    in
      base + andThen;

  capdlCommon = {
    url = "https://github.com/coliasgroup/capdl.git";
    rev = "6ea203e8ebbcf8d786939aa706262b5073aae676"; # branch rust-testing
    local = localRoot + "/capdl";
  };

  srcRoot = ../../..;

  # TODO
  # localRoot = srcRoot + "/tmp/src";
  localRoot = srcRoot + "/../x";

  mkKeepRef = rev: "refs/tags/keep/${builtins.substring 0 32 rev}";

in rec {
  inherit srcRoot localRoot;
  inherit fetchGit mkKeepRef;

  # NOTE: Be sure to keep the commit hashes in the top-level README up-to-date.

  seL4 = {
    rust = fetchGit {
      url = "https://github.com/coliasgroup/seL4.git";
      rev = "dcdd2bcf2a64f3b34d2a406824fa54df7ed55571"; # branch "rust-testing"
      local = localRoot + "/seL4";
    };

    rust-microkit = fetchGit {
      url = "https://github.com/coliasgroup/seL4.git";
      rev = "968d8f6f97a37ea315243510348e933b612319f1"; # branch "rust-microkit"
      local = localRoot + "/seL4";
    };
  };

  microkit = fetchGit {
    url = "https://github.com/coliasgroup/microkit.git";
    rev = "f3e3265085b5a6ff547906fdee75e7aa2590884d"; # branch "rust-nix"
    local = localRoot + "/microkit";
  };

  capdlTool = fetchGit (capdlCommon // {
    andThen = "/capDL-tool";
  });

  pythonCapDLTool = fetchGit (capdlCommon // {
    andThen = "/python-capdl-tool";
  });

  objectSizes = fetchGit (capdlCommon // {
    andThen = "/object_sizes";
  });
}
