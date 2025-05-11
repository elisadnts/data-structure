use std::collections::HashMap;
use crate::produto::Produto;
use crate::trie::Trie;

pub struct Catalogo {
    produtos: HashMap<String, Produto>,
    trie: Trie,
}

impl Catalogo {
    pub fn new() -> Self {
        Catalogo {
            produtos: HashMap::new(),
            trie: Trie::new(),
        }
    }

    pub fn adicionar(&mut self, produto: Produto) {
        self.trie.insert(&produto.nome);
        self.produtos.insert(produto.nome.clone(), produto);
    }

    pub fn buscar_por_nome(&self, nome: &str) -> Option<&Produto> {
        self.produtos.get(nome)
    }

    pub fn sugestoes_por_prefixo(&self, prefixo: &str) -> Vec<&Produto> {
        self.trie
            .starts_with(prefixo)
            .iter()
            .filter_map(|nome| self.produtos.get(nome))
            .collect()
    }
}
