## tetcoin-erasure-coding

As part of Tetcoin's availability system, certain pieces of data
for each block are required to be kept available.

The way we accomplish this is by erasure coding the data into n pieces
and constructing a merkle root of the data.

Each of n validators stores their piece of data. We assume n=3f+k, 0 < k ≤ 3.
f is the maximum number of faulty validators in the system.
The data is coded so any f+1 chunks can be used to reconstruct the full data.