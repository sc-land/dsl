# SC DSL

## Grammar

```pest
sc     = { SOI ~ fly ~ EOI }
fly    = { (pog ~ nl)+ }
pog    = { genome+ }
genome = { ddl | dml }

ddl = { bug }
dml = { assign | oop }

bug  = { "bug" ~ s ~ specie ~ nl ~ gene* }
gene = { s ~ "gene" ~ s ~ tag ~ s ~ specie ~ NEWLINE* }

assign = { tag ~ s ~ "=" ~ s ~ oop }

oop      = { emitter ~ trail* }
emitter  = { specie | tag | literal }
trail    = { catalysis | carrier }

catalysis = { "." ~ tag ~ carrier? }
carrier   = { "(" ~ s ~ (binds | sequence)? ~ s ~ ")" }

binds    = { bind ~ (s ~ "," ~ s ~ bind)* }
sequence = { oop ~ (s ~ "," ~ s ~ oop)* }
bind     = { tag ~ s ~ ":" ~ s ~ oop }

literal = _{ bit | hex | decimal | int | str }
bit     = @{ "0b" ~ ASCII_BIN_DIGIT+ }
hex     = @{ "0x" ~ ASCII_HEX_DIGIT+ }
int     = @{ "-"? ~ ASCII_DIGIT+ }
str     = @{ "\"" ~ (!("\"") ~ ANY)* ~ "\"" }
decimal = @{ "-"? ~ ASCII_DIGIT+ ~ "." ~ ASCII_DIGIT+ }

tag    = { ASCII_ALPHA_LOWER ~ (ASCII_ALPHANUMERIC | "_")* }
specie = { ASCII_ALPHA_UPPER ~ (ASCII_ALPHANUMERIC)+ }

nl = _{ NEWLINE* }
s  = _{ (" " | "\t" | "\n" )* }

```

## AST

```puml
@startuml
!theme crt-amber

hide circle

package ast {
    class SC {
        +raw Str
        +fly Fly
    }

    class Fly {
        +raw Str
        +pogs Pog[]
    }

    class Pog {
        +raw Str
        +genome Genome[]
    }

    enum Genome {
        DDL
        DML
    }

    enum DML {
        Assign
        Oop
    }

    class Assign {
        +raw Str
    }

    class Oop {
        +raw Str

    }

    class DDL {
        +raw Str
        +bug Bug
    }

    class Bug {
        +raw Str
        +specie Str
    }
}

package parser {
    class Parser
    class Tree
}

parser.Tree --> ast.SC
parser.Tree --> parser.Parser

ast.SC --> ast.Fly

ast.Fly --> ast.Pog

ast.Pog --> ast.Genome

ast.Genome --> ast.DDL
ast.Genome --> ast.DML

ast.DML --> ast.Assign
ast.DML --> ast.Oop

ast.DDL --> ast.Bug

@enduml
```
