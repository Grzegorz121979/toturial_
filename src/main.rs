use rand::Rng;

fn main() {
    let mut vec: Vec<i32> = Vec::new();

    let random_int_vec: &Vec<i32> = even_generate_vec(&mut vec);
    
    println!("{:?}", random_int_vec);
}

fn even_generate_vec(v: &mut Vec<i32>) -> &Vec<i32> {
    let mut count: i32 = 0;

    while count <= 10 {
        let rand_number: i32 = rand::thread_rng().gen_range(1..=11);
        v.push(rand_number);
        count += 1;
    }
    v.retain(|x: &i32| x % 2 == 0);
    v
}
