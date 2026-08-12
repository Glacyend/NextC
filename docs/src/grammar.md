# Grammar

The following grammar rules are given in EBNF.

> [!Warning]
>
> NextC is currently in the early design and development phase; syntax rules may be added, removed, or modified at any time. Until NextC 1.0 is released, this document is subject to significant change.

## Keywords

All NextC keywords are:

```text
as      break       byte        case        char
const   constexpr   continue    default     delete
else    enum        external    false       float
for     foreach     if          import      int
long    loop        new         nullptr     return
sbyte   short       slice       string      struct
switch  true        typedef     uint        ulong
union   unsafe      ushort      void        while
```

## Type Identifiers

> [!Tip]
>
> Pointer types incorporate `*` into the type identifier. This means the following declares three pointer variables:
>
> ```nextc
> int* a, b, c;
> ```

```ebnf
TypeIdent ::= "(" TypeIdent ")"
            | ("const")? TypeName
            | PointerType ("const")?
            | FunctionType ("const")?
            | "void"

TypeName ::= BasicType
           | ArrayType
           | SliceType
           | "string"
           | (Identifier "::")* Identifier

BasicType ::= "byte"
            | "sbyte"
            | "short"
            | "ushort"
            | "int"
            | "uint"
            | "long"
            | "ulong"
            | "ssize"
            | "usize"
            | "char"
            | "bool"
            | "float"
            | "double"

ArrayType ::= TypeIdent "[" Expression "]"

SliceType ::= "slice" "[" TypeIdent "]"

PointerType ::= TypeIdent "*"

FunctionTypeArgList ::= TypeIdent ("," TypeIdent)* (",")?
FunctionType ::= TypeIdent "(" (FunctionTypeArgList)? ")" "*"
```

## Code Labels

> [!Warning]
>
> NextC has no `goto` statement. Code labels are only allowed inside loops and are used to break out of nested loops.

```ebnf
CodeTag ::= "@" Identifier;
```

## Expressions

Operator precedence in NextC generally follows that of C.

> [!Tip]
>
> In NextC, assignment (`=`), increment (`++`), and decrement (`--`) are statements, not expressions. However, their precedence is the same as in C.
>
> NextC does not have the comma operator.
>
> Parentheses can change precedence; when building the AST, the expression inside parentheses becomes a separate node, so it is not a separate grammar rule.
>
> NextC requires parentheses around the conditional operator, so it has the highest precedence.

### Literals

Basically the same as in C.

For integer literals (`IntegerLiteral`):

- Default type is `int`; no automatic conversion; type suffixes must be added manually.
- Prefixes `0x`, `0o`, `0b` denote hexadecimal, octal, and binary respectively.
- Suffixes `b` and `sb` denote `byte` and `sbyte` respectively.
- Suffixes `s` and `l` denote `short` and `long` respectively.
- Suffixes `us`, `u`, and `ul` denote `ushort`, `uint`, and `ulong` respectively.
- Signs belong to unary expressions, not to the literal.
- Underscores (`_`) are allowed anywhere except at the beginning or end, in any number and position.

For floating‑point literals (`FloatLiteral`):

- Default type is `double`; no automatic conversion; type suffixes must be added manually.
- Syntax is essentially the same as in C.
- Underscores (`_`) are allowed anywhere except at the beginning or end, in any number and position.
- Suffix `f` denotes `float`.

For boolean literals (`BoolLiteral`):

- `true` and `false` denote truth values.

For string literals (`StringLiteral`):

- Syntax is essentially the same as in C.
- `r#"..."` denotes a raw string: no escape sequences.
- `"""..."""` denotes a multi‑line string. Except for the first line, common indentation of the remaining lines is stripped. If the opening `"""` is immediately followed by a newline, that newline is ignored, and the common indentation of the first line and the remaining lines is stripped.

Null pointer literal (`NullPtrLiteral`):

- `nullptr` denotes a null pointer.
- Since NextC does not allow `void*` in safe code, `nullptr` cannot be used alone; its concrete pointer type must be deducible. In variable definitions and comparison expressions, the concrete pointer type can be determined.

