#[cfg(test)]
mod unit_tests {
    use super::*;
    use crate::produto::Produto;
    use crate::trie::Trie;
    use crate::catalogo::Catalogo;

    #[test]
    fn test_busca_exata() {
        let mut catalogo = Catalogo::new();
        let produto = Produto { id: 1, nome: "camiseta".to_string() };
        catalogo.adicionar(produto.clone());

        let resultado = catalogo.buscar_por_nome("camiseta");

        assert_eq!(resultado.unwrap().nome, "camiseta");
    }

    #[test]
    fn test_sugestoes_por_prefixo() {
        let mut catalogo = Catalogo::new();

        // Adicionando alguns produtos
        catalogo.adicionar(Produto { id: 1, nome: "camiseta".into() });
        catalogo.adicionar(Produto { id: 2, nome: "camisa".into() });
        catalogo.adicionar(Produto { id: 3, nome: "calça".into() });

        // Testando sugestões por prefixo
        let sugestoes = catalogo.sugestoes_por_prefixo("ca");
        assert_eq!(sugestoes.len(), 2);
        assert_eq!(sugestoes[0].nome, "camiseta");
        assert_eq!(sugestoes[1].nome, "camisa");
    }

    #[test]
    fn test_busca_por_prefixo_vazio() {
        let mut catalogo = Catalogo::new();
        catalogo.adicionar(Produto { id: 1, nome: "caneca".into() });

        // Testando um prefixo sem resultados
        let sugestoes = catalogo.sugestoes_por_prefixo("abc");
        assert_eq!(sugestoes.len(), 0);
    }

    #[test]
    fn test_busca_exata_nao_encontrado() {
        let mut catalogo = Catalogo::new();
        catalogo.adicionar(Produto { id: 1, nome: "camiseta".into() });

        // Testando uma busca não encontrada
        let resultado = catalogo.buscar_por_nome("calça");
        assert!(resultado.is_none());
    }
}
