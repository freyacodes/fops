# Grammar
This uses a variant of BNF

```
program                → statement* EOF ;

statement              → block_statement
                       | declaration_statement
                       | reassignment_statement
                       | expression_statement ;

block_statement        → "{" statement* "}" ;
declaration_statement  → "let" IDENTIFIER "=" expression ";" ;
reassignment_statement → IDENTIFIER "=" expression ";" ;
expression_statement   → expression ";" ;

---

expression     → equality ;
function_call  → IDENTIFIER "(" ( expression ")" ;
equality       → comparison ( ( "!=" | "==" ) comparison )* ;
comparison     → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
term           → factor ( ( "-" | "+" ) factor )* ;
factor         → unary ( ( "/" | "*" ) unary )* ;
unary          → ( "!" | "-" ) unary
               | primary ;
primary        → "true" | "false" | "nil"
               | NUMBER | STRING
               | "(" expression ")"
               | IDENTIFIER ;
```
