# Rust-Data-Processor
Rust-based CLI utility for parsing commands and computing statistical functions like log, sum, square, average, and standard deviation.

This project is a command-line Rust program that reads a set of instructions from a text file and computes the corresponding mathematical operations. It supports functions like:

- `log(base, value)`
- `sum_list` of numbers
- `double_list` (squares each value)
- `average_list`
- `std_dev` (standard deviation)

---

## Files

- `hw4.rs` – Main implementation of the data parser and logic engine.
- `test-hw4.rs` – Unit tests to validate correctness.
- `input.txt`, `input-1.txt`, `input-2.txt` – Sample input files used for testing.
- `output-1.txt`, `output-2.txt`, `output-1MINE.txt`, `output-2MINE.txt` – Expected vs actual outputs.

> **Note:** The `spam` and `ham` folders were not attached due to their large size.

---

## How It Works

Each line in the input file is a command followed by integers. The program parses these commands and executes the operation:

### Example Commands:
log 528 3 → Computes log base 3 of 528
sum_list 10 20 30 → Outputs sum of values
double_list 5 6 7 → Outputs square of each
average_list 3 5 7 → Computes average
std_dev 1 2 3 4 5 → Computes standard deviation

---

## 🛠️ Run Instructions

1. Ensure Rust is installed. If not:
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
