# LU matrix decomposition with paralellized computations
Given a text file representing a matrix, compute independent tasks
for matrix decomposition, construct a dependency graph for the tasks
and calculate groups fit for paralellization. Then execute tasks
per group and use results for modification of matrix.
Lastly print decomposed matrix to stdout.

## Usage
Compile the binary and run with input file
```bash
cargo install --git https://github.com/unluckysiata/paralell_gauss
paralell_gauss [input file]
```
or clone the repo and run
```bash
cargo build --release
cargo run [input_file]
```

## Example
Suppose you have a file named a.txt with the following contents
```
3
2.0 1.0 3.0
4.0 3.0 8.0
6.0 5.0 16.0
6.0 15.0 27.0
```
where 3 equals n and last row is vector b transposed.
Output:
```
Decomposed:
2 1 3 | 6
0 1 2 | 3
0 0 3 | 3
```
