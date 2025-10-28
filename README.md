# Implementação do Algoritmo RSA em Rust

Este projeto contém uma implementação funcional do algoritmo RSA (Rivest–Shamir–Adleman) em Rust, utilizando a biblioteca `num-bigint` para manipulação de números inteiros de precisão arbitrária, essencial para a criptografia de chave pública.

O código demonstra as três etapas principais do RSA:
1. **Geração de Chaves:** Criação do par de chaves pública e privada.
2. **Criptografia:** Cifragem de uma mensagem usando a chave pública.
3. **Descriptografia:** Decifragem da mensagem usando a chave privada.

## Estrutura do Projeto

O projeto segue a estrutura padrão de um projeto Rust gerenciado pelo `Cargo`.

- `Cargo.toml`: Define as dependências do projeto.
- `src/main.rs`: Contém a lógica principal do algoritmo RSA.

## Requisitos

Para compilar e executar este projeto, você precisa ter o **Rust** e o **Cargo** instalados em seu sistema.

## Como Compilar e Executar

1. **Clone o repositório** (você já fez isso!):

   ```bash
   git clone [URL_DO_SEU_REPOSITORIO]
   cd rsa-rust-implementation
