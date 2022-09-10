# AccDump

Inspect Solana accounts data bytes.


### Installation
```sh
cargo install --git https://github.com/lucasig11/accdump
```

### Usage
```sh
accdump [ACCOUNT_ADDRESS] -u https://api.mainnet-beta.solana.com/
```

### Example output
```console
$ accdump AU16tHyVqcCUu93qoYRxPHktZxufQmivCwAZGjQ3UvUf
dd 60 ef 9b  61 87 02 92 | Ý . ï . a . . . |
00 4a c7 4d  3c d9 23 92 | . J Ç M . Ù . . |
f8 7d 7d aa  43 a4 cf 29 | ø . . ª C . Ï . |
a6 8f ae 72  d5 db 59 9f | . . . r Õ Û Y . |
2f 3b fd ba  0e 87 10 07 | . . ý º . . . . |
aa 5a 00 00  00 00 00 00 | ª Z . . . . . . |
00 80 f0 fa  02 00 00 00 | . . ð ú . . . . |
00 20 00 01  00 00 00 00 | . . . . . . . . |
00 00 00 00  00 00 00 00 | . . . . . . . . |
00 00 00 00  00 00 00 00 | . . . . . . . . |
00 00 00 00  00 00 00 00 | . . . . . . . . |
00 00 00 00  00 00 00 00 | . . . . . . . . |
00 00 00 00  00 00 00 00 | . . . . . . . . |
00 00 00 00  00 00 00 00 | . . . . . . . . |
00 00 00 00  00 00 00 00 | . . . . . . . . |
00 00 00 00  00 00 00    |   . . . . . . . |
```
