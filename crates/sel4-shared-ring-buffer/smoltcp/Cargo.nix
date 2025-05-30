#
# Copyright 2023, Colias Group, LLC
#
# SPDX-License-Identifier: BSD-2-Clause
#

{ mk, localCrates, versions, smoltcpWith }:

mk {
  package.name = "sel4-shared-ring-buffer-smoltcp";
  dependencies = {
    inherit (versions)
      log
      lock_api
      one-shot-mutex
    ;
    smoltcp = smoltcpWith [];
    inherit (localCrates)
      sel4-abstract-rc
      sel4-shared-ring-buffer
      sel4-shared-ring-buffer-bookkeeping
      sel4-abstract-allocator
      sel4-shared-memory
    ;
  };
}
