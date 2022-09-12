use std::io;
//#[derive(Debug)]
struct Produto
{
    nome: String,
    valor: f64,
    estoque: u16,
    
}

fn new_produto() -> Produto
{
    println!("Nome do Produto");
    let mut nome:String = String::new();
    io::stdin().read_line(&mut nome).expect("Failed to read line");
    
    println!("Valor do Produto");
    let mut valor = String::new();
    io::stdin().read_line(&mut valor).expect("Failed to read line");
    let mut valor = valor.trim().parse::<f64>().expect("ERRO: Falha na conversão");

    println!("Estoque inicial do produto");
    let mut estoque = String::new();
    io::stdin().read_line(&mut estoque).expect("Failed to read line");
    let mut estoque = estoque.trim().parse::<u16>().expect("ERRO: Falha na conversão");

    let produto = Produto
    {
        nome,
        valor,
        estoque,
    };
    
    produto

}

fn produtos(produto_list:&mut [Produto]) -> bool
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
            1 => listar_produto(&produto_list),
            //2 => ,
            3 => return true,
            _ => continue,
        }
    }  
}

fn listar_produto(list: &[Produto])
{
    println!("| N° | ID |       NOME        | VALOR UNITÁRIO | LUCRO % | VALOR + LUCRO | ESTOQUE |");
    
    for p in list
    {
        println!("| N° | ID |       {}      | R$ {} | Lucro % | VL | {} |", p.nome.trim(), p.valor, p.estoque);
    } 
}

fn add_estoque(produto_list:&mut [Produto], index_produto:usize)
{
    if produto_list.len() > 0
    {
        produto_list[index_produto].estoque += 1;
    }
    else
    {
        println!("Não a produtos cadastrados ainda");
    }

}

fn estoque_interface(produto_list:&mut [Produto])
{
    loop 
    {      
        let mut opcao:String = String::new();
    
        println!("------------Estoque------------");
    
        listar_produto(&produto_list);
    
        println!("Escolha uma opção");
        println!("1) ADD Estoque\n2) Altera Estoque\n0) Sair");
    
        io::stdin().read_line(&mut opcao).expect("Failed to read line");
        let opcao = opcao.trim().parse::<u8>().expect("ERRO: Falha na conversão");

        match opcao
        {
            0 => break,
            1 => add_estoque(produto_list, 0),
            //2 => ,
            _=>{println!("opção invalida!"); continue},
        }
    }
}

fn main()
{
    let mut produto_list:Vec<Produto> = Vec::new();

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
            2 => estoque_interface(&mut produto_list),
            3 => 
            {
                if produtos(&mut produto_list)
                {
                    produto_list.push(new_produto());
                }
            },
            _=>{println!("opção invalida!"); continue},
        }
    }

}
