use byteorder::{LittleEndian, WriteBytesExt};

fn main() {
    let mut v = [1723, 456, 76, 85, 431, 31, 904];
    let mut v2 = [0; 7];

    for i in 0..4 {
        let v_byte = v
            .iter()
            .map(|&x| {
                let mut wtr = vec![];
                wtr.write_u32::<LittleEndian>(x).unwrap();
                wtr[i]
            })
            .collect::<Vec<u8>>();

        let mut counter = [0; 256];
        v_byte.iter().for_each(|&b| counter[b as usize] += 1);

        for j in 1..256 {
            counter[j] += counter[j - 1];
        }
        v_byte.iter().enumerate().rev().for_each(|(j, &b)| {
            counter[b as usize] -= 1;
            v2[counter[b as usize]] = v[j]
        });
        v = v2;
    }

    println!("Yo: {:?}", v.to_vec());
}
