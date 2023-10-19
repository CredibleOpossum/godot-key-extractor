use std::{env, fs::File, io::Read, process::exit};

const ADDRESS_SIZE: usize = 4;
const KEY_SIZE: usize = 32;
const VIRTUAL_ADDRESS_OFFSET: usize = 0x00400000; // Research this

struct Pattern {
    name: String,
    bytes: Vec<u8>,
    offset: usize,
    implemented: bool,
}
impl Pattern {
    fn new(name: String, bytes: &[u8], offset: usize, implemented: bool) -> Pattern {
        Pattern {
            name,
            bytes: bytes.to_vec(),
            offset,
            implemented,
        }
    }
}

fn find_subset_of_bytes(haystack: &[u8], needle: &[u8]) -> Vec<usize> {
    // Slight issue, if looking for 0xff byte it will treat it as a wildcard, haven't found the need for it yet
    haystack
        .windows(needle.len())
        .enumerate()
        .filter(|(_, window)| {
            window
                .iter()
                .zip(needle.iter())
                .all(|(&a, &b)| b == 0xFF || a == b)
        })
        .map(|(index, _)| index)
        .collect()
}

fn bytes_to_hex_string(bytes: &[u8]) -> String {
    bytes
        .iter()
        .map(|byte| format!("{:02X}", byte))
        .collect::<Vec<String>>()
        .join("")
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let mut file = match File::open(&args[1]) {
            Ok(file) => file,
            Err(why) => {
                eprintln!("{}", why);
                exit(1);
            }
        };
        let patterns = [
            Pattern::new(
                // Linux pattern.
                "Linux".into(),
                &[0x00, 0x45, 0x0f, 0xb6, 0xac],
                6,
                true,
            ),
            Pattern::new(
                // Windows pattern, less reliable, not able to read key without external tools
                "Windows".into(),
                &[0x00, 0x00, 0x00, 0xff, 0x0f, 0xb6, 0xff, 0x1e, 0x85],
                0, // Operand doesn't contain location, manual for now
                false,
            ),
        ];

        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).unwrap();

        for pattern in patterns {
            let possible_vectors = find_subset_of_bytes(&buffer, &pattern.bytes);
            for vector in possible_vectors {
                let important_location = vector + pattern.offset;
                print!("{}:{:x} -> ", pattern.name, important_location);
                let read_address: [u8; 4] = TryFrom::try_from(
                    &buffer[important_location..important_location + ADDRESS_SIZE],
                )
                .unwrap(); // Address size always = 4, unwrap SHOULD never fail
                let address_base: usize = u32::from_le_bytes(read_address).try_into().unwrap();
                let should_extract_key = address_base > VIRTUAL_ADDRESS_OFFSET
                    && address_base < buffer.len()
                    && pattern.implemented;
                if should_extract_key {
                    let address = address_base - VIRTUAL_ADDRESS_OFFSET;
                    let key = &buffer[address..address + KEY_SIZE];
                    println!("{}", bytes_to_hex_string(key));
                } else {
                    println!(
                        "FAILED (Ghidra memory search -> {})",
                        bytes_to_hex_string(
                            &buffer[important_location - 4..important_location + 4]
                        )
                    )
                }
            }
        }
    }
}
