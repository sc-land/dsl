# SC DSL

Uma Domain-Specific Language (DSL) implementada em Rust usando Pest para parsing.

## 📖 Visão Geral

Este projeto implementa um DSL com suporte a:
- **Anatomy**: Definição de estruturas (`bug`)
- **Behavior**: Operações e atribuições
- **Literais**: Suporte a inteiros, hexadecimais, binários, decimais e strings
- **Operações**: Chamadas de métodos e atribuições

## 🚀 Instalação e Uso

### Pré-requisitos
- Rust 1.75+ (edition 2024)

### Compilar o projeto
```bash
cargo build
```

### Executar os testes
```bash
cargo test
```

## 📝 Sintaxe Básica

### Definições (Anatomy)
```
bug Cat
bug Dog
```

### Operações (Behavior)
```
variable = Value.method
result = Class.call(param: value)
```

### Literais suportados
```
42          # inteiro
0xFF        # hexadecimal
0b1010      # binário
3.14        # decimal
"hello"     # string
```

### Operações
```
Class.method
Object.call(arg1, arg2)
method(param: value)
```

## 🏗️ Estrutura do Projeto

- **Grammar**: A gramática está definida em [`src/dsl/sc.dsl`](src/dsl/sc.dsl)
- **AST**: Árvore sintática abstrata em [`src/dsl/ast/`](src/dsl/ast/)
- **Parser**: Implementação do parser em [`src/dsl/parser/`](src/dsl/parser/)
- **Tests**: Testes unitários e de integração em [`tests/`](tests/)

## 📊 Arquitetura

O DSL é organizado hierarquicamente:
```
SC → Fly → Strand → Genome → {Anatomy | Behavior}
```

- **SC**: Nó raiz da sintaxe
- **Fly**: Contêiner de blocos de código
- **Strand**: Agrupamento de genomas
- **Genome**: Unidade básica (Anatomy ou Behavior)

## 🧪 Exemplos de Uso

```rust
use sc_dsl::dsl::parser::parser::{Rule, SCP};
use pest::Parser;

// Parse uma expressão simples
let input = "bug Cat";
let result = SCP::parse(Rule::bug, input);
assert!(result.is_ok());
```

## 📚 Documentação

- **AST**: Diagrama da AST em [`doc/ast.puml`](doc/ast.puml)
- **Grammar**: Especificação completa em [`src/dsl/sc.dsl`](src/dsl/sc.dsl)
