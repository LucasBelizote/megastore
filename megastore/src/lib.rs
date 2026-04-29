use std::collections::{HashMap, HashSet};

/// Representa um produto no catálogo da MegaStore
#[derive(Debug, Clone)]
pub struct Product {
    pub id: u32,
    pub name: String,
    pub brand: String,
    pub category: String,
    pub price: f64,
}

/// Estrutura principal do Sistema de Busca
pub struct SearchEngine {
    // Armazena os dados reais dos produtos indexados pelo ID
    products: HashMap<u32, Product>,
    
    // Índices Invertidos para busca rápida O(1)
    name_index: HashMap<String, HashSet<u32>>,
    brand_index: HashMap<String, HashSet<u32>>,
    category_index: HashMap<String, HashSet<u32>>,
}

impl SearchEngine {
    /// Inicializa um novo motor de busca
    pub fn new() -> Self {
        SearchEngine {
            products: HashMap::new(),
            name_index: HashMap::new(),
            brand_index: HashMap::new(),
            category_index: HashMap::new(),
        }
    }

    /// Normaliza o texto (minúsculas e divide por espaços)
    fn tokenize(text: &str) -> Vec<String> {
        text.to_lowercase()
            .split_whitespace()
            .map(|s| s.to_string())
            .collect()
    }

    /// Adiciona e indexa um produto
    pub fn add_product(&mut self, product: Product) {
        let id = product.id;
        
        for token in Self::tokenize(&product.name) {
            self.name_index.entry(token).or_insert_with(HashSet::new).insert(id);
        }

        self.brand_index.entry(product.brand.to_lowercase()).or_insert_with(HashSet::new).insert(id);
        self.category_index.entry(product.category.to_lowercase()).or_insert_with(HashSet::new).insert(id);
        
        self.products.insert(id, product);
    }

    /// Busca por palavra-chave no nome
    pub fn search_by_keyword(&self, keyword: &str) -> Vec<&Product> {
        let key = keyword.to_lowercase();
        if let Some(ids) = self.name_index.get(&key) {
            ids.iter().filter_map(|id| self.products.get(id)).collect()
        } else {
            vec![]
        }
    }

    /// Busca por categoria
    pub fn search_by_category(&self, category: &str) -> Vec<&Product> {
        let key = category.to_lowercase();
        if let Some(ids) = self.category_index.get(&key) {
            ids.iter().filter_map(|id| self.products.get(id)).collect()
        } else {
            vec![]
        }
    }
}

// --- TESTES UNITÁRIOS ---
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn teste_busca_simples() {
        let mut engine = SearchEngine::new();
        engine.add_product(Product {
            id: 1,
            name: String::from("Teclado"),
            brand: String::from("Dell"),
            category: String::from("Periféricos"),
            price: 150.0,
        });
        let result = engine.search_by_keyword("Teclado");
        assert_eq!(result.len(), 1);
    }
}