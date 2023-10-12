# drunken-bishop

OpenSSH fingerprint visualization algorithm in Rust. Just for my learning purpose.

## Usage

See [`examples/basic.rs`](examples/basic.rs).

Handy CLI is available at [`src/main.rs`](src/main.rs).

```console
$ cargo run --quiet -- -h
OpenSSH fingerprint visualization algorithm in Rust.

Usage: drunken-bishop [OPTIONS] [STRING]...

Arguments:
  [STRING]...  Strings to visualize. All strings will be joined with a space

Options:
  -s, --sha256   Use SHA-256 digest of given string
  -v, --verbose  Verbose output
  -h, --help     Print help
  -V, --version  Print version
```

e.g.

```console
$ cargo run --quiet -- --sha256 --verbose "hello world"
- Input string  : hello world
- SHA-256 digest: b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9
- Drunken bishop:
+-----------------+
|                 |
|                 |
|          ..     |
|         =.o.    |
|     ...So* +.   |
|    ..oooB.#  .  |
|    ...o+oB O..  |
|    .. +o..++o . |
|   ..++oE o*=+.  |
+-----------------+
```

## License

MIT. See [`LICENSE`](LICENSE).

## Reference

- [The drunken bishop: An analysis of the OpenSSH fingerprint visualization algorithm](http://www.dirk-loss.de/sshvis/drunken_bishop.pdf); Dirk Loss, Tobias Limmer, Alexander von Gernler, September 20, 2009

