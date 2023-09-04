use std::fs::File;
use std::io::Write;

fn delete_file(file_path: String) {
    if !std::path::Path::new(&file_path).exists() {
        return;
    }

    std::fs::remove_file(file_path)
        .expect("Something went wrong deleting the file");
}

fn write_file(file_name: String, binary: Vec<i64>) {
    let mut file = File::create(file_name)
        .expect("Something went wrong creating the file");

    for byte in binary {
        file.write_all(&byte.to_le_bytes())
            .expect("Something went wrong writing the file");
    }
}

pub fn write_binary(binary: Vec<i8>) {
    let mut full_binary: Vec<i64> = Vec::new();
    let mut i: i64 = 0;
    let mut j: i16 = 0;

    full_binary.push(0);

    let reversed_binary: Vec<i8> = binary
        .iter()
        .rev()
        .cloned()
        .collect();

    for bit in reversed_binary {
        if j >= 64 {
            i += 1;
            j = 0;

            full_binary.push(0);
        }

        if bit == 0 {
            full_binary[i as usize] &= !(1 << j);
        } else if bit == 1 {
            full_binary[i as usize] |= 1 << j;
        }

        j += 1;
    }

    println!();

    delete_file("a.out".to_string());
    write_file("a.out".to_string(), full_binary);
}
