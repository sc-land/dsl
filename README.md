# SC-LAND DSL

Uma Domain-Specific Language (DSL) implementada em Rust com nomenclatura biológica inspiradora, usando Pest para parsing e uma arquitetura AST hierárquica modular.

## 📖 Visão Geral

O SC-LAND DSL implementa um sistema de programação usando terminologia biológica, onde:
- **SC (Super Cell)**: O programa como um organismo vivo
- **Fly**: Unidades de execução que "voam" através do código
- **Strand**: Sequências genéticas que contêm informação
- **Genome**: Unidades básicas de informação (Anatomy + Behavior)
- **Bug**: Estruturas de dados (equivalente a classes)
- **Gene**: Propriedades/atributos
- **Ethics**: Métodos/funções
- **Beat**: Operações orientadas a objetos

### Funcionalidades Principais
- **Anatomy**: Definição de estruturas usando `bug` (classes/objetos)
- **Behavior**: Operações, atribuições e chamadas de método
- **Literais**: Suporte completo a tipos de dados (int, string, bool, hex, decimal)
- **Operações OOP**: Sistema robusto de orientação a objetos
- **Arquitetura Modular**: Estrutura hierárquica bem organizada

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
```sc
bug Cat
  gene energia Int
  gene folego Int
  ethics correr()
    energia = energia.minus(1)
  end
  ethics miar() String
    "miau"
  end
end

bug Dog
  gene nome String
  gene idade Int
  ethics latir()
    "au au"
  end
  ethics envelhecer()
    idade = idade.plus(1)
  end
end
```

### Operações (Behavior)
```sc
# Atribuições simples
meuGato = Cat.new()
energia = meuGato.energia

# Chamadas de método
som = meuGato.miar()
meuGato.correr()

# Operações com parâmetros
resultado = Calculator.add(a: 10, b: 20)
```

### Literais suportados
```sc
42          # inteiro
0xFF        # hexadecimal
0b1010      # binário
3.14        # decimal
"hello"     # string
true        # booleano
false       # booleano
```

### Operações Orientadas a Objetos
```sc
# Criação de objetos
animal = Dog.new()

# Acesso a propriedades
nome = animal.nome

# Chamadas de método
animal.latir()

# Métodos com parâmetros
resultado = Math.calculate(x: 10, y: 20)

# Encadeamento de operações
valor = objeto.metodo1().metodo2(param: "test")
```

## 🏗️ Estrutura do Projeto

- **Grammar**: Gramática Pest em [`src/dsl/sc.dsl`](src/dsl/sc.dsl)
- **AST**: Árvore sintática hierárquica em [`src/dsl/ast/sc/`](src/dsl/ast/sc/)
- **Parser**: Implementação do parser em [`src/dsl/parser/`](src/dsl/parser/)
- **Tests**: Testes unitários e de integração em [`tests/`](tests/)
- **Documentation**: Documentação detalhada em [`doc/`](doc/)

### Componentes Principais
- **SC Module**: Nó raiz (`src/dsl/ast/sc/mod.rs`)
- **Fly Module**: Unidade de execução (`src/dsl/ast/sc/fly/`)
- **Strand Module**: Sequências genéticas (`src/dsl/ast/sc/fly/strand/`)
- **Genome Module**: Unidades básicas (`src/dsl/ast/sc/fly/strand/genome/`)
- **Anatomy**: Estruturas de dados (`src/dsl/ast/sc/fly/strand/genome/anatomy/`)
- **Behavior**: Operações e comportamentos (`src/dsl/ast/sc/fly/strand/genome/behavior/`)

## 📊 Arquitetura

O SC-LAND DSL segue uma arquitetura hierárquica modular inspirada em biologia:

```
SC (Super Cell)
└── Fly (Unidade de Execução)
    └── Strand (Sequência Genética)
        └── Genome (Unidade Básica)
            ├── Anatomy (Estruturas)
            │   └── Bug (Classes)
            │       ├── Gene (Propriedades)
            │       └── Ethics (Métodos)
            └── Behavior (Operações)
                ├── Assign (Atribuições)
                ├── Beat (Operações OOP)
                └── Trace (Trilhas de Execução)
```

### Estrutura de Arquivos
```
src/dsl/ast/sc/
├── mod.rs                          # SC (Super Cell)
└── fly/
    ├── mod.rs                      # Fly
    └── strand/
        ├── mod.rs                  # Strand
        └── genome/
            ├── mod.rs              # Genome
            ├── anatomy/            # Definições estruturais
            │   └── bug/           # Classes e objetos
            └── behavior/          # Operações e comportamentos
```

## 🧪 Exemplos de Uso

```rust
use sc_dsl::dsl::parser::parser::{Rule, SCP};
use pest::Parser;

// Parse uma definição de bug (classe)
let input = "bug Cat
  gene nome String
  ethics miar()
    \"miau\"
  end
end";
let result = SCP::parse(Rule::bug, input);
assert!(result.is_ok());

// Parse uma operação
let operation = "gato.miar()";
let result = SCP::parse(Rule::beat, operation);
assert!(result.is_ok());

// Parse uma atribuição
let assignment = "nome = gato.nome";
let result = SCP::parse(Rule::assign, assignment);
assert!(result.is_ok());
```

## 🌟 Características Especiais

### Nomenclatura Biológica
- **Intuitiva**: Termos biológicos facilitam o entendimento
- **Consistente**: Toda a arquitetura segue a metáfora biológica
- **Memorável**: Conceitos mais fáceis de lembrar e associar

### Arquitetura Modular
- **Hierárquica**: Estrutura clara de dependências
- **Extensível**: Fácil adição de novos componentes
- **Manutenível**: Cada módulo tem responsabilidade específica

### Parsing Robusto
- **Pest Grammar**: Parser formal e confiável
- **Error Handling**: Tratamento elegante de erros
- **AST Generation**: Geração automática da árvore sintática

## 📚 Documentação

- **AST**: Documentação completa da AST em [`doc/ast.md`](doc/ast.md)
- **Diagrama PlantUML**: Visualização da estrutura em [`doc/ast.puml`](doc/ast.puml)
- **Grammar**: Especificação formal em [`src/dsl/sc.dsl`](src/dsl/sc.dsl)
- **Examples**: Exemplos práticos em [`examples/`](examples/)

## 🚀 Próximos Passos

- [ ] Implementação de estruturas de controle (if, while, for)
- [ ] Sistema de tipos mais robusto
- [ ] Suporte a herança e polimorfismo
- [ ] Compilação para bytecode
- [ ] IDE support com syntax highlighting
- [ ] Documentação interativa

## 🤝 Contribuindo

Contribuições são bem-vindas! Por favor:
1. Fork o projeto
2. Crie uma branch para sua feature (`git checkout -b feature/AmazingFeature`)
3. Commit suas mudanças seguindo a convenção de commits biológicos
4. Push para a branch (`git push origin feature/AmazingFeature`)
5. Abra um Pull Request

## 📄 Licença

Este projeto está licenciado sob a licença especificada no arquivo [LICENSE](LICENSE).
