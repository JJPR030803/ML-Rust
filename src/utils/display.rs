//Usado solo para debug y mostrar resultados

use std::io::repeat;

pub fn print_resultados_torneo(parent_matrix:[[[f64;3];2];3]){
    println!("Resultados Torneo Binario:");
    println!("{}","-".repeat(50));

    for i in 0..parent_matrix.len(){
        let min_value = parent_matrix[i][0][0];
        let formatted_min = format!("{:.2}",min_value);
        println!("Padre {}:", i + 1);
        println!("Valor Minimo: {}", formatted_min);
        print!("Vector: [");
        for j in 0..parent_matrix[i][1].len() {
            print!("{}", parent_matrix[i][1][j]);
            if j < parent_matrix[i][1].len() - 1 {
                print!(", ");
            }
        }
        println!("]");
        println!("{}", "-".repeat(50));
    }
}

pub fn print_diccionarios<K, V>(diccionario: &std::collections::HashMap<K, V>)
where
    K: std::fmt::Debug,
    V: std::fmt::Debug,
{
    for (key, value) in diccionario {
        println!("Key: {:?}, Type: {}", key, std::any::type_name::<K>());
        println!("Valor: {:?}, Type: {}\n", value, std::any::type_name::<V>());
    }
}