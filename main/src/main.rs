//#[derive(Debug)]
struct Produto
{
    nome: String,
    
}

fn new_produto(nome:String) -> Produto
{
    let produto = Produto
    {
        nome: nome,
    };
    
    produto

}

fn main()
{
    let mut ts = Vec::new();
    
    ts.push(new_produto(String::from("MA")));
    
    let ts1 = &ts[0];
    println!("{}", ts1.nome);
}
