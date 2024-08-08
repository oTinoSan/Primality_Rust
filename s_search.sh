#!/bin/bash

## Resource Request
#SBATCH --job-name=fermat_s
#SBATCH --output=results_s.out
#SBATCH --ntasks=16
#SBATCH --ntasks-per-node=1

#SBATCH --mail-user ebtenne@lps.umd.edu
#SBATCH --mail-type BEGIN
#SBATCH --mail-type END
#SBATCH --mail-type FAIL

srun echo "Start process"
srun target/release/s_lamellar
srun echo "End process"