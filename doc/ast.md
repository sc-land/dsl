# Documentação da AST (Abstract Syntax Tree)

## Visão Geral
A AST (Abstract Syntax Tree) representa a estrutura hierárquica do código fonte da DSL. Cada nó da árvore representa uma construção do código fonte.

## Estrutura Principal

### SC (Super Cell)
- **Descrição**: Nó raiz da AST
- **Atributos**:
  - `fly`: Referência para o nó Fly

### Fly
- **Descrição**: Representa uma unidade de execução
- **Atributos**:
  - `strand`: Referência para o nó Strand

### Strand
- **Descrição**: Contém os genomas (Anatomy e Behavior)
- **Atributos**:
  - `genome`: Array de genomas (Anatomy e Behavior)

## Genomas

### Anatomy
- **Descrição**: Define a estrutura anatômica
- **Tipos**:
  - `Bug`: Representa um bug com espécie, genes e éticas

### Behavior
- **Descrição**: Define o comportamento
- **Tipos**:
  - `Assign`: Atribuição de valores
  - `Oop`: Operação orientada a objetos
  - `Return`: Retorno de valor
  - `If`: Estrutura condicional
  - `While`: Loop condicional
  - `For`: Loop iterativo

## Componentes de Behavior

### Assign
- **Descrição**: Representa uma atribuição
- **Atributos**:
  - `tag`: Tag associada
  - `oop`: Operação a ser atribuída

### Return
- **Descrição**: Representa um retorno
- **Atributos**:
  - `oop`: Operação a ser retornada

### If
- **Descrição**: Estrutura condicional
- **Atributos**:
  - `condition`: Condição (Oop)
  - `matrix`: Bloco principal
  - `elsif`: Array de condições alternativas
  - `else`: Bloco opcional

### While
- **Descrição**: Loop condicional
- **Atributos**:
  - `condition`: Condição (Oop)
  - `matrix`: Bloco do loop

### For
- **Descrição**: Loop iterativo
- **Atributos**:
  - `tag`: Tag do iterador
  - `oop`: Coleção a ser iterada
  - `matrix`: Bloco do loop

### Oop
- **Descrição**: Operação orientada a objetos
- **Atributos**:
  - `emitter`: Emissor (Specie, Tag ou Literal)
  - `trails`: Array de trilhas (Carrier ou Catalysis)

## Estruturas de Código

### Matrix
- **Descrição**: Bloco de código
- **Atributos**:
  - `signals`: Array de sinais (comportamentos)

### Signal
- **Descrição**: Comportamento individual
- **Atributos**:
  - `behavior`: Comportamento associado

## Trilhas (Trails)

### Carrier
- **Descrição**: Transporta dados
- **Atributos**:
  - `transport`: Transporte opcional (Binds ou Sequence)

### Catalysis
- **Descrição**: Catalisa operações
- **Atributos**:
  - `tag`: Tag associada
  - `carrier`: Carrier opcional

## Transportes

### Binds
- **Descrição**: Ligações entre elementos
- **Atributos**:
  - `binds`: Array de ligações

### Sequence
- **Descrição**: Sequência de operações
- **Atributos**:
  - `oops`: Array de operações

## Anatomia

### Bug
- **Descrição**: Representa um bug
- **Atributos**:
  - `specie`: Espécie do bug
  - `genes`: Array de genes
  - `ethics`: Array de éticas

### Gene
- **Descrição**: Representa um gene
- **Atributos**:
  - `tag`: Tag associada
  - `specie`: Espécie associada

### Ethics
- **Descrição**: Representa uma ética (método/função)
- **Atributos**:
  - `tag`: Tag associada
  - `signature`: Assinatura opcional
  - `feedback`: Feedback opcional
  - `matrix`: Bloco de código

### Signature
- **Descrição**: Assinatura de ética
- **Atributos**:
  - `binds`: Array de ligações de parâmetros

### EthicsBind
- **Descrição**: Ligação de parâmetro de ética
- **Atributos**:
  - `tag`: Tag do parâmetro
  - `specie`: Espécie do parâmetro

## Tipos de Dados

### Emitter
- **Tipos**:
  - `Specie`: Espécie
  - `Tag`: Tag
  - `Literal`: Valor literal

### Literal
- **Tipos**:
  - `Bit`: Valor booleano
  - `Hex`: Valor hexadecimal
  - `Int`: Número inteiro
  - `Str`: String
  - `Decimal`: Número decimal

## Observações
- A estrutura permite composição complexa de operações através de trilhas e transportes
- O sistema de tipos é flexível, permitindo diferentes tipos de emissores e literais
- Suporte completo a programação orientada a objetos com éticas (métodos)
- Suporte a estruturas de controle de fluxo (if, while, for)
- Suporte a blocos de código através de matrix

# SC-LAND AST Documentation

## Estrutura da AST

A AST (Abstract Syntax Tree) do SC-LAND é composta pelos seguintes componentes principais:

### SC (Super Cell)
- Nó raiz da AST
- Contém um único nó Fly

### Fly
- Representa uma unidade de execução
- Contém um único nó Strand

### Strand
- Sequência de genomas
- Contém um array de Genome

### Genome
- Pode ser Anatomy ou Behavior
- Anatomy: Define a estrutura anatômica
- Behavior: Define o comportamento

### Anatomy
- Enum com variante Bug
- Bug: Contém specie, genes e ethics

### Behavior
- Enum com variantes:
  - Statement (If, While, For)
  - Assign (atribuição)
  - Oop (operação orientada a objetos)

### Ethics
- Representa um método/função
- Atributos:
  - tag: nome do método
  - signature: parâmetros (opcional)
  - feedback: tipo de retorno (opcional)
  - body: bloco de código (opcional)

### Gene
- Representa uma propriedade
- Atributos:
  - tag: nome da propriedade
  - specie: tipo da propriedade

## Gramática

### Tokens Básicos
- `bug`: Início de definição de bug
- `gene`: Definição de propriedade
- `ethics`: Definição de método
- `end`: Fim de bloco
- `if`: Início de condicional
- `while`: Início de loop
- `for`: Início de iteração
- `return`: Retorno de valor

### Tipos
- `Int`: Número inteiro
- `String`: Texto
- `Bool`: Booleano
- `Void`: Sem retorno

### Operadores
- `.`: Acesso a método/propriedade
- `=`: Atribuição
- `(`: Início de parâmetros
- `)`: Fim de parâmetros
- `:`: Separação de tipo

## Notas de Implementação

1. Todos os componentes são imutáveis por padrão
2. A AST suporta clonagem para manipulação segura
3. Os testes cobrem todos os casos de uso principais
4. A documentação é mantida atualizada com a implementação
5. Toda operação é orientada a objetos, usando métodos apropriados
6. Os testes de erro são feitos através de fragmentos específicos

## Diagramas

Ver `ast.puml` para diagramas detalhados da estrutura da AST.
