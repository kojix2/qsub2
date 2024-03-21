#!/bin/bash
#PBS -N job
#PBS -l select=1:ncpus=1
#PBS -q batch
#PBS -l walltime=30:00:00:00

cd $PBS_O_WORKDIR

echo Hello, world!
