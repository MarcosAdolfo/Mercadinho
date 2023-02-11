use std::io;
//use std::fmt::Display;
use rand::Rng;
use std::collections::HashMap;

extern crate rand;

//#[derive(Debug)]
struct Produto
{
    nome: String,
    valor: f64,
    lucro: f64,
    estoque: u16,
    id: u16,
}

impl Produto
{
    fn calc_preco(&self) -> f64
    {
        ((self.lucro / 100.0) * self.valor) + self.valor
    }

    fn new_nome(&mut self, nome:String)
    {
        self.nome = nome.to_uppercase();
    }

    fn new_valor(&mut self, valor:f64)
    {
        self.valor = valor;
    }

    fn new_lucro(&mut self, valor:f64)
    {
        self.lucro = valor;
    }

    fn new_estoque(&mut self, valor:u16)
    {
        self.estoque = valor;
    }
}

fn new_produto() -> Produto
{
    println!("Nome do Produto");
    let mut nome:String = String::new();
    io::stdin().read_line(&mut nome).expect("Failed to read line");
    let nome = nome.to_uppercase();
    
    println!("Valor do Produto");
    let mut valor = String::new();
    io::stdin().read_line(&mut valor).expect("Failed to read line");
    let valor = valor.trim().parse::<f64>().expect("ERRO: Falha na conversão");

    println!("Margem De Lucro %");
    let mut lucro:String = String::new();
    io::stdin().read_line(&mut lucro).expect("Failed to read line");
    let lucro = lucro.trim().parse::<f64>().expect("ERRO:Falha na conversão");

    println!("Estoque inicial do produto");
    let mut estoque = String::new();
    io::stdin().read_line(&mut estoque).expect("Failed to read line");
    let estoque = estoque.trim().parse::<u16>().expect("ERRO: Falha na conversão");

    let id: u16 = rand::thread_rng().gen_range(1000..=9999);

    let produto:Produto = Produto
    {
        nome,
        valor,
        lucro,
        estoque,
        id,
    };

    produto

}

fn produtos(produto_biblioteca:&mut HashMap<u16, Produto>)
{
    loop
    {
        let mut alternativa = String::new();

        println!("-----------Produto-----------");
        println!("Escolha uma opção");
        println!("1) - lista produtos\n2) - Modifica um Produto\n3) - Criar novo Produto\n0) - Sai");

        io::stdin().read_line(&mut alternativa).expect("Failed to read line");
        let alternativa = alternativa.trim().parse::<u8>().expect("ERRO: Falha na conversão");

        match alternativa
        {
            0 => break,
            1 => listar_produto(produto_biblioteca),
            2 => modifica_produto(produto_biblioteca),
            3 => 
            {
                let produto = new_produto();
                produto_biblioteca.insert(produto.id,produto);
            },
            _ => continue,
        }
    }
}

fn listar_produto(produto_biblioteca: &HashMap<u16, Produto>)
{
    let mut num:u16 = 0;

    println!("| N° |  ID  |       NOME        | VALOR UNITÁRIO | Margem De Lucro % | PREÇO UNITÁRIO | ESTOQUE |");
    
    for produto in produto_biblioteca 
    {
        num += 1;
        println!("| {}° | {} |    {}    |   R$ {:.2}   |   {}%   |    R$ {:.2}    |  {}  |", num, produto.1.id, produto.1.nome.trim(), produto.1.valor, produto.1.lucro, produto.1.calc_preco(), produto.1.estoque);
    } 
}

fn buscar_produto(produto_biblioteca: &HashMap<u16, Produto>) -> u16
{
    if produto_biblioteca.len() == 0 {return 0}

    println!("Pesquisa por Produtos - ( 0 ) Para Sair");

    loop
    {
        println!("Nome do Produtos: ");
        
        let mut pesquisa:String = String::new();
        io::stdin().read_line(&mut pesquisa).expect("Error reading pesquisa");
        let pesquisa = pesquisa.to_uppercase();
        
        if pesquisa.trim() == "0"{break 0;}
        
        let pesquisa: Vec<&str> = pesquisa.trim().split_whitespace().collect();
        
        let mut num:u16 = 0;
        
        let mut resut_pesquisa:Vec<u16> = Vec::new();

        for (k, produto) in produto_biblioteca
        {
            let produto_nomes: Vec<&str> = produto.nome.trim().split(' ').collect();
            
            let mut comparacao_nome_produto:bool = false;

            for nome_pesquisa in &pesquisa
            {
                for nome in &produto_nomes
                {
                    if &nome_pesquisa == &nome
                    {
                        comparacao_nome_produto = true;
                        num += 1;
                        println!("{}) {} - > {} ", num, k, produto.nome);
                        resut_pesquisa.push(produto.id);
                        break;
                    }
                }
                if comparacao_nome_produto { break };
            }
        }
    
        if resut_pesquisa.len() > 1
        {
            loop 
            {
                println!("Escolha um produto(N°):");
        
                let mut opcao_pesquisa = String::new();
                io::stdin().read_line(&mut opcao_pesquisa).expect("Failed to read line");
                let opcao_pesquisa = opcao_pesquisa.trim().parse::<usize>().expect("Failed to parse");

                if opcao_pesquisa <= resut_pesquisa.len() && opcao_pesquisa > 0
                {
                    return resut_pesquisa[opcao_pesquisa-1]
                }

                else if opcao_pesquisa == 0 {break;}

                else{println!("\nOpção Invalida!\nDigite Novamente.");}
            }
        }

        else if resut_pesquisa.len() == 1 {return resut_pesquisa[0];}

        else{println!("\nProduto não encontrado!!!\n");}
    }
}

