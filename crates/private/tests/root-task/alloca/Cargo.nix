#
# Copyright 2023, Colias Group, LLC
#
# SPDX-License-Identifier: BSD-2-Clause
#

{ mk, localCrates }:

mk {
  package.name = "tests-root-task-alloca";
  dependencies = {
    inherit (localCrates)
      sel4
      sel4-root-task
      sel4-alloca
    ;
  };
}
