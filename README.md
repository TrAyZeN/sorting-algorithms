<h1 align="center">
    sorting-algorithms
</h1>

> Sorting algorithms implementation and visualization in Rust

## Comparison of sorting algorithms
### Complexity
In the following table n is the number of record to be sorted.
(The following table comes from [Wikipedia](https://en.wikipedia.org/wiki/Sorting_algorithm#Comparison_sorts))
| Name                                                              | Best     | Average  | Worst    | Memory |
|-------------------------------------------------------------------|----------|----------|----------|--------|
| [Bubble Sort](https://en.wikipedia.org/wiki/Bubble_sort)          | n        | n²       | n²       | 1      |
| [Selection Sort](https://en.wikipedia.org/wiki/Selection_sort)    | n²       | n²       | n²       | 1      |
| [Insertion Sort](https://en.wikipedia.org/wiki/Insertion_sort)    | n        | n²       | n²       | 1      |
| [Shaker Sort](https://en.wikipedia.org/wiki/Cocktail_shaker_sort) | n        | n²       | n²       | 1      |
| [Gnome Sort](https://en.wikipedia.org/wiki/Gnome_sort)            | n        | n²       | n²       | 1      |
| [Merge Sort](https://en.wikipedia.org/wiki/Merge_sort)            | n*log(n) | n*log(n) | n*log(n) | n      |

### Benchmark
The following metrics were captured with an Intel Core i5 4590.
| Name           | 10000 entries | 100000 entries |
|----------------|---------------|----------------|
| Bubble Sort    | 115.39 ms     | 14.346 s       |
| Selection Sort | 39.694 ms     | 3.8699 s       |
| Insertion Sort | 14.960 ms     | 1.4587 s       |
| Shaker Sort    | 125.24 ms     | 12.766 s       |
| Gnome Sort     | 58.121 ms     | 5.4517 s       |
| Merge Sort     | 3.1771 ms     | 29.921 ms      |

## Requirements
- [Rust](https://www.rust-lang.org/)

## Install
```
git clone https://github.com/TrAyZeN/sorting-algorithms
```

## Usage

### Run visualization
```
cargo run
```

### Run benchmarks
```
cargo bench
```

### Run tests
```
cargo test
```

## License
MIT TrAyZeN
