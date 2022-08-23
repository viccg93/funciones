//la funcion main es la funcion especial donde inician los programas en Rust
fn main() {
    hey_crustaceans();
    say_hey('🦀');
    dummy_function();
    let x = five();
    println!("el valor de x es {x}");
}

//la definicion de una funcion se realiza con la palabra reservada fn, seguido del nombre y parentesis
//en los parentesis se indican los parametros
//la convencion de Rust para funciones y variables es snake
fn hey_crustaceans(){
    println!("Hey crustaceans!");
}

//se pueden incluir multiples parametros, pero es forzoso utilizar la anotacion del tipo (parte del diseño de Rust)

fn say_hey(emoji: char){
    println!("{emoji} {emoji} Hey crustaceans {emoji} {emoji}")
}

//Rust es un lenguaje basado en expresiones, por lo que existe una diferencia clave entre statements y expressions
//un statement no resuelve a un valor, es solo una instraccion
//un ejemplo claro de statement es una asignacion como let x = 20; que no devuelve nada, solo asigna un valor.
//una expresion si resuelve hacia un valor, por ejemplo una operacion matematica 1+2, tiene un resultado o resolucion.
//las llamadas de funciones o macros y los scopes generados con curly braces son expresiones que devuelven algo o tienen resoluciones.

//la siguiente funcion tiene un scope que se asigna a la variable y
//dentro del scope estamos creando una variable, al ser una definicion de variable es un statement
//posteriormente creamos una expresion que es x+1, esta no es una asignacion, es una expresion que devuelve un resultado
//dicho resultado se asigna a y.

//la expresion no tiene punto y coma, si se le agrega se convierte en statement y tendriamos un error de compilacion.

fn dummy_function(){
    let y = {
        let x = 3;
        x+1
    };
    println!("{y}");
}

//los tipos de retorno se deben indicar con -> y son obligatorios
fn five () -> u8 {
    5
}

