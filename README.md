# hamming-bow

[![Discord][dci]][dcl] [![Crates.io][ci]][cl] ![MIT/Apache][li] [![docs.rs][di]][dl] ![LoC][lo] ![Tests][btl] ![Lints][bll] ![no_std][bnl]

[ci]: https://img.shields.io/crates/v/hamming-bow.svg
[cl]: https://crates.io/crates/hamming-bow/

[li]: https://img.shields.io/crates/l/specs.svg?maxAge=2592000

[di]: https://docs.rs/hamming-bow/badge.svg
[dl]: https://docs.rs/hamming-bow/

[lo]: https://tokei.rs/b1/github/rust-cv/hamming-bow?category=code

[dci]: https://img.shields.io/discord/550706294311485440.svg?logo=discord&colorB=7289DA
[dcl]: https://discord.gg/d32jaam

[btl]: https://github.com/rust-cv/hamming-bow/workflows/tests/badge.svg
[bll]: https://github.com/rust-cv/hamming-bow/workflows/lints/badge.svg
[bnl]: https://github.com/rust-cv/hamming-bow/workflows/no-std/badge.svg

Produces binary term frequency bit arrays for hamming-space bag of words

## How it works

This works by using `hamming-dict` to create codewords in the hamming space that are as maximally spaced out as possible.

For each input key, its nearest neighbor is found in the dictionary and the corresponding bit is set in the bag. If the number of bits set in the bag becomes
sufficiently large, the threshold number of word occurences required to set a bit will increase to balance the hash.
