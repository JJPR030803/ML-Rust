
use rand::Rng;


pub fn rand_list_f64(n:usize,min:f64,max:f64) -> Vec<f64>{
    let mut rng = rand::thread_rng();

    (0..n).map(|_|{
        let num:f64 = rng.gen_range(min..max);
        (num * 100.0).round() / 100.0
    }).collect()
    
}