fn modifica_produto(produto_biblioteca: &mut HashMap<u16, Produto>)
{
    let mut chave_produto = 0;
    
    if produto_biblioteca.len() == 1
    {
        chave_produto = *produto_biblioteca.keys().next().unwrap();
    }
    else if produto_biblioteca.len() > 1
    {
        println!("\nEscolha um produto para Modifica");   
        chave_produto = buscar_produto(&produto_biblioteca);
    }

    if chave_produto > 0
    {
        let produto = produto_biblioteca.get_mut(&chave_produto).expect("ERRO!!");
        
        loop
        {
            println!("\n----[ {} ]----",produto.nome.trim());
            println!("| Nome: {} - Valor Unitário: R${:.2} - Margem De Lucro %: {:.2}% - Estoque: {} |\n",produto.nome.trim(), produto.valor, produto.lucro, produto.estoque);

            println!("Qual campo gostaria de altera");
            println!("1) - NOME \n2) - VALOR UNITÁRIO \n3) - MARGEM DE LUCRO % \n4) - ESTOQUE \n0) - Sair ");

            let mut opcao = String::new();
            io::stdin().read_line(&mut opcao).expect("Failed to read line");
            let opcao:u16 = opcao.trim().parse::<u16>().expect("ERRO: Falha na conversão");
            
            match opcao
            {
                0 => break,
                1 => 
                {
                    println!("Novo Nome:");
                    let mut nome:String = String::new();
                    io::stdin().read_line(&mut nome).expect("Failed to read line");
                    produto.new_nome(nome);
                },
                2 => 
                {
                    println!("Novo VALOR UNITÁRIO:");
                    let mut valor:String = String::new();
                    io::stdin().read_line(&mut valor).expect("Failed to read line");
                    let valor = valor.trim().parse::<f64>().expect("ERRO! de conversão");
                    produto.new_valor(valor)
                },
                3 => 
                {
                    println!("Novo Margem De Lucro %:");
                    let mut valor:String = String::new();
                    io::stdin().read_line(&mut valor).expect("ERRO!");
                    let valor = valor.trim().parse::<f64>().expect("ERRO! na conversão");
                    produto.new_lucro(valor)
                },
                4 => 
                {
                    println!("Novo ESTOQUE:");
                    let mut valor:String = String::new();
                    io::stdin().read_line(&mut valor).expect("ERRO!");
                    let valor = valor.trim().parse::<u16>().expect("ERRO! na conversão");
                    produto.new_estoque(valor)
                },
                _=> println!("opção invalida!"),
            }
        }
    }
}

fn new_estoque_interface(produto_biblioteca: &mut HashMap<u16, Produto>, add_estoque:bool) 
{
    let mut chave_produto = 0;

    if produto_biblioteca.len() == 1
    {
        chave_produto = *produto_biblioteca.keys().next().unwrap();
    }
    else if produto_biblioteca.len() > 1
    {
        println!("\nEscolha Um Produto Para Adicionar/Altera O Estoque");
        chave_produto = buscar_produto(&produto_biblioteca);
    }

    if chave_produto > 0
    {
        let produto = produto_biblioteca.get_mut(&chave_produto).expect("ERRO!");

        println!("\n----[ {} ]----",produto.nome.trim());
        println!("| Nome: {} - Estoque: {}.Unt |\n",produto.nome.trim(), produto.estoque);
        println!("\nValor:");

        let mut valor = String::new();
        io::stdin().read_line(&mut valor).expect("Failed to read line");
        let valor:u16 = valor.trim().parse::<u16>().expect("ERRO: Falha na conversão");

        if add_estoque { produto.new_estoque(produto.estoque + valor); }
        else { produto.new_estoque(valor); }

        //add ? produto.new_estoque(produto.estoque + valor) : produto.new_estoque(valor);
        /*
        for (k, produto) in produto_biblioteca
        {
            if k == &chave_produto && add{produto.new_estoque(produto.estoque+valor)}
            else {produto.new_estoque(valor)}
        }
        */
    }
}

fn estoque_interface(produto_biblioteca: &mut HashMap<u16, Produto>)
{
    loop 
    {      
        let mut opcao:String = String::new();
    
        println!("\n------------Estoque------------");
    
        listar_produto(produto_biblioteca);
    
        println!("Escolha uma opção");
        println!("1) - ADD Estoque\n2) - Altera Estoque\n0) - Sair");
    
        io::stdin().read_line(&mut opcao).expect("Failed to read line");
        let opcao = opcao.trim().parse::<u8>().expect("ERRO: Falha na conversão");

        match opcao
        {
            0 => break,
            1 => new_estoque_interface(produto_biblioteca, true),
            2 => new_estoque_interface(produto_biblioteca, false),
            _=>{println!("opção invalida!"); continue},
        }
    }
}

fn main()
{
    //let mut produto_list:Vec<Produto> = Vec::new();
    let mut produto_biblioteca:HashMap<u16,Produto> = HashMap::new();

    loop
    {
        let mut opcao:String = String::new();

        println!("{}","-=".repeat(20));

        println!("{:-^40}", "Mercadinho");
        println!("Escolha uma opção:");
        println!("1) - Venda\n2) - Estoque\n3) - Produto\n0) - Sair");

    
        io::stdin().read_line(&mut opcao).expect("Failed to read line");
        let opcao = opcao.trim().parse::<u8>().expect("ERRO: Falha na conversão");

        match opcao
        {
            0 => break,
            //1 => ,
            2 => estoque_interface(&mut produto_biblioteca),
            3 => produtos(&mut produto_biblioteca),
            _=>{println!("opção invalida!"); continue},
        }
    }

}
// dividi string -> split_whitespace()