# 语法规则

以下语法规则采用 EBNF 表示。

> [!Warning]
>
> 目前 NextC 处于初期设计和开发阶段，随时可能会加入、删除或修改一些语法规则。直到 NextC 1.0 发布之前，此文档都有很大可能进行改动。

## 关键字

NextC 所有的关键字如下：

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

## 类型标识符

> [!Tip]
>
> 指针类型将 `*` 纳入到了类型标识符中，这意味着这段代码将声明三个指针变量：
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

## 代码标签

> [!Warning]
>
> NextC 中没有 `goto` 语句，代码标签也只允许在循环中使用，用于跳出多层循环！

```ebnf
CodeTag ::= "@" Identifier;
```

## 表达式

在 NextC 中，运算符的优先级一般与 C 相同。

> [!Tip]
>
> 在 NextC 中，赋值、自增（`++`）和自减（`--`）都是语句，不是表达式。但是其优先级与 C 相同。
>
> NextC 中没有逗号表达式。
>
> 括号可以改变优先级，构建 AST 时括号内的表达式单独成一节点，所以不单独成一条语法规则。
>
> NextC 要求条件运算符周围必须打括号，因此优先级最高。

### 字面量

基本与 C 语言一致。

对于整型字面量（`IntegerLiteral`）：

- 默认 `int` 类型，不会自动转换类型，需要手动加类型后缀。
- 前缀 `0x`、`0o`、`0b` 分别表示十六进制、八进制和二进制。
- 后缀 `b` 和 `sb` 分别表示 `byte` 类型和 `sbyte` 类型。
- 后缀 `s` 和 `l` 分别表示 `short` 和 `long` 类型。
- 后缀 `us`、`u` 和 `ul` 分别表示 `ushort`、`uint` 和 `ulong` 类型。
- 若有正负号，不属于字面量，而属于一元表达式。
- 除开头和结尾外，允许在整型字面量中使用任意数量和任意位置的下划线（`_`）。

对于浮点型字面量（`FloatLiteral`）：

- 默认 `double` 类型，不会自动转换类型，需要手动加类型后缀。
- 语法与 C 基本一致。
- 除开头和结尾外，允许在浮点型字面量中使用任意数量和任意位置的下划线（`_`）。
- 后缀 `f` 表示 `float` 类型。

对于布尔型字面量（`BoolLiteral`）：

- `true` 和 `false` 分别表示真值和假值。

对于字符串字面量（`StringLiteral`）：

- 语法与 C 基本一致。
- `r#"..."` 表示原始字符串：无转义字符。
- `"""..."""` 表示多行字符串。除第一行外，剩余行的公共缩进将被忽略。若开头的 `"""` 后立即换行，则该换行符忽略，且第一行与剩余行的公共缩进将被忽略。

空指针字面量（`NullPtrLiteral`）：

- `nullptr` 关键字表示空指针。
- 由于 NextC 在安全状态下不允许使用 `void*`，所以 `nullptr` 不能单独使用，其必须能够被推导出具体指针类型。在变量定义、比较表达式中，都可以确定其具体指针类型。

所有的字面量有：

```ebnf
Literal ::= IntegerLiteral
          | FloatLiteral
          | BoolLiteral
          | StringLiteral
          | NullPtrLiteral
```

### 最高级表达式

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

### 算术运算

```ebnf
Factor ::= TypeConv (("*" | "/" | "%") TypeConv)*
Term ::= Factor (("+" | "-") Factor)*
```

### 位运算

```ebnf
Shift ::= Term (("<<" | ">>") Term)*
BitAnd ::= Equality ("&" Equality)*
BitXor ::= BitAnd ("^" BitAnd)*
BitOr ::= BitXor ("|" BitXor)*
```

### 关系运算

```ebnf
Comparison ::= Shift (("<" | "<=" | ">" | ">=") Shift)*
Equality ::= Comparison (("==" | "!=") Comparison)*
```

### 逻辑运算

```ebnf
LogicalNot ::= "!" Expression
LogicalAnd ::= BitOr ("&&" BitOr)*
LogicalOr ::= LogicalAnd ("||" LogicalAnd)*
```

### 一元运算

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

### 类型转换

```ebnf
TypeConv ::= Unary ("as" (BasicType | PointerType))*
```

## 语句

NextC 要求部分语句后必须有分号（`;`）。

