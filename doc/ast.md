# Documentação da AST (Abstract Syntax Tree)

## Visão Geral
A AST (Abstract Syntax Tree) do SC-LAND representa a estrutura hierárquica do código fonte da DSL usando uma nomenclatura biológica inspirada. Cada nó da árvore representa uma construção do código fonte organizada em uma hierarquia modular.

## Arquitetura Hierárquica

A AST segue uma estrutura hierárquica bem definida:
```
SC → Fly → Strand → Genome → {Anatomy | Behavior}
```

### SC (Super Cell)
- **Descrição**: Nó raiz da AST, representa o contêiner principal do programa
- **Localização**: `src/dsl/ast/sc/mod.rs`
- **Atributos**:
  - `fly`: Referência para o nó Fly

### Fly
- **Descrição**: Representa uma unidade de execução completa, contendo todo o código do programa
- **Localização**: `src/dsl/ast/sc/fly/mod.rs`
- **Atributos**:
  - `strand`: Referência para o nó Strand

### Strand
- **Descrição**: Sequência de genomas que formam o programa, contendo definições estruturais e comportamentais
- **Localização**: `src/dsl/ast/sc/fly/strand/mod.rs`
- **Atributos**:
  - `genome`: Vector de genomas (Anatomy e Behavior)

## Genomas

### Genome (Enum)
- **Descrição**: União tipo que pode ser Anatomy ou Behavior
- **Localização**: `src/dsl/ast/sc/fly/strand/genome/mod.rs`
- **Variantes**:
  - `Anatomy`: Define estruturas de dados (classes/objetos)
  - `Behavior`: Define operações e comportamentos

### Anatomy
- **Descrição**: Define a estrutura anatômica de objetos (equivalente a classes)
- **Localização**: `src/dsl/ast/sc/fly/strand/genome/anatomy/mod.rs`
- **Tipos**:
  - `Bug`: Representa uma classe/objeto com propriedades e métodos

#### Bug
- **Descrição**: Estrutura principal que define uma classe
- **Localização**: `src/dsl/ast/sc/fly/strand/genome/anatomy/bug/mod.rs`
- **Componentes**:
  - `specie`: Nome/tipo da classe
  - `gene`: Propriedades da classe (equivalente a campos/atributos)
  - `ethics`: Métodos da classe (equivalente a funções/métodos)

#### Gene
- **Descrição**: Representa uma propriedade/atributo de uma classe
- **Localização**: `src/dsl/ast/sc/fly/strand/genome/anatomy/bug/gene/mod.rs`
- **Componentes**:
  - `tag`: Nome da propriedade
  - `specie`: Tipo da propriedade

#### Ethics
- **Descrição**: Representa um método/função de uma classe
- **Localização**: `src/dsl/ast/sc/fly/strand/genome/anatomy/bug/ethics/mod.rs`
- **Componentes**:
  - `tag`: Nome do método
  - `signature`: Parâmetros do método (opcional)
  - `feedback`: Tipo de retorno (opcional)
  - `matrix`: Corpo do método (bloco de código)
### Behavior
- **Descrição**: Define o comportamento e operações do programa
- **Localização**: `src/dsl/ast/sc/fly/strand/genome/behavior/mod.rs`
- **Tipos**:
  - `Assign`: Atribuição de valores a variáveis
  - `Beat`: Operações orientadas a objetos complexas
  - `Bind`: Ligação simples de valores
  - `Binds`: Múltiplas ligações
  - `Sequence`: Sequência de operações
  - `Transport`: Transporte de dados

#### Assign
- **Descrição**: Representa uma atribuição de valor
- **Localização**: `src/dsl/ast/sc/fly/strand/genome/behavior/assign.rs`
- **Atributos**:
  - `tag`: Variável de destino
  - `beat`: Operação a ser atribuída

#### Beat
- **Descrição**: Operação orientada a objetos (equivalente a chamadas de método)
- **Localização**: `src/dsl/ast/sc/fly/strand/genome/behavior/beat/mod.rs`
- **Componentes**:
  - `forager`: Emissor da operação (objeto ou literal)
  - `course`: Trilhas de operação (métodos e parâmetros)

