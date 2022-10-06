use std::io;
use std::fmt::Display;
use rand::Rng;
use std::collections::HashMap;

extern crate rand;
//#[derive(Debug)]
struct Produto
{
    nome: String,
    valor: f64,
    estoque: u16,
    id: u16,
    
}

impl Produto
{
    fn add_estoque(&mut self, valor:u16)
    {
        self.estoque += valor;
    }
}

fn new_produto() -> Produto
{
    println!("Nome do Produto");
    let mut nome:String = String::new();
    io::stdin().read_line(&mut nome).expect("Failed to read line");
    let mut nome = nome.to_uppercase();
    
    println!("Valor do Produto");
    let mut valor = String::new();
    io::stdin().read_line(&mut valor).expect("Failed to read line");
    let mut valor = valor.trim().parse::<f64>().expect("ERRO: Falha na conversão");

    println!("Estoque inicial do produto");
    let mut estoque = String::new();
    io::stdin().read_line(&mut estoque).expect("Failed to read line");
    let mut estoque = estoque.trim().parse::<u16>().expect("ERRO: Falha na conversão");

    let id: u16 = rand::thread_rng().gen_range(1000..=9999);

    let produto:Produto = Produto
    {
        nome,
        valor,
        estoque,
        id,
    };
    
    produto

}

fn produtos(produto_list:&mut HashMap<u16, Produto>) -> bool
{
    loop
    {
        let mut alternativa = String::new();

        println!("-----------Produto-----------");
        println!("Escolha uma opção");
        println!("1) - lista produtos\n2) - Modifica um Produto\n3) - Criar novo Produto\n0) - Sai");

        io::stdin().read_line(&mut alternativa).expect("Failed to read line");
        let mut alternativa = alternativa.trim().parse::<u8>().expect("ERRO: Falha na conversão");

        match alternativa
        {
            0 => return false,
            1 => listar_produto(produto_list),
            //2 => ,
            3 => return true,
            _ => continue,
        }
    }  
}

fn listar_produto(list: &mut HashMap<u16, Produto>)
{
    let mut num:u16 = 0;

    println!("| N° | ID |       NOME        | VALOR UNITÁRIO | LUCRO % | VALOR + LUCRO | ESTOQUE |");
    
    for (k,p) in list
    {
        num += 1;
        println!("| {}° | {} |...{}...R$ {}...Lucro %...VL...{} |", num, p.id, p.nome.trim(), p.valor, p.estoque);
    } 
}

fn buscar_produto(produto_biblioteca: &HashMap<u16, Produto>) -> u16
{
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
    
        if resut_pesquisa.len() > 0
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
            }
        }
    }
}

fn add_estoque_interface(produto_biblioteca: &mut HashMap<u16, Produto>)
{
    let mut valor = String::new();
    let mut chave_produto : u16 = 0;

    println!("\nEscolha um produto para adicionar mais estoque");
    
    chave_produto = buscar_produto(&produto_biblioteca);

    if chave_produto > 0
    {
        println!("\nValor:");
    
        io::stdin().read_line(&mut valor).expect("Failed to read line");
        let valor = valor.trim().parse::<u16>().expect("ERRO: Falha na conversão");
    
        //let produto = produto_biblioteca.get(&chave_produto);
        //let mut produto = produto.add_estoque(valor);
    
        for (k, produto) in produto_biblioteca
        {
            if k == &chave_produto{produto.add_estoque(valor)}
        }
        //produto_biblioteca.get(&chave_produto).expect("ERRO:Falha").add_estoque(valor);
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
        println!("1) ADD Estoque\n2) Altera Estoque\n0) Sair");
    
        io::stdin().read_line(&mut opcao).expect("Failed to read line");
        let opcao = opcao.trim().parse::<u8>().expect("ERRO: Falha na conversão");

        match opcao
        {
            0 => break,
            1 => add_estoque_interface(produto_biblioteca),
            //2 => ,
            _=>{println!("opção invalida!"); continue},
        }
    }
}

fn main()
{
    let mut produto_list:Vec<Produto> = Vec::new();
    let mut produto_biblioteca:HashMap<u16,Produto>= HashMap::new();

    loop
    {
        let mut opcao:String = String::new();

        println!("------------Mercadinho------------");
        println!("Escolha uma opção:");
        println!("1) Venda\n2) Estoque\n3) Produto\n0) Sair");

    
        io::stdin().read_line(&mut opcao).expect("Failed to read line");
        let opcao = opcao.trim().parse::<u8>().expect("ERRO: Falha na conversão");

        match opcao
        {
            0 => break,
            //1 => ,
            2 => estoque_interface(&mut produto_biblioteca),
            3 => 
            {
                if produtos(&mut produto_biblioteca)
                {
                    //produto_list.push(new_produto());
                    let produto = new_produto();
                    produto_biblioteca.insert(produto.id,produto);
                }
            },
            _=>{println!("opção invalida!"); continue},
        }
    }

}
// dividi string -> split_whitespace()