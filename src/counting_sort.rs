use byteorder::{LittleEndian, WriteBytesExt};
use std::default::Default;

pub trait HasSortData {
    fn bytes_count() -> u8;
    fn bytes(num: Self) -> Vec<u8>;
}

impl HasSortData for u32 {
    fn bytes_count() -> u8 {
        4
    }

    fn bytes(num: u32) -> Vec<u8> {
        let mut wtr = vec![];
        wtr.write_u32::<LittleEndian>(num)
            .expect("What could go wrong...");
        wtr
    }
}

struct GetSortData;
impl GetSortData {
    fn bytes_count<T>() -> u8
    where
        T: HasSortData,
    {
        T::bytes_count()
    }

    fn bytes<T>(num: T) -> Vec<u8>
    where
        T: HasSortData,
    {
        T::bytes(num)
    }
}

pub fn sort<T>(coll: &Vec<T>) -> Vec<T>
where
    T: HasSortData + Copy + Default,
{
    let mut v = coll.clone();
    for i in 0..GetSortData::bytes_count::<u32>() {
        let mut v2 = vec![Default::default(); v.len()];
        let v_byte = v
            .iter()
            .map(|&x| GetSortData::bytes(x)[i as usize])
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
    v
}

#[test]
fn positives_test() {
    let v = vec![1723, 456, 76, 85, 431, 31, 904];
    let v1 = sort(&v);
    assert_eq!(v1, vec![31, 76, 85, 431, 456, 904, 1723]);
}
