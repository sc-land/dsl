// ========================================
// SC-LAND DSL GRAMMAR
// ========================================

// ========================================
// MAIN STRUCTURE
// ========================================
sc     = { SOI ~ fly ~ EOI }
fly    = { strand }
strand = { (genome ~ i)+ }
genome = { anatomy | behavior }

// ========================================
// GENOME TYPES
// ========================================
anatomy  = { bug }
behavior = { assign | statement | oop  }
// ========================================
// ANATOMY - BUG DEFINITIONS
// ========================================
bug         =  { "bug" ~ i ~ specie ~ i ~ (gene|ethics)* ~ i ~ "end" }
gene        =  { "gene" ~ i ~ tag ~ i ~ specie ~ i }

// Ethics (functions/methods) - suporte a todos os casos
ethics = { ethics_head_body | ethics_head }
ethics_head = _{ (ethics_d | ethics_c | ethics_b | ethics_a) ~ i }
ethics_head_body = _{ ethics_head ~ ethics_body }
ethics_a = _{ t_ethics ~ i ~ tag  }
ethics_b = _{ t_ethics ~ i ~ tag ~ i ~ feedback }
ethics_c = _{ t_ethics ~ i ~ tag ~ i ~ signature }
ethics_d = _{ t_ethics ~ i ~ tag ~ i ~ signature ~ i ~ feedback }
t_ethics = _{ "ethics" }
signature = { "(" ~ i ~ ethics_binds? ~ i ~ ")" }
feedback = _{ specie }
ethics_body = _{ i ~ matrix ~ i ~ nucleus_ends ~ i }

// ========================================
// CODE BLOCKS
// ========================================
nucleus       =  { nucleus_start ~ i ~ matrix ~ i ~ nucleus_ends }
nucleus_start = _{ "do" }
nucleus_ends  = _{ "end" }
matrix        =  { signal+ }
signal        =  { i ~ ( behavior ) ~ i }

// ========================================
// CONTROL FLOW STATEMENTS
// ========================================
statement = { if | while | for | return }

// If statement
if = {
    if_start ~ condition ~ i ~ matrix ~ elsif* ~ else? ~ if_ends
}
if_start    = _{ "if" ~ i }
elsif       =  { elsif_start ~ condition ~ i ~ matrix }
elsif_start = _{ "elsif" ~ i }
else        =  { else_start ~ matrix }
else_start  = _{ "else" ~ i }
if_ends     = _{ i ~ "end" }

// While loop
while       =  {
    while_start ~ condition ~ i ~ matrix ~ while_ends
}
while_start = _{ "while" ~ i }
while_ends  = _{ i ~ "end" }


// For loop
for       =  {
    for_start ~ each ~ in ~ oop ~ i ~ matrix ~ for_ends
}
for_start = _{ "for" ~ i }
each      = { tag ~ i }
in        = _{ "in" ~ i }
for_ends  = _{ i ~ "end" }

// ========================================
// EXPRESSIONS & ASSIGNMENTS
// ========================================
condition  = { oop }
return     = { i ~ "return" ~ i ~ oop }
assign     = { tag ~ i ~ "=" ~ i ~ oop }

// ========================================
// OBJECT-ORIENTED PROGRAMMING
// ========================================
oop     = { emitter ~ trail* }
emitter = { specie | tag | literal }
trail   = { catalysis | carrier }

// Method calls and property access
catalysis = { "." ~ tag ~ carrier? }
carrier   = { "(" ~ transport? ~ ")" }
transport = { binds | sequence }

// Function parameters and arguments
binds    = { bind ~ (i ~ "," ~ i ~ bind)* }
sequence = { oop ~ (i ~ "," ~ i ~ oop)* }
bind     = { tag ~ i ~ ":" ~ i ~ oop }
ethics_binds = _{ ethics_bind ~ (i ~ "," ~ i ~ ethics_bind)* }
ethics_bind  = { tag ~ i ~ ":" ~ i ~ specie }

// ========================================
// LITERALS
// ========================================
literal =  { bit | hex | decimal | int | str }
bit     = @{ "0b" ~ ASCII_BIN_DIGIT+ }
hex     = @{ "0x" ~ ASCII_HEX_DIGIT+ }
decimal = @{ "-"? ~ ASCII_DIGIT+ ~ "." ~ ASCII_DIGIT+ }
int     = @{ "-"? ~ ASCII_DIGIT+ }
str     = @{ "\"" ~ (!("\"") ~ ANY)* ~ "\"" }

// ========================================
// IDENTIFIERS
// ========================================
tag      =  { !reserved ~ ASCII_ALPHA_LOWER ~ (ASCII_ALPHANUMERIC | "_")* }
specie   =  { ASCII_ALPHA_UPPER ~ (ASCII_ALPHANUMERIC | "_")* }
self_ref = @{ "$" }

// ========================================
// WHITESPACE & UTILITIES
// ========================================
i = _{ (" " | "\t" | NEWLINE)* }

// ========================================
// RESERVED KEYWORDS
// ========================================

reserved = {
    ( "if"
    | "elsif"
    | "else"
    | "end"
    | "do"
    | "while"
    | "for"
    | "in"
    | "return"
    | "ethics"
    | "gene"
    | "bug"
    ) ~ !(ASCII_ALPHANUMERIC | "_")
}
