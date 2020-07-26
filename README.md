<h1 align="center">
    sorting-algorithms
</h1>

> Sorting algorithms implementation in Rust

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

### Benchmark
The following metrics were captured with an Intel Core i5 4590.
| Name           | 100 entries | 1000 entries | 10000 entries | 100000 entries |
|----------------|-------------|--------------|---------------|----------------|
| Bubble Sort    | 6.5944 us   | 723.26 us    | 105.94 ms     | 13.917 s       |
| Selection Sort | 4.4945 us   | 434.07 us    | 39.318 ms     | 3.9262 s       |
| Insertion Sort | 2.5376 us   | 211.77 us    | 20.680 ms     | 1.9795 s       |
| Shaker Sort    | 4.1071 us   | 766.16 us    | 88.224 ms     | 10.509 s       |
| Gnome Sort     | 5.5956 us   | 591.39 us    | 54.595 ms     | 5.4465 s       |

## Requirements
- [Rust](https://www.rust-lang.org/)

## Install
```
git clone https://github.com/TrAyZeN/sorting-algorithms
cd sorting-algorithms
```

## License
MIT TrAyZeN
