pub fn greet() {
    let hello_en = "Hello, world!";
    let hello_fr = "Bonjour, monde!";
    let hello_gr = "Grüß Gott!";
    let hello_jp = "ハロー・ワールド";
    let hellos = [hello_en, hello_fr, hello_gr, hello_jp];
    for hello in hellos.iter() {
        println!("{}", &hello);
    }
}