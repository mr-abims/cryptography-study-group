# Questions from Week 3
Link to questions: https://uncloak.org/courses/rust+cryptography+engineering/course-2022-12-02+Session+3+Notes

## Chapter 5

### Benchmarking blake3, sha2 and sha3

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


## Chapter 6

### 6.1
Lets say that CBC-MAC is used at a company for their internal email communication. There are often updates sent out internally about upcoming public release. Suppose that EVE performs a Man-in-the-middle
attack and is able to view the message be sent from the boss's email to all of the employees. Eve can then take the MAC(m) and m and modify m to include a new block of text at the end which would
change the meaning of the intended message drastically. For example, she could change the update to include a different deadline at the end which could cause the company to miss the deadline of their
launch since some employees had the wrong info.

Let's call the modified message m'. Eve can then easily compute MAC(m') by using message extension. Therefore, the employees that recieve this message will verify it as legit.

### 6.3
Attacker has M(a), M(b), M(a||b). Message that the attacker wants to forge a MAC for is b || (M(b) xor M(a) xor b)

M(a||b) = E(b xor M(a))

H0 = 0 (IV)
H1 = E(b) = M(b)
H2 = E((M(b) xor M(a) xor b) xor H1)
   = E((M(b) xor M(a) xor b) xor M(b))
   = E(M(a) xor b)
   = M(a||b)

### 6.4
Attacker has `a` and `M(a) = t`.

CBC-MAC is defined as
H0 = 0
Hi = E(K, Pi xor Hi-1)
MAC = Hk

So in the case of `a`, since it is one block long, `M(a) = E(K, a)`.

I am not sure how to proceed from here

