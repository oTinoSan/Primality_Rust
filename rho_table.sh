#!/bin/bash

## Resource Request
#SBATCH --job-name=btc_mine
#SBATCH --output=rho_table.out
#SBATCH --ntasks=1
#SBATCH --cpus-per-task=128

#SBATCH --mail-user ebtenne@lps.umd.edu
#SBATCH --mail-type BEGIN
#SBATCH --mail-type END
#SBATCH --mail-type FAIL

srun echo "`Start process`"
srun target/release/rho_table
srun echo "`End process`"