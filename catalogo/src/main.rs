mod produto;
mod trie;
mod catalogo;

use produto::Produto;
use catalogo::Catalogo;

fn main() {
    let mut catalogo = Catalogo::new();

    // Produtos variados
    let produtos = vec![
        Produto { id: 1, nome: "camiseta".into() },
        Produto { id: 2, nome: "camisa".into() },
        Produto { id: 3, nome: "calça".into() },
        Produto { id: 4, nome: "caneca".into() },
        Produto { id: 5, nome: "cadeira".into() },
        Produto { id: 6, nome: "carregador".into() },
        Produto { id: 7, nome: "cafeteira".into() },
        Produto { id: 8, nome: "celular".into() },
        Produto { id: 9, nome: "chinelo".into() },
        Produto { id: 10, nome: "controle remoto".into() },
        Produto { id: 11, nome: "abajur".into() },
        Produto { id: 12, nome: "armário".into() },
        Produto { id: 13, nome: "aspirador".into() },
        Produto { id: 14, nome: "ar-condicionado".into() },
        Produto { id: 15, nome: "blusa".into() },
    ];

    for produto in produtos {
        catalogo.adicionar(produto);
    }

    //  Busca exata
    let nome = "celular";
    match catalogo.buscar_por_nome(nome) {
        Some(produto) => println!(" Produto encontrado: {:?}", produto),
        None => println!(" Produto '{}' não encontrado.", nome),
    }

    //  Testes com múltiplos prefixos
    let prefixos = ["ca", "ce", "ch", "ar", "bl"];

    for prefixo in prefixos {
        println!("\n -- Sugestões para '{}':", prefixo);
        let sugestoes = catalogo.sugestoes_por_prefixo(prefixo);
        if sugestoes.is_empty() {
            println!("- Nenhum produto encontrado.");
        } else {
            for produto in sugestoes {
                println!("- {} (id: {})", produto.nome, produto.id);
            }
        }
    }
}
