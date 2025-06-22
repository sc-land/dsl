// ========================================
// SC GRAMMAR
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
anatomy  = { bug | totem }
behavior = { assign | beat | trace }
// ========================================
// ANATOMY - TOTEM DEFINITIONS
// ========================================
totem       =  { "totem" ~ i ~ insignia ~ i ~ folklore+ ~ i ~ "end" }
insignia    = @{ specie }
folklore    =  { prophecy ~ i ~ runes? ~ i }
prophecy    = @{ specie | primor }
runes       =  { "(" ~ i ~ (whispers | invocations) ~ i ~ ")" }
whispers    =  { specie ~ (i ~ "," ~ i ~ specie)* }
invocations =  { entity ~ (i ~ "," ~ i ~ entity)* }
entity      =  { primor ~ i ~ ":" ~ i ~ specie }

// ========================================
// ANATOMY - BUG DEFINITIONS
// ========================================
bug  = { "bug" ~ i ~ specie ~ i ~ (gene | ethics)* ~ i ~ "end" }
gene = { "gene" ~ i ~ primor ~ i ~ specie ~ i ~ seals? ~ i }
seals = { trace ~ (i ~ "," ~ i ~ trace)* }

// Ethics (functions/methods) - suporte a todos os casos
ethics           =  { ethics_head_body | ethics_head }
ethics_head      = _{ "ethics" ~ i ~ primor ~ i ~ (signature ~ i ~ feedback | signature | feedback)? ~ i }
ethics_head_body = _{ ethics_head ~ ethics_body }
signature        =  { "(" ~ i ~ (ethics_binds |  sequence  )? ~ i ~ ")" }
feedback         = @{ specie }
ethics_body      = _{ i ~ matrix ~ i ~ "end" ~ i }

// ========================================
// CODE BLOCKS
// ========================================
matrix = { signal+ }
signal = { i ~ (behavior) ~ i }

// ========================================
// CONTROL FLOW STATEMENTS
// ========================================
beat = { sprout | swirl | crawl | nectar }

// If statement
sprout = {
    "if" ~ i ~ condition ~ i ~ matrix ~ splice* ~ den? ~ i ~ "end"
}
splice = { "elsif" ~ i ~ condition ~ i ~ matrix }
den    = { "else" ~ i ~ matrix }

// While loop
swirl = {
    "while" ~ i ~ condition ~ i ~ matrix ~ i ~ "end"
}

// For loop
crawl = {
    "for" ~ i ~ primor ~ i ~ "in" ~ i ~ trace ~ i ~ matrix ~ i ~ "end"
}

// ========================================
// EXPRESSIONS & ASSIGNMENTS
// ========================================
condition = _{ trace }
nectar    =  { i ~ "return" ~ i ~ trace }
assign    =  { primor ~ i ~ "=" ~ i ~ trace }

// ========================================
// OBJECT-ORIENTED PROGRAMMING
// ========================================
trace   = { emitter ~ course* }
emitter = { literal | self_ref | specie | primor }
course  = { catalysis | carrier }

// Method calls and property access
catalysis = { "." ~ primor ~ carrier? }
carrier   = { "(" ~ transport? ~ ")" }
transport = { binds | sequence }

// Function parameters and arguments
binds        =  { bind ~ (i ~ "," ~ i ~ bind)* }
sequence     =  { trace ~ (i ~ "," ~ i ~ trace)* }
bind         =  { primor ~ i ~ ":" ~ i ~ trace }
ethics_binds =  { ethics_bind ~ (i ~ "," ~ i ~ ethics_bind)* }
ethics_bind  =  { primor ~ i ~ ":" ~ i ~ specie }

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
primor   = @{ !reserved ~ ASCII_ALPHA_LOWER ~ (ASCII_ALPHANUMERIC | "_")* }
specie   = @{ ASCII_ALPHA_UPPER ~ (ASCII_ALPHANUMERIC | "_")* }
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
    | "totem"
    ) ~ !(ASCII_ALPHANUMERIC | "_")
}
