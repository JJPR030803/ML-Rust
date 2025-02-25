use rand::Rng;

pub fn generar_lista(min:i32,max:i32,n:usize) -> Vec<i32>{
    let mut rng = rand::thread_rng();
    (0..n).map(|_| rng.gen_range(min..=max)).collect()
}

pub fn generar_matriz(min:i32,max:i32,n:usize,m:usize)-> Vec<Vec<i32>> {
    (0..m).map(|_| generar_lista(min, max, n)).collect()
    
}