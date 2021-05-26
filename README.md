# Better-Hand

Calculates your equity against opponent starting hands for No-Limit Texas Hold 'Em. 

### Installation

This package has been published to crates.io, and is installable with cargo.

```
	cargo install better-hand
```

### Usage

```
USAGE:
    better-hand [FLAGS] --board <BOARD> --hand <HAND>

FLAGS:
        --help           Prints help information
    -I, --interactive    Starts the tool in interactive mode. Exit with 'exit'
    -V, --version        Prints version information

OPTIONS:
    -b, --board <BOARD>    Takes a string of the board so far, with cards indicated in RANKsuit form (e.g. Th is the 10
                           of hearts). Cards are unseparated (e.g. AhAsAcAd9s)
    -h, --hand <HAND>      Takes a string of your hole cards, with cards indicated in RANKsuit form (e.g. 9s is the 9 of
                           spades). Cards are unseparated (e.g. AhAs)
```

### Explanation

This application allows you to input your hole cards and a set of cards on the board (at the flop, turn or river), and calculate the equity your hand has against possible opponent hands. Equity is defined as the probability that you will win that hand, taking into account any future cards that may appear, and the many different versions of each opponent starting hand (e.g. there are 4 ways to make a suited hand, whereas the same hand off-suit can be made in 12 different ways).

NLH starting hands are often described with a "range", a square chart of the ranks of your cards, as well as if they're suited or not. The output of this program is a chart in that form, where each cell includes your equity against it. Output is additionally color-coded if your terminal supports it. Blue squares are suited, white is off-suit, red implies that your odds are worse than 3 to 1, and yellow squares indicate impossible hands for the opponent to have given board texture.

For example: 

```
$ better-hand -b Jd4h6d9d -h 4s4c

+------+------+------+------+------+------+------+------+------+------+------+------+------+
| AA:  | AKs: | AQs: | AJs: | ATs: | A9s: | A8s: | A7s: | A6s: | A5s: | A4s: | A3s: | A2s: |
| 0.86 | 0.81 | 0.81 | 1.00 | 0.81 | 1.00 | 0.81 | 0.81 | 1.00 | 0.81 | 0.20 | 0.81 | 0.81 |
+------+------+------+------+------+------+------+------+------+------+------+------+------+
| AKo: | KK:  | KQs: | KJs: | KTs: | K9s: | K8s: | K7s: | K6s: | K5s: | K4s: | K3s: | K2s: |
| 0.91 | 0.86 | 0.74 | 1.00 | 0.74 | 1.00 | 0.81 | 0.81 | 1.00 | 0.81 | 0.20 | 0.81 | 0.81 |
+------+------+------+------+------+------+------+------+------+------+------+------+------+
| AQo: | KQo: | QQ:  | QJs: | QTs: | Q9s: | Q8s: | Q7s: | Q6s: | Q5s: | Q4s: | Q3s: | Q2s: |
| 0.91 | 0.83 | 0.86 | 1.00 | 0.67 | 1.00 | 0.74 | 0.81 | 1.00 | 0.81 | 0.20 | 0.81 | 0.81 |
+------+------+------+------+------+------+------+------+------+------+------+------+------+
| AJo: | KJo: | QJo: | JJ:  | JTs: | J9s: | J8s: | J7s: | J6s: | J5s: | Not  | J3s: | J2s: |
| 0.94 | 0.94 | 0.94 | 0.02 | 1.00 | 0.91 | 1.00 | 1.00 | 0.91 | 1.00 | Poss | 1.00 | 1.00 |
+------+------+------+------+------+------+------+------+------+------+------+------+------+
| ATo: | KTo: | QTo: | JTo: | TT:  | T9s: | T8s: | T7s: | T6s: | T5s: | T4s: | T3s: | T2s: |
| 0.91 | 0.83 | 0.75 | 0.94 | 0.86 | 1.00 | 0.67 | 0.74 | 1.00 | 0.81 | 0.20 | 0.81 | 0.81 |
+------+------+------+------+------+------+------+------+------+------+------+------+------+
| A9o: | K9o: | Q9o: | J9o: | T9o: | 99:  | 98s: | 97s: | 96s: | 95s: | Not  | 93s: | 92s: |
| 0.94 | 0.94 | 0.94 | 0.91 | 0.94 | 0.02 | 1.00 | 1.00 | 0.91 | 1.00 | Poss | 1.00 | 1.00 |
+------+------+------+------+------+------+------+------+------+------+------+------+------+
| A8o: | K8o: | Q8o: | J8o: | T8o: | 98o: | 88:  | 87s: | 86s: | 85s: | 84s: | 83s: | 82s: |
| 0.91 | 0.91 | 0.83 | 0.94 | 0.75 | 0.94 | 0.86 | 0.67 | 1.00 | 0.74 | 0.20 | 0.81 | 0.81 |
+------+------+------+------+------+------+------+------+------+------+------+------+------+
| A7o: | K7o: | Q7o: | J7o: | T7o: | 97o: | 87o: | 77:  | 76s: | 75s: | 74s: | 73s: | 72s: |
| 0.91 | 0.91 | 0.91 | 0.94 | 0.83 | 0.94 | 0.75 | 0.86 | 1.00 | 0.67 | 0.20 | 0.74 | 0.81 |
+------+------+------+------+------+------+------+------+------+------+------+------+------+
| A6o: | K6o: | Q6o: | J6o: | T6o: | 96o: | 86o: | 76o: | 66:  | 65s: | Not  | 63s: | 62s: |
| 0.94 | 0.94 | 0.94 | 0.91 | 0.94 | 0.91 | 0.94 | 0.94 | 0.02 | 1.00 | Poss | 1.00 | 1.00 |
+------+------+------+------+------+------+------+------+------+------+------+------+------+
| A5o: | K5o: | Q5o: | J5o: | T5o: | 95o: | 85o: | 75o: | 65o: | 55:  | 54s: | 53s: | 52s: |
| 0.91 | 0.91 | 0.91 | 0.94 | 0.91 | 0.94 | 0.83 | 0.75 | 0.94 | 0.86 | 0.20 | 0.67 | 0.74 |
+------+------+------+------+------+------+------+------+------+------+------+------+------+
| A4o: | K4o: | Q4o: | J4o: | T4o: | 94o: | 84o: | 74o: | 64o: | 54o: | Not  | 43s: | 42s: |
| 0.80 | 0.80 | 0.80 | 0.75 | 0.80 | 0.75 | 0.80 | 0.80 | 0.75 | 0.80 | Poss | 0.20 | 0.20 |
+------+------+------+------+------+------+------+------+------+------+------+------+------+
| A3o: | K3o: | Q3o: | J3o: | T3o: | 93o: | 83o: | 73o: | 63o: | 53o: | 43o: | 33:  | 32s: |
| 0.91 | 0.91 | 0.91 | 0.94 | 0.91 | 0.94 | 0.91 | 0.83 | 0.94 | 0.75 | 0.80 | 0.91 | 0.74 |
+------+------+------+------+------+------+------+------+------+------+------+------+------+
| A2o: | K2o: | Q2o: | J2o: | T2o: | 92o: | 82o: | 72o: | 62o: | 52o: | 42o: | 32o: | 22:  |
| 0.91 | 0.91 | 0.91 | 0.94 | 0.91 | 0.94 | 0.91 | 0.91 | 0.94 | 0.83 | 0.80 | 0.83 | 0.91 |
+------+------+------+------+------+------+------+------+------+------+------+------+------+
```