##### Forager (Emitter)
- **Descrição**: Emissor de uma operação
- **Localização**: `src/dsl/ast/sc/fly/strand/genome/behavior/trace/forager/mod.rs`
- **Tipos**:
  - `Literal`: Valores literais (números, strings, etc.)
  - `SelfRef`: Referência a variáveis ou objetos

##### Course (Trails)
- **Descrição**: Trilhas que definem como a operação é executada
- **Localização**: `src/dsl/ast/sc/fly/strand/genome/behavior/trace/course/mod.rs`
- **Tipos**:
  - `Carrier`: Transporta dados/parâmetros
  - `Catalysis`: Catalisa operações (chamadas de método)

## Estruturas de Código

### Matrix
- **Descrição**: Bloco de código que contém múltiplas operações
- **Localização**: `src/dsl/ast/sc/fly/strand/genome/anatomy/bug/ethics/matrix/mod.rs`
- **Atributos**:
  - `signal`: Array de sinais (comportamentos individuais)

### Signal
- **Descrição**: Comportamento individual dentro de um bloco
- **Localização**: `src/dsl/ast/sc/fly/strand/genome/anatomy/bug/ethics/matrix/signal/mod.rs`
- **Atributos**:
  - `behavior`: Comportamento associado

## Tipos de Dados e Literais

### Literal
- **Descrição**: Valores constantes do programa
- **Localização**: `src/dsl/ast/sc/fly/strand/genome/behavior/trace/forager/literal.rs`
- **Tipos**:
  - `Bit`: Valor booleano (true/false)
  - `Hex`: Valor hexadecimal (0xFF)
  - `Int`: Número inteiro (42)
  - `Str`: String ("hello")
  - `Decimal`: Número decimal (3.14)

### Specie
- **Descrição**: Tipos de dados do sistema
- **Localização**: `src/dsl/ast/sc/fly/strand/genome/anatomy/bug/gene/specie.rs`
- **Exemplos**: Int, String, Bool, etc.

### Tag
- **Descrição**: Identificadores/nomes de variáveis, métodos, etc.
- **Localização**: `src/dsl/ast/sc/fly/strand/genome/anatomy/bug/gene/tag.rs`

## Organização Modular

A nova arquitetura segue um padrão hierárquico que reflete a estrutura conceitual:

```
src/dsl/ast/sc/
├── mod.rs                          # SC (Super Cell)
└── fly/
    ├── mod.rs                      # Fly
    └── strand/
        ├── mod.rs                  # Strand
        └── genome/
            ├── mod.rs              # Genome (enum)
            ├── anatomy/            # Definições estruturais
            │   ├── mod.rs
            │   └── bug/
            │       ├── mod.rs      # Bug (classe)
            │       ├── gene/       # Propriedades
            │       └── ethics/     # Métodos
            └── behavior/           # Operações
                ├── mod.rs
                ├── assign.rs       # Atribuições
                ├── beat/           # Operações OOP
                └── trace/          # Trilhas de execução
```

## Vantagens da Nova Arquitetura

1. **Modularidade**: Cada conceito tem seu próprio módulo
2. **Hierarquia Clara**: A estrutura de pastas reflete a hierarquia conceitual
3. **Manutenibilidade**: Fácil localização e modificação de componentes
4. **Extensibilidade**: Novos tipos podem ser adicionados sem afetar outros
5. **Nomenclatura Intuitiva**: Termos biológicos facilitam o entendimento

## Observações de Implementação

- Todos os componentes implementam `Debug`, `Clone`, `PartialEq`, `Eq`, `Serialize`, `Deserialize`
- A estrutura suporta parsing seguro através do Pest parser
- Cada nó da AST pode ser construído a partir de `Pair<Rule>` do Pest
- O sistema permite composição complexa através de trilhas e transportes
- Suporte completo a programação orientada a objetos
- Estruturas de controle de fluxo integradas

## Diagramas

Para uma visualização completa da estrutura da AST, consulte:
- **PlantUML**: [`doc/ast.puml`](ast.puml) - Diagrama detalhado da hierarquia
- **Gramática**: [`src/dsl/sc.dsl`](../src/sc.dsl) - Especificação formal da sintaxe
