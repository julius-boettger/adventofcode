{
  inputs = {
    # for rocmPackages.clr version 6.4.3 with llvm 19
    # to not break OpenCL target if rocmPackages.clr is installed
    nixpkgs.url = "github:nixos/nixpkgs?rev=f4606b01b39e09065df37905a2133905246db9ed";
    systems.url = "github:nix-systems/default"; # can run on all systems
  };

  outputs = { self, nixpkgs, systems, ... }:
  let
    # to only use config for available hardware
    # this only works with `--impure`! fails silently otherwise
    enableHip = builtins.pathExists /dev/kfd; # AMD GPU
    enableCuda = builtins.pathExists /dev/nvidia0; # NVIDIA GPU

    eachSystem = fn: nixpkgs.lib.genAttrs (import systems) (system: fn system (import nixpkgs {
      inherit system;
      config.allowUnfree = enableCuda; # for cuda target
    }));
  in
  {
    devShells = eachSystem (system: pkgs: {
      default = with pkgs; mkShell {
        packages =
          [
            futhark
            ispc # for ISPC target
            opencl-headers ocl-icd # for OpenCL target
            hyperfine # benchmarking
          ]
          # for CUDA target
          ++ lib.optional enableCuda cudatoolkit
          # for HIP (AMD's CUDA) target
          ++ lib.optional enableHip rocmPackages.clr
        ;

        # more CUDA config
        shellHook = lib.optionalString enableCuda ''
          export CUDA_PATH=${cudatoolkit}
          export LIBRARY_PATH=${cudatoolkit}/lib/stubs:$LIBRARY_PATH
          export LD_LIBRARY_PATH=/run/opengl-driver/lib:$LD_LIBRARY_PATH
        '';
      };
    });
  };
}
