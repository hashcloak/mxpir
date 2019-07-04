# mXPIR: A computational multi-query PIR library for XPIR

mXPIR is a research library and should not be used in production systems. 
mXPIR allows a client to download several elements from a database stored by a 
server without revealing which elements were downloaded. mPIR was introduced at 
the Symposium on Security and Privacy (Oakland) in 2018. You can find
a copy of the paper [here](https://eprint.iacr.org/2017/1142.pdf).

NOTE: mXPIR works only with XPIR. A version of mPIR that works for
SealPIR is also available, but it does not work with the latest version
of SealPIR (we updated SealPIR's interface and have not yet updated mPIR).

# Compiling mXPIR

Make sure you have all of the pre-requisites of [xpir-rust](https://github.com/pung-project/xpir-rust).

You will need Rust nightly version 1.31 (it might work with newer versions but we have not tested it).
You can install that version of rust with rustup:

  ``rustup override set nightly-2018-10-24``

You can then compile mXPIR with cargo:

  ``cargo build --release``

# Reproducing results from paper

You can call: ``cargo bench`` to reproduce the microbenchmarks.

# How to use this library.

We do not have a user-friendly example at this time (sadly). Take a look
at the microbenchmark code (benches/pir.rs) for examples on how to use it.
