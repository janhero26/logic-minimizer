# 🧮 logic-minimizer

A command-line tool that takes a propositional logic formula and minimizes it,
both by applying boolean simplification rules and by converting it into normal
forms (CNF/DNF).

Ever stared at something like `A ∧ (A ∨ B)` and known it just collapses to `A`,
but wished a tool would prove it for you? That's the idea here.

## ✨ What it does

Give it a formula, get back the smallest equivalent one. Under the hood it
parses the input into a syntax tree and applies logical equivalences (absorption,
De Morgan, distribution, and friends) until nothing more can be reduced.

## 🚦 Status

Actively building this out. Here's where things stand:

- [x] Syntax tree for propositional formulas (`Var`, `Not`, `And`, `Or`, `Implies`, `Iff`)
- [x] Readable formula output via the `Display` trait
- [x] Lexer (string to tokens)
- [ ] Parser (tokens to syntax tree)
- [ ] Boolean simplification (e.g. `A ∧ (A ∨ B)` becomes `A`)
- [ ] Conversion to CNF / DNF
- [ ] CLI interface

## 🔎 Example

Right now the formula is built directly in code. The expression `A ∧ (A ∨ B)`
prints as:

```
(A ∧ (A ∨ B))
```

Once the parser and simplifier land, that same input will reduce straight down
to `A` by the absorption law.

## 🛠️ Build & Run

You'll need the [Rust toolchain](https://rustup.rs).

```
git clone https://github.com/janhero26/logic-minimizer.git
cd logic-minimizer
cargo run
```

## 📄 License

MIT