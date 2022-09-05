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
    //let mut ts: [Produto;1] = [];

    let test = new_produto(String::from("Tss"));
    
    println!("{}",test.nome);
}