语句语法框架如下：

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

### 赋值语句

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

### 自增/自减语句

> [!Tip]
>
> 符号用前缀或后缀都可以，语义上是一致的。

```ebnf
IncreaseWithoutSemicolon ::= "++" (Get | PtrGet | Index | Deref)
                           | (Get | PtrGet | Index | Deref) "++"
Increase ::= IncreaseWithoutSemicolon ";"

DecreaseWithoutSemicolon ::= "--" (Get | PtrGet | Index | Deref)
                           | (Get | PtrGet | Index | Deref) "--"
Decrease ::= DecreaseWithoutSemicolon ";"
```

### 表达式语句

```ebnf
ExprStmt ::= Expression ";"
```

### 定义变量

```ebnf
VariableDefine ::= TypeIdent Identifier ("=" Expression)? ("," Identifier ("=" Expression)?)* ";"
```

### 定义常量

```ebnf
ConstantDefine ::= "constexpr" TypeIdent Identifier "=" Expression ("," Identifier "=" Expression)* ";"
```

### 定义函数

```ebnf
FunctionDelcaration ::= TypeIdent Identifier "(" (FunctionDefineArgList)? ")"

FunctionDefineArgList ::= (TypeIdent Identifier) ("," (TypeIdent Identifier))* (",")?

FunctionDefine ::= FunctionDeclaration "{" (Statement)* "}"
```

### 外部 C 接口

```ebnf
External ::= "external" "unsafe" FunctionDeclaration ";"
```

### 结构体

```ebnf
Members ::= (TypeIdent Identifier ";")+

Struct ::= "struct" Identifier "{" Members "}"
```

### 枚举

```ebnf
Enum ::= "enum" Identifier (":" TypeIdent)
         "{" Identifier ("," Identifier)* (",")? "}"
```

### 联合体

```ebnf
Union ::= "union" Identifier "{" Members "}"
```

### 类型别名

```ebnf
TypeDef ::= "typedef" Identifier "=" TypeIdent ";"
```

### 返回语句

```ebnf
Return ::= "return" (Expression)? ";"
```

### 条件控制流

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

### `loop` 循环

```ebnf
Loop ::= "loop" (CodeTag)? "{" (Statement)* "}"
```

### `while` 循环

```ebnf
While ::= "while" (CodeTag)? "(" Expression ")"
          "{" (Statement)* "}"
```

### `do-while` 循环

> [!Tip]
>
> 由于 NextC 要求分句必须是块（带花括号），所以 `do-while` 后面可以不加分号。若加了分号，则将被视为空语句，编译器将发出警告。

```ebnf
DoWhile ::= "do" (CodeTag)? "{" (Statement)* "}"
            "while" "(" Expression ")"
```

### `for` 循环

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

### `foreach` 循环

```ebnf
ForEach ::= "foreach" (CodeTag)? "(" TypeIdent Identifier ":" Expression ")"
            "{" (Statement)* "}"
```

### `break` 语句

```ebnf
Break ::= "break" (CodeTag)? ";"
```

### `continue` 语句

```ebnf
Continue ::= "continue" (CodeTag)? ";"
```

### `switch` 语句

> [!Tip]
>
> 为了让语法更简洁和现代化，NextC 中的 `switch` 语句的 `case` 和 `default` 字句没有冒号，且 `default` 字句必须是最后一句。

```ebnf
SwitchCase ::= "case" Expression "{" (Statement)* "}"
SwitchDefault ::= "default" "{" (Statement)* "}"

Switch ::= "switch" "(" Expression ")"
           "{" (SwitchCase)* (SwitchDefault)? "}"
```

### `delete` 语句

```ebnf
Delete ::= "delete" Identifier ("," Identifier)* ";"
```

## 注释

与 C 基本一致。

- `//` 表示单行注释的开头。

- 使用 `/* ... */` 可以表示多行注释或行内注释。但是要注意，与 C 不同的是，这个注释可以嵌套，即你可以写：

  ```nextc
  /* outer /* inner */ outer */
  ```

- 使用 `///` 表示文档注释。

- 注释可以在宏中使用。


## 预编译指令与宏

正在设计中。

初步想法：区分值宏与函数宏。值宏只用于条件编译，不能用于代码中。函数宏与 Rust 的过程宏类似但更精简。
