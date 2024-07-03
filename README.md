# Artifact for "T-Rec: Fine-Grained Language-Agnostic Program Reduction Guided by Lexical Syntax"

## Getting Started Guide

### Obtain the Docker Image and Run a Container
1. If docker is not installed, install it by following the [instructions](https://docs.docker.com/get-docker/).
2. Pull the docker image from DockerHub: ```docker pull cancel/trec:latest```
3. Run a container from the image:
   ```docker container run --cap-add SYS_PTRACE --interactive --tty cancel/trec:latest```

### Directory Structure

After running the container the current directory should be ```/tmp```.
- ```binaries```: This folder contains the binaries of T-Rec
- ```benchmarks```: This folder contains all the scripts and benchmarks
  - ```c_benchmarks```: This folder contains all the C benchmarks
  - ```rust_benchmarks```: This folder contains all the Rust benchmarks
  - ```smt_benchamrks```: This folder contains all the SMT-LIBv2 benchmarks
  - ```binaries```: Contains scripts for running each reducer
  - ```covert_result_to_csv.py```: A script to convert results to csv format
  - ```patch_oracle.py```: Used by scripts in ```binaries```
  - ```run_exp_parallel.py```: A script for running experiments
  - ```deduplication```: Contains results and scripts that analyzes the results

### Running Experiments
#### Running a specific reducer to reduce a specific benchmark:
The following command can be used to run a specific reducer to reduce a specific benchmark
```./run_exp_parallel.py -s <benchmark_folder> -r <reducer> -o <output_dir>```
The result will be saved at ```<output_dir>```
E.g. to run perses to reduce clang-22382:
```./run_exp_parallel.py -s c_benchmarks/clang-22382 -r perses -o results```

#### Running multiple reducers to reduce multiple benchmarks:
For example to run both perses and trec to reduce all the benchmarks in Benchmark-Multi, the following command can be used:
```
./run_exp_parallel.py \
    -s c_benchmarks/* rust_benchmarks/* smt_benchmarks/benchmarks/* \
    -r perses trec \
    -o results \
    -j <number of cpu>
```
#### Supported reducers
```
creduce: the default C-Reduce
creduce_slow: C-Reduce with sllooww flag enabled
creduce_no_cano: C-Reduce without C-Reduce-cano
creduce_cano: C-Reduce-cano
creduce_no_cano_slow: C-Reduce with slloww falg enabled without C-Reduce-cano
perses: Perses
vulcan: Vulcan
trec: T-Rec on top of Perses
trec_vulcan: T-Rec on top of Vulcan
trec_no_cano: T-Rec with only lexical syntax-guided reduction
```
#### Reproduce Deduplication Results
##### Directory Structure in ```deduplication```
- ```results```: Contains results of different reducers
  - ```crash```: Contains results on the crash test cases
  - ```wrongcode```: Contains results on the miscompilation test cases
- ```analyze_results.py```: Script to analyze the results
  - Usage: ```python3 analyze_results.py -d <results_folder>```
  - E.g., ```python3 analyze_results.py -d results/crash/perses_0/```
- ```check_ddset_results.py```: Script to check the results of ddset
  - Usage: ```python3 check_ddset_results.py```