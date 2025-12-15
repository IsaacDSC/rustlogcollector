use rustlogcollector::compress;
use rustlogcollector::store::Store;
use std::thread;
use std::time::Duration;

fn main() {
    println!("=== Exemplo 1: Uso básico ===");
    basic_example();

    println!("\n=== Exemplo 2: Uso concorrente com múltiplas threads ===");
    concurrent_example();
}

fn basic_example() {
    let store = Store::new(20);

    let text =
        "A raposa rápida pula sobre o cão preguiçoso. A raposa rápida pula sobre o cão preguiçoso.";
    println!("Original: {} bytes", text.len());

    // --- Compressão ---
    let compressed = compress(text.as_bytes());
    println!("Comprimido: {} bytes", compressed.len());

    // --- Adição de dados ---
    store.add(compressed);

    let text2 = "Text 2";
    let compressed2 = compress(text2.as_bytes());
    store.add(compressed2);

    if let Ok(first) = store.retrieve_first() {
        println!("Dados armazenados: {} itens no primeiro batch", first.len());
    } else {
        println!("No data found");
    }
}

fn concurrent_example() {
    let store = Store::new(5);
    let mut handles = vec![];

    // Criar 3 threads que adicionam dados simultaneamente
    for thread_id in 0..3 {
        let store_clone = store.clone_handle();

        let handle = thread::spawn(move || {
            for i in 0..10 {
                let message = format!("Thread {} - Mensagem {}", thread_id, i);
                let compressed = compress(message.as_bytes());
                store_clone.add(compressed);

                println!("Thread {} adicionou mensagem {}", thread_id, i);

                // Pequeno delay para simular processamento
                thread::sleep(Duration::from_millis(10));
            }
        });

        handles.push(handle);
    }

    // Aguardar todas as threads terminarem
    for handle in handles {
        handle.join().unwrap();
    }

    // Verificar resultado final
    if let Ok(first) = store.retrieve_first() {
        println!("\n✓ Todas as threads terminaram!");
        println!("✓ Total de itens no primeiro batch: {}", first.len());
        println!("✓ Store protegido contra race conditions com Mutex");
    }
}
