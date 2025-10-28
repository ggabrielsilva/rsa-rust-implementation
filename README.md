Criptografia RSA

A criptografia  RSA (Rivest-Shamir-Adleman), desenvolvido em 1977, é um dos algoritmos de chave pública mais antigos e amplamente utilizados no mundo, sendo a base de muitos protocolos modernos de segurança.

Nosso objetivo é entender não apenas o que o RSA faz, mas *como* a matemática permite que ele funcione de maneira tão segura, desde a perspectiva de um leigo até a de um especialista.



1. O Princípio Básico: A Chave Assimétrica

O RSA é um algoritmo de **criptografia assimétrica**. Isso significa que ele usa *duas* chaves diferentes que estão matematicamente ligadas:

1.  **Chave Pública (🔒 Abrir):** É compartilhada abertamente. Qualquer pessoa pode usá-la para criptografar uma mensagem.
2.  **Chave Privada (🔑 Segredo):** É mantida em segredo absoluto pelo proprietário. Apenas ela pode descriptografar as mensagens que foram criptografadas pela chave pública correspondente.

**Analogia:** Imagine um cadeado especial. Você compartilha a caixa (Chave Pública) para que as pessoas coloquem suas mensagens trancadas lá dentro. Somente você, que possui a chave mestra para abrir o cadeado (Chave Privada), pode ler o conteúdo.

O RSA é **bidirecional**. Além de criptografar dados para garantir a confidencialidade, ele também é amplamente usado para **assinatura digital** (com PSS), garantindo a autenticidade e a não-repúdio.

2. Fundamentos Matemáticos: O "Portão Secreto"

A segurança do RSA não se baseia em segredos guardados, mas sim em um **problema matemático difícil de resolver**.

O princípio central é a **dificuldade de fatorar números grandes**.

2.1. Números Primos e Fatoração (O Segredo)

O RSA começa escolhendo dois números primos distintos muito grandes, chamados $p$ e $q$.

1.  **Cálculo do Módulo ($n$):** A chave pública é baseada no produto desses primos: $n = p \times q$. Este módulo ($n$) é público e define o "tamanho" da chave.
2.  **A Dificuldade da Fatoração:** O processo de criptografia e descriptografia depende de conhecer $p$ e $q$ separadamente. No entanto, dado apenas o resultado $n$ (um número enorme, como 2048 bits), é **computacionalmente inviável** encontrar os fatores $p$ e $q$ em tempo hábil com os algoritmos clássicos atuais.

Se alguém conseguisse fatorar $n$ rapidamente, a criptografia RSA estaria quebrada.

2.2. Aritmética Modular e o Teorema de Euler (A Chave Mágica)

Para que o sistema funcione, são necessárias ferramentas de matemática avançada:

*   **Aritmética Modular:** É a matemática do "resto" (como a leitura de um relógio, onde após 12, voltamos a 1). As operações no RSA são realizadas "módulo n".
*   **Função Totiente de Euler ($\varphi(n)$):** Esta função é crucial para a geração da chave privada. Ela é calculada usando os primos secretos: $\varphi(n) = (p-1) \times (q-1)$. **Este valor deve ser mantido em segredo!**.
*   **Teorema de Euler:** Garante que a descriptografia funcione. Ele permite que a mensagem original seja recuperada elevando o texto criptografado a um expoente específico.

A relação entre a chave pública ($e$) e a chave privada ($d$) é definida pelo **Inverso Modular**: $e \times d \equiv 1 \pmod{\varphi(n)}$.

3. Funcionamento Passo-a-Passo do Algoritmo

O processo é dividido em duas fases principais:

Fase 1: Geração de Chaves

1.  **Escolher Primos ($p, q$):** Gerar dois primos grandes e distintos.
2.  **Calcular o Módulo ($n$):** $n = p \times q$.
3.  **Calcular $\varphi(n)$ (Secreto):** $\varphi(n) = (p-1) \times (q-1)$.
4.  **Escolher o Expoente Público ($e$):** Um valor comum é $e = 65537$.
5.  **Calcular o Expoente Privado ($d$):** Encontrar $d$ (o inverso modular de $e \pmod{\varphi(n)}$) usando o Algoritmo Euclidiano Estendido.

**Resultado:**
*   **Chave Pública:** $(n, e)$.
*   **Chave Privada:** $(n, d)$.

Fase 2: Criptografia e Descriptografia

Uma mensagem ($m$) é primeiro convertida em um número ($m < n$).

| Operação | Fórmula | Chave Utilizada |
| :--- | :--- | :--- |
| **Criptografia** | $c = m^e \pmod n$ | Chave Pública $(n, e)$ |
| **Descriptografia** | $m = c^d \pmod n$ | Chave Privada $(n, d)$ |

A **Exponenciação Modular Rápida** (algoritmo "Square-and-Multiply") é fundamental para realizar esses cálculos de forma eficiente, mesmo com números gigantescos.

4. Aplicações Práticas em Segurança da Informação

O RSA é vital para a infraestrutura de segurança digital.

1.  **Troca de Chaves Simétricas (Uso Híbrido):** O RSA é **lento** (cerca de 1000 vezes mais lento que o AES). Por isso, ele raramente é usado para criptografar grandes volumes de dados. Em vez disso, ele é usado para **criptografar uma chave simétrica temporária** (como uma chave AES). Essa chave simétrica, pequena e segura, é então usada para criptografar os dados em alta velocidade.
2.  **Protocolos de Segurança:** O RSA é a base de muitos protocolos amplamente adotados, como **HTTPS** (para navegação segura na web) e **SSH** (para conexões remotas seguras).
3.  **Assinatura Digital:** Usado para provar a origem e a integridade de um documento, garantindo que ele não foi alterado.
4.  **Tamanhos de Chave:** Atualmente, o tamanho de chave **mínimo recomendado é de 2048 bits**, mas **3072 bits** são recomendados para 2025 para maior segurança.

5. Limitações e Desafios

Embora o RSA tenha uma base matemática sólida e uma maturidade de mais de 45 anos, ele enfrenta desafios importantes:

*   **Vulnerabilidade Quântica:** O **Algoritmo de Shor** (1994) pode quebrar o RSA em tempo polinomial. Se computadores quânticos suficientemente poderosos (cerca de 4096 qubits lógicos) forem construídos, a segurança do RSA será comprometida. A estimativa atual é de 10 a 20 anos para essa implementação prática.
*   **Performance:** É mais lento que alternativas modernas, como a Criptografia de Curva Elíptica (ECC). Sistemas mais novos geralmente preferem o ECC.
*   **Complexidade de Implementação:** O RSA é fácil de ser implementado **incorretamente**, o que pode abrir vulnerabilidades (como ataques de *side-channel* ou o uso de *padding* inseguro). Por isso, o uso recomendado é sempre através de bibliotecas profissionais e testadas, como OpenSSL ou BoringSSL.
