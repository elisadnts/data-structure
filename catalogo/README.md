🚀 Como executar o sistema de busca

1. Pré-requisitos
Certifique-se de ter o Rust instalado. Para instalar, siga as instruções no site oficial do Rust: Rust - Install.

Verifique a instalação:

    rustc --version
    cargo --version

2. Compilar o projeto
No diretório do projeto, execute o comando para compilar o código em modo de liberação (otimizado para desempenho):

    cargo build --release

3. Executar o sistema
Para rodar o sistema de busca, utilize o seguinte comando:

    cargo run


▶️ Como usar o sistema:

Aqui estão algumas das principais funções que você pode usar para interagir com o sistema de busca do catálogo:

*Listar todas as categorias e produtos disponíveis:

    catalogo.listar_todos();

*Buscar produtos por categoria:
Exemplo de busca por categoria "eletrônicos":

    catalogo.buscar_por_categoria("eletronicos");


*Listar buscas frequentes:
Para listar as buscas mais frequentes no sistema:

    catalogo.listar_buscas_frequentes();


*Exibir recomendações de produtos:
Exemplo de exibição de recomendações para "Smartphone":

    catalogo.exibir_recomendacoes("Smartphone");

