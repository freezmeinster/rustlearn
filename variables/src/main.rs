fn main() {
    // Jadi di rust itu angka bisa dikasi separtor underscore
    const MAX_UMUR: i32 = 56_000_000;

    let mut x :i32 = 5;
    println!("Nilai dari x adalah {}", x);
    x = 6;
    println!("Nilai dari x sekarang adalah {}", x);
    println!("Umur saya adalah {}", MAX_UMUR);
    

    // Ini namanya shadowing.
    let b = 2;

    println!("Nilai B = {}", b);
    let b = b + 5;

    println!("Nilai B = {}", b);
    let b = b + 8;

    println!("Nilai B = {}", b);
}