All literals:

```ebnf
Literal ::= IntegerLiteral
          | FloatLiteral
          | BoolLiteral
          | StringLiteral
          | NullPtrLiteral
```

### Primary Expressions

```ebnf
Expression ::= "(" Expression ")"
             | Call
             | Index
             | Get
             | PtrGet
             | Conditional
             | ArrayInitList
             | Literal
             | New

ArgumentList ::= Expression ("," Expression)* (",")?
Call ::= Expression "(" ArgumentList? ")"
Index ::= Expression "[" Expression "]"
Get ::= Expression "." Identifier
PtrGet ::= Expression "->" Identifier
Conditional ::= "(" Expression "?" Expression ":" Expression ")"
ArrayInitList ::= "{" ArgumentList "}"
New ::= "new" (TypeIdent | (TypeIdent "[" Expression "]"))
```

### Arithmetic Operations

```ebnf
Factor ::= TypeConv (("*" | "/" | "%") TypeConv)*
Term ::= Factor (("+" | "-") Factor)*
```

### Bitwise Operations

```ebnf
Shift ::= Term (("<<" | ">>") Term)*
BitAnd ::= Equality ("&" Equality)*
BitXor ::= BitAnd ("^" BitAnd)*
BitOr ::= BitXor ("|" BitXor)*
```

### Relational Operations

```ebnf
Comparison ::= Shift (("<" | "<=" | ">" | ">=") Shift)*
Equality ::= Comparison (("==" | "!=") Comparison)*
```

### Logical Operations

```ebnf
LogicalNot ::= "!" Expression
LogicalAnd ::= BitOr ("&&" BitOr)*
LogicalOr ::= LogicalAnd ("||" LogicalAnd)*
```

### Unary Operations

```ebnf
Unary ::= Expression
        | "+" Expression
        | "-" Expression
        | LogicalNot
        | Deref
        | Pointer

Deref ::= "*" Expression
Pointer ::= "&" Expression
```

### Type Conversions

```ebnf
TypeConv ::= Unary ("as" (BasicType | PointerType))*
```

## Statements

NextC requires that certain statements end with a semicolon (`;`).

Statement framework:

```ebnf
GlobalStatement ::= External
                  | ("pub")? FunctionDefine
                  | ("pub")? VariableDefine
                  | Import
                  | ("pub")? Typedef
                  | ("pub")? ConstantDefine
                  | ("pub")? Struct
                  | ("pub")? Enum
                  | ("pub")? Union

Statement ::= ExprStmt
            | Assignment
            | Increase
            | Decrease
            | VariableDefine
            | ConstantDefine
            | Return
            | IfElse
            | Loop
            | While
            | DoWhile
            | For
            | ForEach
            | Break
            | Continue
            | Switch
            | Delete
```

### Assignment Statement

```ebnf
AssignmentSymbol ::= "="
                   | "*="
                   | "/="
                   | "%="
                   | "+="
                   | "-="
                   | "<<="
                   | ">>="
                   | "&="
                   | "^="
                   | "|="

AssignmentWithoutSemicolon ::= ((Get | PtrGet | Index | Deref) AssignmentSymbol)+ Expression
Assignment ::= AssignmentWithoutSemicolon ";"
```

Increment/Decrement Statement

> [!Tip]
>
> Prefix and suffix forms are both allowed and have the same semantics.

```ebnf
IncreaseWithoutSemicolon ::= "++" (Get | PtrGet | Index | Deref)
                           | (Get | PtrGet | Index | Deref) "++"
Increase ::= IncreaseWithoutSemicolon ";"

DecreaseWithoutSemicolon ::= "--" (Get | PtrGet | Index | Deref)
                           | (Get | PtrGet | Index | Deref) "--"
Decrease ::= DecreaseWithoutSemicolon ";"
```

### Expression Statement

```ebnf
ExprStmt ::= Expression ";"
```

### Variable Definition

