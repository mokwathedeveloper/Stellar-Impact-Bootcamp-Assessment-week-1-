# Project 1: Interactive Bill Manager

A command-line bills and expenses manager built in Rust. This is Week 1 of the Stellar Impact Bootcamp assessment, bringing together core Rust concepts — structs, hashmaps, functions, loops, and user input — into a single working program.

---

## What It Does

Run the program and you get an interactive menu in your terminal. You can add bills by name and amount, view everything you owe, remove bills you've paid off, and edit existing ones if the amount changes. Type a number, hit enter, done.

```
1. Add bill
2. View bills
3. Remove bill
4. Edit bill
5. Quit
```

---

## How to Run

Make sure you have Rust installed. If not, get it at [rustup.rs](https://rustup.rs).

```bash
git clone <repo-url>
cd Stellar-Impact-Bootcamp-Assessment-week-1-
cargo run
```

---

## Example Session

```
1. Add bill
2. View bills
3. Remove bill
4. Edit bill
5. Quit
> 1
Bill name:
Rent
Amount:
1200
Bill added.

> 1
Bill name:
Electricity
Amount:
85.50
Bill added.

> 2
Rent: $1200.00
Electricity: $85.50

> 4
Bill name to edit (or 'back' to cancel):
Electricity
New amount (or 'back' to cancel):
90
Bill updated.

> 3
Bill name to remove:
Rent
Bill removed.

> 5
```

---

## Project Structure

```
Stellar-Impact-Bootcamp-Assessment-week-1-/
├── Cargo.toml
├── src/
│   └── main.rs
└── README.md
```

---

## How It Was Built — Stage by Stage

### Stage 1 — Add and View Bills
Introduced the `Bill` struct, a `HashMap` to store bills by name, and two functions: `add_bill` and `view_bills`. A `loop` drives the interactive menu.

```bash
git commit -m "Complete stage 1: add and view bills"
```

### Stage 2 — Remove Bills
Added the `remove_bill` function. Uses `HashMap::remove` to find and delete a bill by name, with feedback if the bill doesn't exist.

```bash
git commit -m "Complete stage 2: remove bills"
```

### Stage 3 — Edit Bills + Go Back
Added the `edit_bill` function. At any prompt during editing, the user can type `back` to cancel the operation and return to the main menu without making changes.

```bash
git commit -m "Complete stage 3: edit bills and go back"
```

---

## Rust Concepts Practiced

- Structs
- `HashMap` for key-value storage
- Mutable and immutable references (`&mut`, `&`)
- Pattern matching with `match`
- Error handling with `Result` and `match`
- `loop` for interactive menus
- Separating logic into focused functions
- Reading user input with `std::io`

---

## Assignment

Stellar Impact Bootcamp — Week 1  
Project 1: Interactive Bill Manager
