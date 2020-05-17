# PIGLATIN_RS

![Rust](https://github.com/gauthamkrishna9991/piglatin_rs/workflows/Rust/badge.svg?branch=master)

## Introduction

This is a small pig-latin program made with Rust Programming Language.

This converts all words starting with a consonant to have their first letter removed and is postfixed by a prefix of the first letter on 'ay'.

`first -> irst-fay`

`russia -> ussia-ray`

For letters staring with vowels, we postfix the whole word with '-hay'

`apple -> apple-hay`

`envelope -> envelope-hay`


The present version is considered by the author to be type-safe and memory-safe yet simple.

For people coming from non-rust background this might be difficult to understand. Do not delve into it unless you understand rust well.

## Requirements

- `rust` (2018 Edition)
- `cargo` (ver. 1.42 or later)

Make sure these are findable in your path (if you're unsure, most probably you might not have it installed. Install from [Rustup  Website](https://rustup.rs/))

## Using

Clone this repository and run the following command on a terminal opened in the folder to start the program:

`cargo run` 

OR for sweet performance benefits :)

`cargo run --release`

## Releases

### v. 0.1.0

- Wrote program and built the necessary binaries, tested it with some words, so far works.
- TODO: Tests to be added to the project.

## Contributions

Though I am not looking for contributions to this project right now, any contributions or any advice on making the code better is all accepted, and also encouraged.

If you wanna do this, hit me up. :)

## License

Copyright (c) 2020 Goutham Krishna K V. All rights reserved.

Licensed under the [MIT LICENSE](LICENSE)
