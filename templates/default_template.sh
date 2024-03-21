#!/bin/bash
#PBS -N {name}
#PBS -l select=1{ncpus}{mem}
#PBS -q {queue}
#PBS -l walltime={walltime}

cd $PBS_O_WORKDIR

{command}
