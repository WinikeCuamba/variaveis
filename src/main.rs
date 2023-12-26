fn main() {
    let x = 5;
   
    let x = x + 5;
    let x = x - 3;

    println!("O valor de x e = {}", x);
   
   // -----------------------------------
   let espacos = "   ";
   let espacos = espacos.len();

   println!("O tamanho de espacos e de {}", espacos);

    // ----------------------------------
    let _guess: u32 = "43".parse().expect("O valor nao e um numero");

    let tup: (u32, f32, u8) = (31, 13.41, 3);
    // let (x , y, v) = tup;

    println!("O valor de index 1 na tupla e {}", tup.1);

    // ---------------------------------
    let a = [1 , 2, 3, 4, 5];
    println!("O valor do index 4 da matrix e {} ", a[4]);
    // -----------------------------------
    outra_funcao();

}


fn outra_funcao() {
    println!("Outra funcao");
}
