use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use std::default::Default;
use std::io::Cursor;

pub trait HasSortData {
    fn bytes_count() -> u8;
    fn bytes(num: Self) -> Vec<u8>;
    fn is_signed() -> bool;
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

    fn is_signed() -> bool {
        false
    }
}

impl HasSortData for i32 {
    fn bytes_count() -> u8 {
        4
    }

    fn bytes(num: i32) -> Vec<u8> {
        let mut wtr = vec![];
        wtr.write_i32::<LittleEndian>(num)
            .expect("What could go wrong...");
        wtr
    }

    fn is_signed() -> bool {
        true
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

    fn is_signed<T>() -> bool
    where
        T: HasSortData,
    {
        T::is_signed()
    }
}

pub fn sort<T>(coll: &Vec<T>) -> Vec<T>
where
    T: HasSortData + Copy + Default,
{
    let mut v = coll.clone();
    let bc = GetSortData::bytes_count::<T>();
    let is_signed = GetSortData::is_signed::<T>();
    for i in 0..bc {
        let mut v2 = vec![Default::default(); v.len()];
        let v_byte = v
            .iter()
            .map(|&x| {
                let byte = GetSortData::bytes(x)[i as usize];
                if is_signed && (i == bc - 1) {
                    let i8_byte = Cursor::new(vec![byte]).read_i8().unwrap();
                    (i8_byte as i16 + 128) as u8
                } else {
                    byte
                }
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
    v
}

#[test]
fn positives_test() {
    let v: Vec<u32> = vec![1723, 456, 76, 85, 431, 31, 904];
    let v1 = sort(&v);
    assert_eq!(v1, vec![31, 76, 85, 431, 456, 904, 1723]);
}

#[test]
fn negatives_test() {
    let v: Vec<i32> = vec![-1337, -1338, -420, -1420];
    let v1 = sort(&v);
    assert_eq!(v1, vec![-1420, -1338, -1337, -420]);
}

#[test]
fn all_in() {
    let v: Vec<i32> = vec![-1337, 345, -1338, 0, -420, 0, -1420, 420, 4, 3, -9999, 432];
    let v1 = sort(&v);
    assert_eq!(
        v1,
        vec![-9999, -1420, -1338, -1337, -420, 0, 0, 3, 4, 345, 420, 432]
    );
}
