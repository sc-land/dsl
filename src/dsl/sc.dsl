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
anatomy  = { bug | totem }
behavior = { assign | beat | trace }
// ========================================
// ANATOMY - TOTEM DEFINITIONS
// ========================================
totem    =  { "totem" ~ i ~ insignia ~ i ~ aspect+ ~ i ~ "end" }
insignia = @{ specie }
aspect   =  { emblem ~ i ~ zoo? ~ i }
emblem   = @{ specie | primor }
zoo      =  { "(" ~ i ~ (plain | signed) ~ i ~ ")" }
plain    =  { specie ~ (i ~ "," ~ i ~ specie)* }
signed = { biome ~ (i ~ "," ~ i ~ biome)* }
biome  = { primor ~ i ~ ":" ~ i ~ specie }
primor = @{ tag }
// ========================================
// ANATOMY - BUG DEFINITIONS
// ========================================
bug  = { "bug" ~ i ~ specie ~ i ~ (gene | ethics)* ~ i ~ "end" }
gene = { "gene" ~ i ~ tag ~ i ~ specie ~ i }

// Ethics (functions/methods) - suporte a todos os casos
ethics           =  { ethics_head_body | ethics_head }
ethics_head      = _{ (ethics_d | ethics_c | ethics_b | ethics_a) ~ i }
ethics_head_body = _{ ethics_head ~ ethics_body }
ethics_a         = _{ t_ethics ~ i ~ tag }
ethics_b         = _{ t_ethics ~ i ~ tag ~ i ~ feedback }
ethics_c         = _{ t_ethics ~ i ~ tag ~ i ~ signature }
ethics_d         = _{ t_ethics ~ i ~ tag ~ i ~ signature ~ i ~ feedback }
t_ethics         = _{ "ethics" }
signature        =  { "(" ~ i ~ ethics_binds? ~ i ~ ")" }
feedback         = _{ specie }
ethics_body      = _{ i ~ matrix ~ i ~ nucleus_ends ~ i }

// ========================================
// CODE BLOCKS
// ========================================
nucleus_ends = _{ "end" }
matrix       =  { signal+ }
signal       =  { i ~ (behavior) ~ i }

// ========================================
// CONTROL FLOW STATEMENTS
// ========================================
beat = { sprout | swirl | crawl | nectar }

// If statement
sprout       =  {
    sprout_start ~ condition ~ i ~ matrix ~ splice* ~ den? ~ if_ends
}
sprout_start = _{ "if" ~ i }
splice       =  { splice_start ~ condition ~ i ~ matrix }
splice_start = _{ "elsif" ~ i }
den          =  { den_start ~ matrix }
den_start    = _{ "else" ~ i }
if_ends      = _{ i ~ "end" }

// While loop
swirl       =  {
    while_start ~ condition ~ i ~ matrix ~ while_ends
}
while_start = _{ "while" ~ i }
while_ends  = _{ i ~ "end" }

// For loop
crawl       =  {
    crawl_start ~ each ~ crawl_in ~ trace ~ i ~ matrix ~ crawl_ends
}
crawl_start = _{ "for" ~ i }
each        =  { tag ~ i }
crawl_in    = _{ "in" ~ i }
crawl_ends  = _{ i ~ "end" }

// ========================================
// EXPRESSIONS & ASSIGNMENTS
// ========================================
condition = _{ trace }
nectar    =  { i ~ "return" ~ i ~ trace }
assign    =  { tag ~ i ~ "=" ~ i ~ trace }

// ========================================
// OBJECT-ORIENTED PROGRAMMING
// ========================================
trace   = { emitter ~ course* }
emitter = { literal | self_ref | specie | tag }
course  = { catalysis | carrier }

// Method calls and property access
catalysis = { "." ~ tag ~ carrier? }
carrier   = { "(" ~ transport? ~ ")" }
transport = { binds | sequence }

// Function parameters and arguments
binds        =  { bind ~ (i ~ "," ~ i ~ bind)* }
sequence     =  { trace ~ (i ~ "," ~ i ~ trace)* }
bind         =  { tag ~ i ~ ":" ~ i ~ trace }
ethics_binds = _{ ethics_bind ~ (i ~ "," ~ i ~ ethics_bind)* }
ethics_bind  =  { tag ~ i ~ ":" ~ i ~ specie }

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
    | "totem"
    ) ~ !(ASCII_ALPHANUMERIC | "_")
}
