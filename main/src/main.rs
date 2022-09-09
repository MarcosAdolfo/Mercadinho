use std::io;
//#[derive(Debug)]
struct Produto
{
    nome: String,
    valor: f64,
    
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

    let produto = Produto
    {
        nome,
        valor,
    };
    
    produto

}

fn listar_produto(list: &[Produto])
{
    for p in list
    {
        println!("-> {}  R$ {}!!!", p.nome.trim(), p.valor);
    }
}

fn main()
{
    let mut produtoList = Vec::new();
    loop
    {
        let mut alternativa = String::new();

        println!("0) - Sai");
        println!("1) - Cadastra Novo Produto");
        println!("1) - Listar Produtos");

        io::stdin().read_line(&mut alternativa).expect("Failed to read line");
        let mut alternativa = alternativa.trim().parse::<u8>().expect("ERRO: Falha na conversão");

        match alternativa
        {
            0 => break,
            1 => produtoList.push(new_produto()),
            2 => listar_produto(&produtoList),
            _ => continue,
        }
    }
    
}
