// Importa o que é público da biblioteca 'megastore'
use megastore::{Product, SearchEngine};

fn main() {
    let mut engine = SearchEngine::new();

    // 1. Populando o sistema
    println!("Indexando produtos da MegaStore...");
    engine.add_product(Product {
        id: 1,
        name: String::from("Smartphone Apple iPhone 15"),
        brand: String::from("Apple"),
        category: String::from("Eletrônicos"),
        price: 7000.0,
    });

    engine.add_product(Product {
        id: 2,
        name: String::from("Monitor Gamer 144hz"),
        brand: String::from("AOC"),
        category: String::from("Informática"),
        price: 1200.0,
    });

    // 2. Testando a busca
    println!("\n--- Resultado da busca por: 'iPhone' ---");
    let resultados = engine.search_by_keyword("iPhone");
    
    if resultados.is_empty() {
        println!("Nenhum produto encontrado.");
    } else {
        for p in resultados {
            println!("ID: {} | Nome: {} | Preço: R$ {:.2}", p.id, p.name, p.price);
        }
    }

    println!("\n--- Resultado da busca por categoria: 'Informática' ---");
    let info_products = engine.search_by_category("Informática");
    for p in info_products {
        println!("Encontrado: {}", p.name);
    }
}