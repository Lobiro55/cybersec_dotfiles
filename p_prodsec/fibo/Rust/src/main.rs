fn main() {
    // inicializamos la variable y que sera el numero de veces que se ejecutar la secuencia
    let z = 10;
    // creamos  un bucle que llame a la función fibo las veces que sea necesario usando el 0..z
    for i in 0..z {
        //imprimimos el valor que nos da la función fibo
        println!("el valor es {}", fibo(i));
    }
}

// creamos la función fibo y escribimos el :u32 para decir que la variable es un integer de 32 bits
fn fibo(z: u32) -> u32 {
    //inicializamos la variable x y la hacemos mutable para que pueda cambiar de valor en cualquier momento
    let mut x = 0;
    //inicializamos la variable x y la hacemos mutable para que pueda cambiar de valor en cualquier momento
    let mut y = 1;
    /*
    creamos un bucle for  para que  nos de el numero de la secuencia que buscamos el "0..z " hace referencia a que el valor
    de las iteraciones los da una variable no un numero exacto
    */
    for _ in 0..z {
        //inicializamos la variable xy que le vamos a dar el valor de x en cada iteración
        let xy = x;
        x = y;
        y = xy + x;
    }

    x
}

/*
configuramos los test en rust, diciéndole al compilador que esto son test y que en la version final no tiene que compilar esta parte
es solo para pruebas
*/
#[cfg(test)]
// definimos el modulo tests que sera donde estén nuestras funciones  para probar el código
mod tests {
    // Le damos permiso al modulo de que puede usar cualquier función que este en el modulo principal
    use super::*;
    // Marcamos que la siguientes funciones son de testeo
    #[test]
    //creamos la función fibo_test para hacer la prueba
    fn fibo_test() {
        assert_eq!(fibo(3), 2);
    }
}
