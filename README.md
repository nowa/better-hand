# Better-Hand

Calculates opponent hands which beat yours for No-Limit Texas Hold 'Em

```
Calculates opponent hands which beat yours for No-Limit Texas Hold 'Em

USAGE:
    better-hand [FLAGS] --board <BOARD> --hand <HAND>

FLAGS:
        --help       Prints help information
    -V, --version    Prints version information
    -v               Presents the table with the exact hands that beat you, not just a range chart.

OPTIONS:
    -b, --board <BOARD>    Takes a string of the board so far, with cards indicated in RANKsuit form (e.g. Th is the 10
                           of hearts). Cards are unseparated (e.g. AhAsAcAd9s)
    -h, --hand <HAND>      Takes a string of your hole cards, with cards indicated in RANKsuit form (e.g. 9s is the 9 of
                           spades). Cards are unseparated (e.g. AhAs)
```

### Installation

```
	cargo install better-hand
```
