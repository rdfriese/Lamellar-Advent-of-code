Solutions to [Advent of Code](https://adventofcode.com) puzzles using the [Lamellar](https://crates.io/crates/lamellar) Runtime.

For most these problems, there appears to exist an efficient serial implementation, limiting the benefit of a parallel (and distributed) runtime.
Still, this is an effort to explore Lamellar and provide some examples on how to use the runtime!

Generally, I will implement a serial solution as well as at least one Lamellar based solution.

I utilize the [cargo-aoc](https://crates.io/crates/cargo-aoc) crate as the execution harness.

install using ` cargo install cargo-aoc`

run using `cargo aoc` optionally specifying a day with `-d <day>`

bench using `cargo aoc bench` optionally specifying a day with `-d <day>` and including the input parsing with `-g`

# Performance
Run in WSL on an AMD 7950x

<table>
<tr><th>Day</th><th>Part 1 Performance</th><th>Part 2 Performance</th></tr>
<tr><td>

|   D   | Implementation       | 
| :---: | -------------------- | 
|   1   | Serial               | 
|   1   | Active Message       | 

| | | 
| :---: | -------------- | 
|   2   | Serial         | 
|   2   | Active Message | 


| | |
| :---: | ---------------|
|   3   | Serial         | 
|   3   | Active Message | 

| | |
| :---: | -------------- |
|   4   | Serial         | 
|   4   | Active Message | 

| | |
| :---: | -------------- |
|   5   | Serial         | 
|   5   | Active Message | 

| | |
| :---: | -------------- |
|   6   | Serial         | 
|   6   | Active Message | 

| | |
| :---: | -------------- |
|   7   | Serial         | 
|   7   | Active Message | 

| | |
| :---: | -------------- |
|   8   | Serial         |
|   8   | Lamellar Array| 

| | |
| :---: | -------------- |
|   9   | Serial         |
|   9   | Active Message | 

| | |
| :---: | -------------- |
|   10  | Serial         |
|   10  | Active Message |

| | |
| :---: | -------------- |
|   11  | Serial         |
|   11  | Active Message |

| | |
| :---: | -------------- |
|   12  | Serial         |
|   12  | Active Message |

| | |
| :---: | -------------- |
|   13  | Serial         |
|   13  | Active Message |

| | |
| :---: | -------------- |
|   14  | Serial         |
|   14  | Active Message |

| | |
| :---: | -------------- |
|   15  | Serial         |
|   15  | Active Message |

</td><td>

| Generator |  Runner  | Total |
| :-------: | :------: | :------- |
| 14.032 ns  | 16.779 µs | 16.793 µs |
| 63.054 µs  | 5.0559 µs | 68.109 µs |

| | | |
| :---: | -------------- | -------- |
| 13.841 ns | 32.918 µs | 32.931 µs |
| 16.551 µs | 10.880 µs | 27.431 µs |

| | | |
| :---: | -------------- | -------- |
| 5.4154 µs | 29.827 µs | 35.242 µs |
| 24.537 µs | 12.151 µs | 36.688 µs |

| | | |
| :---: | -------------- | -------- |
| 14.200 ns  | 270.32 µs | 270.334 µs|
| 22.012 µs  | 62.169 µs | 84.181 µs |

| | | |
| :---: | -------------- | -------- |
| 13.757 ns |  17.769 µs | 17.782 µs
| 13.837 ns  | 43.560 µs | 43.573 µs

| | | |
| :---: | -------------- | -------- |
| 195.02 ns |  14.865 ns | 209.885 ns
| 199.93 ns  | 2.7972 µs | 2.996 µs

| | | |
| :---: | -------------- | -------- |
| 13.784 ns |  127.97 µs  |127.98 µs
| 28.847 µs  | 159.19 µs | 188.037 µs

| | | |
| :---: | -------------- | -------- |
| 8.8126 µs |  22.189 µs|
|           |  23.770 ms|

| | | |
| :---: | -------------- | -------- |
| 81.735 µs |  23.400 µs|
| 80.600 µs |  9.2678 µs|

| | | |
| :---: | -------------- | -------- |
| 14.266 ns | 77.939 µs |
| 14.031 ns |  1.3844 ms|

| | | |
| :---: | -------------- | -------- |
| 13.945 ns | 50.208 µs |
| 13.986 ns |  27.313 µs|

| | | |
| :---: | -------------- | -------- |
| 13.806 ns |  332.89 µs |  332.90 µs
| 13.950 ns  | 190.53 µs | 190.54 µs

| | | |
| :---: | -------------- | -------- |
| 47.878 µs |  22.416 µs |  70.294 µs
| 111.10 µs  | 10.529 µs | 121.629 µs

| | | |
| :---: | -------------- | -------- |
| 13.949 ns |  8.9763 µs |  8.9893 µs
| 14.015 ns  | 26.257 µs |  26.271 µs

| | | |
| :---: | -------------- | -------- |
| 13.783 ns |  22.018 µs |  22.031 µs
| 110.73 µs  | 4.0274 µs | 114.75 µs


</td><td>

| Generator |  Runner  | Total |
| :-------: | :------: | :------- |
| 14.011 ns  | 283.80 µs | 283.81 µs |
| 72.646 µs  | 60.733 µs | 133.37 µs |

| | | |
| :---: | -------------- | -------- |
| 13.786 ns | 33.285 µs | 33.298 µs |
| 16.181 µs | 9.4443 µs | 27.625 µs |

| | | |
| :---: | -------------- | -------- |
| 5.1958 µs | 37.210 µs | 42.405 µs |
| 24.124 µs | 11.453 µs | 35.177 µs |

| | | |
| :---: | -------------- | -------- |
|  13.715 ns | 299.45 µs | 313.165 µs |
| 21.927 µs  | 78.105 µs| 100.032 µs |

| | | |
| :---: | -------------- | -------- |
|  13.779 ns | 38.057 µs | 38.070 µs
| 14.019 ns  | 54.063 µs | 54.077 µs

| | | |
| :---: | -------------- | -------- |
|  145.28 ns | 4.8219 ns | 150.1 ns
|  146.73 ns  | 789.15 ns | 935.8 ns

| | | |
| :---: | -------------- | -------- |
|  13.800 ns | 148.86 µs | 148.87 µs
|  29.071 µs  | 165.84 µs | 183.91 µs

| | | |
| :---: | -------------- | -------- |
| 9.1250 µs | 154.30 µs |
|           |  66.549 ms |

| | | |
| :---: | -------------- | -------- |
| 80.005 µs | 30.690 µs |
| 80.010 µs |  56.556 µs |

| | | |
| :---: | -------------- | -------- |
| 14.084 ns | 230.04 µs |
| 14.043 ns |  180.17 µs |

| | | |
| :---: | -------------- | -------- |
| 14.030 ns | 50.487 µs |
| 14.020 ns |  27.679 µs|

| | | |
| :---: | -------------- | -------- |
| 14.052 ns |  20.596 ms | 20.596 ms
| 13.889 ns  | 5.561 ms | 5.561 ms

| | | |
| :---: | -------------- | -------- |
| 49.435 µs  |  134.58 µs | 183.493 µs
| 112.14 µs  | 51.599 µs | 163.739 µs

| | | |
| :---: | -------------- | -------- |
| 14.087 ns  | 3.4685 ms | 3.4685 ms
| 14.069 ns  | 6.5674 ms | 6.5674 ms

| | | |
| :---: | -------------- | -------- |
| 13.834 ns | 113.54 µs | 113.55 µs
| WIP  | WIP | WIP

</td></tr>
</table>
