Solutions to [Advent of Code](https://adventofcode.com) puzzles using the [Lamellar](https://crates.io/crates/lamellar) Runtime.

For most these problems, there appears to exist an efficient serial implementation, limiting the benefit of a parallel (and distributed) runtime.
Still, this is an effort to explore Lamellar and provide some examples on how to use the runtime!

Generally, I will implement a serial solution as well as at least one Lamellar based solution.

# Performance
Run in WSL on an AMD 7950x

<table>
<tr><th>Day</th><th>Part 1 Performance</th><th>Part 2 Performance</th></tr>
<tr><td>

|   D   | Implementation       | 
| :---: | -------------------- | 
|   1   | Serial               | 
|   1   | Active Message       | 
|   1   | Active Message Group | 

| | |
| :---: | ---------------------|
|   2   | Serial               | 
|   2   | Active Message       | 
|   2   | Active Message Group | 


| | |
| :---: | ---------------------|
|   3   | Serial               | 
|   3   | Active Message       | 

| | |
| :---: | ---------------------|
|   4   | Serial               | 
|   4   | Active Message       | 

| | |
| :---: | ---------------------|
|   5   | Serial               | 
|   5   | Active Message       | 

| | |
| :---: | ---------------------|
|   6   | Serial               | 
|   6   | Active Message       | 

| | |
| :---: | ---------------------|
|   7   | Serial               | 
|   7   | Active Message       | 

| | |
| :---: | ---------------------|
|   8   | Serial               |
|   8   | Lamellar Array       | 

| | |
| :---: | ---------------------|
|   9   | Serial               |
|   9   | Active Message       | 

| | |
| :---: | ---------------------|
|   10   | Serial               |
|   10   | Active Message       |

| | |
| :---: | ---------------------|
|   11   | Serial               |
|   11   | Active Message       |



</td><td>

| Generator |  Runner  |
| :-------: | :------: |
| 14.032 ns  | 16.779 µs |
| 14.039 ns  | 995.55 µs |
| 14.093 ns  | 110.04 µs |

| | |
| :---: | ---------------------|
| 13.841 ns | 32.918 µs |
| 13.723 ns  | 121.13 µs |
| 13.806 ns  | 45.955 µs |

| | |
| :---: | ---------------------|
| 24.366 µs | 31.315 µs |
| 25.052 µs  | 103.78 µs |

| | |
| :---: | ---------------------|
| 14.200 ns | 271.21 µs |
| 13.991 ns  | 191.44 µs |

| | |
| :---: | ---------------------|
| 13.757 ns |  17.769 µs |
| 13.837 ns  | 55.552 µs |

| | |
| :---: | ---------------------|
| 195.02 ns |  14.865 ns |
| 199.93 ns  | 2.7972 µs |

| | |
| :---: | ---------------------|
| 13.784 ns |  127.97 µs  |
| 13.738 ns  | 854.15 µs |

| | |
| :---: | ---------------------|
| 8.8126 µs |  22.189 µs|
|           |  23.770 ms|

| | |
| :---: | ---------------------|
| 81.735 µs |  23.400 µs|
| 80.600 µs |  9.2678 µs|

| | |
| :---: | ---------------------|
| 14.266 ns | 77.939 µs |
| 14.031 ns |  1.3844 ms|

| | |
| :---: | ---------------------|
| 13.986 ns | 50.208 µs |
| 13.945 ns |  27.313 µs|


</td><td>

| Generator |  Runner  |
| :-------: | :------: |
| 14.011 ns  | 283.80 µs |
| 14.192 ns  | 906.67 µs |
| 14.012 ns  | 369.71 µs |

| | |
| :---: | ---------------------|
| 13.786 ns  | 34.194 µs |
| 13.748 ns | 121.56 µs |
| 13.854 ns | 45.970 µs |

| | |
| :---: | ---------------------|
|  25.097 µs | 26.659 µs |
| 24.718 µs  | 109.68 µs |

| | |
| :---: | ---------------------|
|  13.975 ns | 284.94 µs |
| 14.050 ns  | 752.05 µs|

| | |
| :---: | ---------------------|
|  13.779 ns | 38.057 µs |
| 14.019 ns  | 54.063 µs |

| | |
| :---: | ---------------------|
|  145.28 ns | 4.8219 ns |
|  146.73 ns  | 789.15 ns |

| | |
| :---: | ---------------------|
|  13.800 ns | 148.86 µs |
|  13.444 ns  | 829.43 µs |

| | |
| :---: | ---------------------|
| 9.1250 µs | 154.30 µs |
|           |  66.549 ms |

| | |
| :---: | ---------------------|
| 80.005 µs | 30.690 µs |
| 80.010 µs |  56.556 µs |

| | |
| :---: | ---------------------|
| 14.030 ns | 50.487 µs |
| 14.020 ns |  27.679 µs|



</td></tr>
</table>
