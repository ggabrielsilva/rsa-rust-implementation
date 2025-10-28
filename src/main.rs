use num_bigint::{BigUint, RandBigInt};
use num_traits::{One, Zero};
use rand::thread_rng;
use std::ops::Rem;

// Estrutura para as chaves RSA
#[derive(Debug)]
struct RsaKey {
    n: BigUint,
    e: BigUint, // Chave pública (expoente)
    d: BigUint, // Chave privada (expoente)
}

// Implementação simplificada do teste de Miller-Rabin para BigUint
// Nota: Esta função é uma simplificação para fins de demonstração,
// uma implementação robusta requer mais testes e otimizações.
fn is_prime(n: &BigUint, k: usize) -> bool {
    if n <= &BigUint::from(1u32) {
        return false;
    }
    if n <= &BigUint::from(3u32) {
        return true;
    }
    let two = BigUint::from(2u32);
    if n.rem(&two).is_zero() { // Corrigido o uso de rem
        return false;
    }

    let mut d = n - BigUint::one();
    while d.clone().rem(&two).is_zero() { // Corrigido o uso de rem
        d /= &two;
    }

    let mut rng = thread_rng();
    let n_minus_one = n - BigUint::one();

    for _ in 0..k {
        // Usando gen_biguint_range
        let a = rng.gen_biguint_range(&two, &n_minus_one);
        let mut x = a.modpow(&d, n);
        if x == BigUint::one() || x == n_minus_one {
            continue;
        }

        let mut d_clone = d.clone();
        while d_clone != n_minus_one {
            x = x.modpow(&two, n);
            if x == n_minus_one {
                break;
            }
            d_clone *= &two;
        }
        if d_clone == n_minus_one {
            return false;
        }
    }
    true
}

// Função para gerar um número primo grande
fn generate_prime(bit_size: usize) -> BigUint {
    let mut rng = thread_rng();
    loop {
        // Gera um número ímpar
        let mut p = rng.gen_biguint(bit_size as u64) | BigUint::one();
        // Garante que o bit mais significativo esteja setado para o tamanho correto
        p |= BigUint::one() << (bit_size - 1);
        if is_prime(&p, 40) {
            return p;
        }
    }
}

// Função para calcular o inverso modular (d)
fn mod_inverse(a: &BigUint, m: &BigUint) -> Option<BigUint> {
    let m_int = num_bigint::BigInt::from(m.clone());
    let a_int = num_bigint::BigInt::from(a.clone());
    let mut x0 = num_bigint::BigInt::zero();
    let mut x1 = num_bigint::BigInt::one();
    let mut m_clone = m_int.clone();
    let mut a_clone = a_int.clone();

    while a_clone > num_bigint::BigInt::one() {
        let q = &a_clone / &m_clone;
        let mut t = m_clone.clone();
        m_clone = a_clone.clone() % m_clone;
        a_clone = t.clone();
        t = x0.clone();
        x0 = x1.clone() - &q * &x0;
        x1 = t.clone();
    }

    if x1 < num_bigint::BigInt::zero() {
        x1 += &m_int;
    }

    x1.to_biguint()
}

// Função de geração de chaves RSA
fn generate_keys(bit_size: usize) -> RsaKey {
    // 1. Escolher dois números primos grandes p e q
    let p = generate_prime(bit_size / 2);
    let q = generate_prime(bit_size / 2);

    // 2. Calcular n = p * q
    let n = &p * &q;

    // 3. Calcular a função totiente de Euler: phi(n) = (p-1) * (q-1)
    let p_minus_one = &p - BigUint::one();
    let q_minus_one = &q - BigUint::one();
    let phi_n = &p_minus_one * &q_minus_one;

    // 4. Escolher o expoente público e (geralmente 65537)
    let e = BigUint::from(65537u32);

    // 5. Calcular o expoente privado d (d = e^-1 mod phi_n)
    let d = mod_inverse(&e, &phi_n).expect("Não foi possível calcular o inverso modular d.");

    RsaKey { n, e, d }
}

// Função de criptografia (C = M^e mod n)
fn encrypt(message: &BigUint, key: &RsaKey) -> BigUint {
    message.modpow(&key.e, &key.n)
}

// Função de descriptografia (M = C^d mod n)
fn decrypt(ciphertext: &BigUint, key: &RsaKey) -> BigUint {
    ciphertext.modpow(&key.d, &key.n)
}

// Função principal de demonstração
fn main() {
    println!("--- Implementação do Algoritmo RSA em Rust ---");

    // 1. Geração de Chaves (tamanho de 1024 bits para demonstração)
    let bit_size = 1024;
    let keys = generate_keys(bit_size);
    println!("\n1. Geração de Chaves ({} bits):", bit_size);
    println!("   Chave Pública (n): {}", keys.n);
    println!("   Chave Pública (e): {}", keys.e);

    // 2. Mensagem a ser criptografada
    let original_message_str = "Montanha é 10";
    let original_message_bytes = original_message_str.as_bytes();
    // Converter a string para um BigUint (representação numérica)
    let original_message = BigUint::from_bytes_be(original_message_bytes);
    println!("\n2. Mensagem Original (String): {}", original_message_str);

    // A mensagem deve ser menor que n
    if original_message >= keys.n {
        println!("\n[ERRO] A mensagem é muito grande para este tamanho de chave (M >= n).");
        return;
    }

    // 3. Criptografia
    let ciphertext = encrypt(&original_message, &keys);
    println!("\n3. Criptografia:");
    println!("   Texto Cifrado (C): {}", ciphertext);

    // 4. Descriptografia
    let decrypted_message = decrypt(&ciphertext, &keys);

    // 5. Conversão de volta para string
    let decrypted_message_bytes = decrypted_message.to_bytes_be();
    let decrypted_message_str = String::from_utf8_lossy(&decrypted_message_bytes);

    println!("\n4. Descriptografia:");
    println!("   Mensagem Descriptografada (String): {}", decrypted_message_str);

    // 6. Verificação
    if original_message_str == decrypted_message_str {
        println!("\n5. Verificação: SUCESSO! A mensagem descriptografada corresponde à mensagem original.");
    } else {
        println!("\n5. Verificação: FALHA! A mensagem descriptografada NÃO corresponde à mensagem original.");
    }
}
