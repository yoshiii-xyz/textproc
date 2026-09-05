# textproc

Count words, lines, and bytes in text files.

## Install

```console
cargo build --release
sudo cp target/release/textproc /usr/local/bin/
```

## Usage

```console
textproc README.md
cat file.txt | textproc
```

Example:

```console
$ printf 'one two\nthree\n' | textproc
-: 2 lines, 3 words, 14 bytes
```
