# Rust SQL Parser By Nisha Murali 🦀📄

A Rust-based SQL parser that can tokenize and parse basic SQL statements like `SELECT` and `CREATE TABLE`. This project demonstrates foundational compiler design concepts like tokenization, expression parsing (via Pratt parsing), abstract syntax trees, and error handling — all in Rust.

---

## 📌 Features

✅ Tokenizer  
✅ Pratt Expression Parser  
✅ SELECT Parser (`SELECT`, `WHERE`, `ORDER BY`)  
✅ CREATE TABLE Parser (with types & constraints)  
✅ AST Output  
✅ CLI-based interactive SQL input  
✅ Full error handling with meaningful messages

---

## 🧠 Technologies Used

- [Rust](https://www.rust-lang.org/)
- CLI (Command Line Interface)
- Pratt Parsing (top-down operator precedence)
- AST (Abstract Syntax Tree)

---

## 🔢 Supported SQL Examples

### ✅ SELECT
```sql
SELECT name, age FROM users WHERE age > 18 ORDER BY name;

CREATE TABLE students ( id INT PRIMARY KEY,name VARCHAR NOT NULL,age INT CHECK(age >= 18));
