# qgpu

A simple job scheduler to monitor gpu usage across multiple nodes and submit jobs to available resources.

# Installation
```{bash}
git clone https://github.com/noamteyssier/qgpu
cd qgpu
cargo build --release && cargo install --path .
```

# Configuration

## Node Pool
Describes the json configuration of the nodes you'd like to queue to.

### Recognized Fields:
* name
  * name to node as specified in your ~/.ssh/config file
  * requires that passwordless ssh is setup
  * requires that proxyjumps are set up
* env
  * Optional configuration
  * name of cuda environment to start on the node

### Example JSON
```{json}
{
    "name": "kgpu1",
    "env": "ifpe_cuda10"
}
{
    "name": "kgpu2"
}
{
    "name": "kgpu10",
    "env": "ifpe_cuda11"
}
```

## Job Pool
Describes the configuration of the jobs you'd like to submit across the node pool


### Recognized Fields:

* command
  * command to use to start the job
  * if you are submitting a bash script use "bash"
  * if you are submitting a python script use "python"
  * can also pass multiple values : "bash <script>.sh"

* args
  * optional configuration
  * list of arguments to provide to command


* relative_path
  * Optional configuration
  * the relative path from $HOME where you'd like the command to be run
  * under the hood will `cd <path>`

* env
  * Optional configuration
  * name of cuda environment to start for job
  * takes priority over node envrionment if both are provided

* n_submission
  * Optional configuration
  * number of times to submit this job
  * not implemented yet, but useful for when the same script needs to be run multiple times

### Example JSON
```{json}
{
    "command": "bash",
    "args": ["tune_dataset.sh", "../_cache/splits/my_dataset1.tab", "cml", "hparams_cml_data2"],
    "relative_path": "~/projects/project_dir/"
}
{
    "command": "bash",
    "args": ["tune_dataset.sh", "../_cache/splits/my_dataset2.tab", "cml", "hparams_cml_data2"],
    "relative_path": "~/projects/project_dir/"
}
```

# Usage

## How it works
qgpu queries which resources are available given free memory/usage thresholds and will set environment variables used to submit jobs across the node pool that meet that threshold.

The way for you to use this is through accessing those environment variables in the same way you would with any job scheduler.

qgpu uses these environment variables :

* QG_NODE_ID
  * the zero_indexed node_id across the node pool

* QG_GPU_ID
  * the zero indexed gpu_id within a given node

* QG_LOG_PATH
  * a path to write stdout
  * node<QG_NODE_ID>_gpu<QG_GPU_ID>.log.txt


## Example Script
```{bash}
#!/usr/bin/env bash

dataset=$1
model=$2
output=$3

python3 my_machine_learning_run.py \
    -i ${dataset} \
    -m ${model} \
    -o ${output} \
    -f 4096 \
    -n 12 \
    -g ${QG_GPU_ID} \
    -M 100 \
    -t 25 > ${QG_LOG_PATH}
```

## Querying Usage and Submitting
```{bash}
# access help menu
qgpu --help

# access help menu for stat
qgpu stat --help

# access help menu for sub
qgpu sub --help

# query the usage statistics on the node pool
qgpu stat -i node_pool.json

# submit a pool of jobs across the available nodes on a node pool
qgpu sub -i node_pool.json -j jobs.json

# submit with very stringent resource availability
qgpu sub -i node_pool.json -j jobs.json -f 100 -F 100

# submit with flexible resource availability
qgpu sub -i node_pool.json -j jobs.json -f 50 -F 50

# submit with stringent utilization with flexible memory utilization
qgpu sub -i node_pool.json -j jobs.json -f 100 -F 20

# submit with flexible utilization with stringent memory utilization
qgpu sub -i node_pool.json -j jobs.json -f 20 -F 100

```
