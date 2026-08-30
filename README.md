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

Output:

```
README.md: 42 lines, 156 words, 1024 bytes
```
