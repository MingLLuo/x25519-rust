mod field;
mod hex;
mod montgomery;
mod x25519;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 && args.len() != 3 {
        eprintln!("Usage: {} <m_hex32> [u_hex32]", args[0]);
        std::process::exit(1);
    }

    let m = match hex::hex_decode::<32>(&args[1]) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Invalid m: {e}");
            std::process::exit(1);
        }
    };

    let u = if args.len() == 2 {
        let mut u = [0u8; 32];
        u[0] = 9;
        u
    } else {
        match hex::hex_decode::<32>(&args[2]) {
            Ok(v) => v,
            Err(e) => {
                eprintln!("Invalid u: {e}");
                std::process::exit(1);
            }
        }
    };

    let out = x25519::x25519(m, u);

    println!("{}", hex::hex_encode(&out));
}
