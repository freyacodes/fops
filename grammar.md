# Grammar
This uses a variant of BNF

```
program                → statement* EOF ;

statement              → if_statement
                       | while_statement
                       | block_statement
                       | declaration_statement
                       | reassignment_statement
                       | expression_statement ;

if_statement           → "if" "(" expression ")" statement 
                         ( "else" "if" )*
                         ( "else" statement )? ;
while_statement        → "while" "(" expression ")" statement ;
block_statement        → "{" statement* "}" ;
declaration_statement  → "let" IDENTIFIER "=" expression ";" ;
reassignment_statement → IDENTIFIER "=" expression ";" ;
expression_statement   → expression ";" ;

---

expression     → logic_or ;
logic_or       → logic_and ( "||" logic_and )* ;
logic_and      → equality ( "&&" equality )* ;
equality       → comparison ( ( "!=" | "==" ) comparison )* ;
comparison     → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
term           → factor ( ( "-" | "+" ) factor )* ;
factor         → unary ( ( "/" | "*" ) unary )* ;
unary          → ( "!" | "-" ) unary
               | call ;
call           → IDENTIFIER "(" expression ")" ;
primary        → "true" | "false" | "nil"
               | NUMBER | STRING
               | "(" expression ")"
               | IDENTIFIER ;
```
