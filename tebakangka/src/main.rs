use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {

    let start_val = 1;
    let end_val = 10;

    let nom_rah = rand::thread_rng().gen_range(start_val,end_val);
    
    println!("Tebak Angkanya ya antara {} dan {}!", start_val, end_val);
    loop{
    
        let mut tebakan = String::new();
        println!("Silangkah Masukan angka :");


        io::stdin().read_line(&mut tebakan).expect("Gagal membaca line");
        let tebakan: u32 = match tebakan.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        match tebakan.cmp(&nom_rah) {
            Ordering::Less => println!("Terlalu Kecil"),
            Ordering::Greater => println!("Terlalu Besar"),
            Ordering::Equal => {
                println!("Anda Menang");
                break;
            }
        }
    }
}
