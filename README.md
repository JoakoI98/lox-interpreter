# 🌟 Lox Interpreter

A complete implementation of the **Lox programming language** interpreter written in Rust.

![Rust](https://img.shields.io/badge/language-Rust-orange.svg)
![Version](https://img.shields.io/badge/version-0.1.0-blue.svg)
![License](https://img.shields.io/badge/license-MIT-green.svg)

## ✨ Features

### 🚀 Language Features
- **Variables** - Dynamic typing with `var` declarations
- **Functions** - First-class functions with closures and recursion
- **Classes** - Object-oriented programming with inheritance
- **Control Flow** - `if/else`, `while`, and `for` loops
- **Operators** - Arithmetic, comparison, logical, and assignment operators
- **Built-in Functions** - Native functions like `clock()` for system interaction
- **Error Handling** - Comprehensive error reporting with line numbers

### 🛠️ Technical Features
- **Multi-stage Processing** - Separate tokenization, parsing, evaluation, and execution phases
- **Tree-walking Interpreter** - Direct AST evaluation with proper scoping
- **Procedural Macros** - Custom `ast_leaf` macros for clean AST generation
- **Robust Error Handling** - Proper exit codes and detailed error messages
- **Memory Safe** - Written in Rust with zero unsafe code

## 📦 Installation

### Prerequisites
- Rust 1.80 or higher
- Cargo (comes with Rust)

### Build from Source
```bash
git clone <repository-url>
cd lox-interpreter
cargo build --release
```

The compiled binary will be available at `target/release/codecrafters-interpreter`.

## 🎯 Usage

The interpreter supports four different modes of operation:

### 1. Run a Lox Program
Execute a complete Lox program:
```bash
cargo run -- run program.lox
```

### 2. Evaluate an Expression
Evaluate a single expression and print the result:
```bash
cargo run -- evaluate expression.lox
```

### 3. Parse and Display AST
Parse source code and display the abstract syntax tree:
```bash
cargo run -- parse program.lox
```

### 4. Tokenize Source Code
Break source code into tokens for analysis:
```bash
cargo run -- tokenize program.lox
```

## 📝 Lox Language Examples

### Variables and Expressions
```lox
var greeting = "Hello";
var name = "World";
print greeting + ", " + name + "!";

var x = 10;
var y = 20;
print x + y * 2; // 50
```

### Functions
```lox
fun fibonacci(n) {
    if (n <= 1) return n;
    return fibonacci(n - 2) + fibonacci(n - 1);
}

print fibonacci(10); // 55
```

### Classes and Inheritance
```lox
class Animal {
    speak() {
        print "Some generic animal sound";
    }
}

class Dog < Animal {
    speak() {
        print "Woof!";
    }
}

var dog = Dog();
dog.speak(); // Woof!
```

### Control Flow
```lox
// For loop
for (var i = 0; i < 5; i = i + 1) {
    print i;
}

// While loop
var counter = 0;
while (counter < 3) {
    print "Counter: " + counter;
    counter = counter + 1;
}

// Conditional statements
var age = 25;
if (age >= 18) {
    print "Adult";
} else {
    print "Minor";
}
```

### Built-in Functions
```lox
print clock(); // Current time in seconds since Unix epoch
```

## 🏗️ Architecture

The interpreter is structured in several key modules:

### 📁 Project Structure
```
src/
├── main.rs              # Entry point and command handling
├── commands/            # Command implementations (run, parse, etc.)
├── tokenizer/           # Lexical analysis and token generation
├── syntax_analysis/     # Parser and AST generation
├── evaluation/          # Runtime evaluation and execution
├── error/              # Unified error handling
└── common/             # Shared utilities

ast_leaf/               # Procedural macro for AST generation
├── src/
│   ├── lib.rs
│   ├── struct_parser.rs
│   └── attribute_parser/
```

### 🔄 Processing Pipeline

1. **Tokenization** - Source code → Tokens
2. **Parsing** - Tokens → Abstract Syntax Tree (AST)
3. **Resolution** - Variable binding and scope analysis
4. **Evaluation** - AST → Runtime values and execution

### 🎭 Key Components

- **Scanner/Tokenizer** - Converts source code into tokens
- **Recursive Descent Parser** - Builds AST from tokens using grammar rules
- **Tree-walking Evaluator** - Directly interprets the AST
- **Runtime Environment** - Manages variables, functions, and scope
- **Native Functions** - Built-in functionality like `clock()`

## 🔧 Development

### Running Tests
```bash
cargo test
```

### Building Documentation
```bash
cargo doc --open
```

### Development Mode
```bash
cargo run -- run examples/example.lox
```

## 📊 Language Grammar

The Lox language follows this grammar (simplified):

```ebnf
program        → declaration* EOF ;
declaration    → classDecl | funDecl | varDecl | statement ;
classDecl      → "class" IDENTIFIER ( "<" IDENTIFIER )? "{" function* "}" ;
funDecl        → "fun" function ;
varDecl        → "var" IDENTIFIER ( "=" expression )? ";" ;
statement      → exprStmt | forStmt | ifStmt | printStmt | returnStmt | whileStmt | block ;
expression     → assignment ;
assignment     → IDENTIFIER "=" assignment | logic_or ;
logic_or       → logic_and ( "or" logic_and )* ;
logic_and      → equality ( "and" equality )* ;
equality       → comparison ( ( "!=" | "==" ) comparison )* ;
comparison     → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
term           → factor ( ( "-" | "+" ) factor )* ;
factor         → unary ( ( "/" | "*" ) unary )* ;
unary          → ( "!" | "-" ) unary | call ;
call           → primary ( "(" arguments? ")" | "." IDENTIFIER )* ;
primary        → "true" | "false" | "nil" | "this" | NUMBER | STRING | IDENTIFIER | "(" expression ")" | "super" "." IDENTIFIER ;
```

## 🤝 Contributing

Contributions are welcome! Please feel free to:

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 📜 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- **Robert Nystrom** - Author of "Crafting Interpreters" and creator of the Lox language
- **Rust Community** - For the amazing tools and ecosystem

## 📚 Resources

- [Crafting Interpreters Book](https://craftinginterpreters.com/)
- [Rust Documentation](https://doc.rust-lang.org/)

---

*Happy interpreting! 🎉*
