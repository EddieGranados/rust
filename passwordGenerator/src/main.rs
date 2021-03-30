fn main() {
    use rand::Rng;
    const CHARSET: &[u8] = b"abcdefghijklmnopqrstuvwxyz\
                            ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            0123456789)(*&^%$#@!~";
    const PASSWORD_LEN: usize = 20;
    let mut rng = rand::thread_rng();

    // Creating a password of String type that is a tuple from 0 to 12,
    // With the map function we will iterate through the tuple
    // At each iteration, a random index value will be generated from 0 to the length of CHARSET
    // Then, the random index is used to choose a random value from CHARSET, 
    // but CHARSET is currently in bytes,it needs to be converted back to char
    let password: String = (0..PASSWORD_LEN)
                                            .map(|_| {
                                                let idx = rng.gen_range(0..CHARSET.len());
                                                CHARSET[idx] as char                
    })
    .collect();

    println!("\npassword: {:?}", password);


}
