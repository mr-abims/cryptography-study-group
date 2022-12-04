# Questions from Week 3
Link to questions: https://uncloak.org/courses/rust+cryptography+engineering/course-2022-12-02+Session+3+Notes

## Benchmarking blake3, sha2 and sha3

blake3                  time:   [67.052 ns 67.390 ns 67.770 ns]
Found 5 outliers among 100 measurements (5.00%)
  4 (4.00%) high mild
  1 (1.00%) high severe

sha3                    time:   [456.28 ns 458.04 ns 459.99 ns]
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high mild

sha2                    time:   [293.40 ns 294.67 ns 296.08 ns]
Found 8 outliers among 100 measurements (8.00%)
  1 (1.00%) high mild
  7 (7.00%) high severe

