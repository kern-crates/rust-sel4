{ mk, versions }:

mk {
  package.name = "tests-capdl-http-server-components-timer-driver-sp804-driver";
  dependencies = rec {
    inherit (versions) log;
    tock-registers = "0.8.1";
  };
}