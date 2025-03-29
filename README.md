# Greedy SC

Aplicação escrita em Rust que processa um arquivo JSON contendo todas as cidades do estado de Santa Catarina. Utilizando um algoritmo guloso, a aplicação encontra o caminho entre duas cidades e retorna a distância entre elas.

## Como rodar?

### Requisitos

- Rust instalado (Recomenda-se utilizar a versão mais recente. Caso não tenha, instale via [Rustup](https://rustup.rs/)).

### Instalação das Dependências

Execute o comando abaixo para garantir que todas as dependências estejam instaladas corretamente:

```sh
cargo build
```

### Execução

Para rodar a aplicação, utilize:

```sh
cargo run
```
A aplicação ira iniciar no terminal e perguntará qual a cidade de origem e destino. Após isso a aplicação lerá os dados do arquivo JSON e calculará a menor distância entre as cidades informadas.

## Como funciona?

### Leitura do JSON

A aplicação carrega um arquivo JSON contendo todas as cidades de Santa Catarina e as conexões entre elas. Esse arquivo deve estar localizado na raiz do projeto e nomeado como `input.json`.

### Algoritmo Guloso

A busca pelo caminho entre duas cidades utiliza um algoritmo guloso. Ele seleciona, a cada passo, a cidade vizinha mais próxima que ainda não foi visitada, acumulando a distância percorrida até encontrar o destino.

## Tecnologias Utilizadas

- **Rust** -> Linguagem moderna e segura para desenvolvimento de alto desempenho
- **Serde** -> Biblioteca para serialização e desserialização de JSON

## Desafios

A maior dificuldade foi validar os cenários aonde poderiam acontecer erros por exemplo quando o caminho não existe.