```ebnf
VariableDefine ::= TypeIdent Identifier ("=" Expression)? ("," Identifier ("=" Expression)?)* ";"
```

### Constant Definition

```ebnf
ConstantDefine ::= "constexpr" TypeIdent Identifier "=" Expression ("," Identifier "=" Expression)* ";"
```

### Function Definition

```ebnf
FunctionDeclaration ::= TypeIdent Identifier "(" (FunctionDefineArgList)? ")"

FunctionDefineArgList ::= (TypeIdent Identifier) ("," (TypeIdent Identifier))* (",")?

FunctionDefine ::= FunctionDeclaration "{" (Statement)* "}"
```

### External C Interface

```ebnf
External ::= "external" "unsafe" FunctionDeclaration ";"
```

### Struct

```ebnf
Members ::= (TypeIdent Identifier ";")+

Struct ::= "struct" Identifier "{" Members "}"
```

### Enum

```ebnf
Enum ::= "enum" Identifier (":" TypeIdent)
         "{" Identifier ("," Identifier)* (",")? "}"
```

### Union

```ebnf
Union ::= "union" Identifier "{" Members "}"
```

### Type Alias

```ebnf
TypeDef ::= "typedef" Identifier "=" TypeIdent ";"
```

### Return Statement

```ebnf
Return ::= "return" (Expression)? ";"
```

### Conditional Control Flow

```ebnf
IfElse ::= "if" "(" Expression ")"
           "{" (Statement)* "}"
           (
                "else" "if" "(" Expression ")"
                "{" (Statement)* "}"
           )*
           (
                "else"
                "{" (Statement)* "}"
           )?
```

### `loop` Loop

```ebnf
Loop ::= "loop" (CodeTag)? "{" (Statement)* "}"
```

### `while` Loop

```ebnf
While ::= "while" (CodeTag)? "(" Expression ")"
          "{" (Statement)* "}"
```

### `do-while` Loop

> [!Tip]
>
> Because NextC requires that clause bodies must be blocks (with braces), `do-while` does not require a semicolon. If a semicolon is present, it is treated as an empty statement and the compiler will issue a warning.

```ebnf
DoWhile ::= "do" (CodeTag)? "{" (Statement)* "}"
            "while" "(" Expression ")"
```

### `for` Loop

```ebnf
Incremental ::= AssignWithoutSemicolon
              | IncreaseWithoutSemicolon
              | DecreaseWithoutSemicolon

For ::= "for" (CodeTag)? "("
        (VariableDefine | ";")
        (Expression)? ";"
        (Expression | Incremental )? ")"
        "{" (Statement)* "}"
```

### `foreach` Loop

```ebnf
ForEach ::= "foreach" (CodeTag)? "(" TypeIdent Identifier ":" Expression ")"
            "{" (Statement)* "}"
```

### `break` Statement

```ebnf
Break ::= "break" (CodeTag)? ";"
```

### `continue` Statement

```ebnf
Continue ::= "continue" (CodeTag)? ";"
```

### `switch` Statement

> [!Tip]
>
> To keep `syntax` concise and modern, `case` and `default` clauses in NextC do not use colons, and the `default` clause must be the last one.

```ebnf
SwitchCase ::= "case" Expression "{" (Statement)* "}"
SwitchDefault ::= "default" "{" (Statement)* "}"

Switch ::= "switch" "(" Expression ")"
           "{" (SwitchCase)* (SwitchDefault)? "}"
```

### `delete` Statement

```ebnf
Delete ::= "delete" Identifier ("," Identifier)* ";"
```

## Comments

Basically the same as in C.

- `//` starts a single‑line comment.

- `/* ... */` denotes multi‑line or inline comments. Unlike C, these comments can be nested, for example:

  ```nextc
  /* outer /* inner */ outer */
  ```

- `///` denotes a documentation comment.

- Comments may be used inside macros.

## Preprocessor Directives and Macros

Under design.

Initial idea: distinguish value macros from function macros. Value macros are only used for conditional compilation and cannot appear in code. Function macros are similar to Rust procedural macros but simpler.
