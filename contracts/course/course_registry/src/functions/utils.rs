// SPDX-License-Identifier: MIT
// Copyright (c) 2025 SkillCert

pub use crate::schema::{Course, CourseModule};
use soroban_sdk::{Bytes, Env, String, Vec, vec};

pub fn generate_unique_id(env: &Env) -> String {
    let ts: u64 = env.ledger().timestamp();
    let rand1: u64 = env.prng().gen();
    let rand2: u64 = env.prng().gen();

    let str_vec = vec![
            &env, 
            u32_to_string(&env, ts as u32),
            String::from_str(&env, "-"), 
            u32_to_string(&env, rand1 as u32),
            String::from_str(&env, "-"),
            u32_to_string(&env, rand1 as u32),
            String::from_str(&env, "-"),
            u32_to_string(&env, rand2 as u32),
            String::from_str(&env, "-"),
            u32_to_string(&env, rand2 as u32),
            String::from_str(&env, "-"),
            ];   

    let rust_str = concat_strings(&env, str_vec);
    rust_str
}

pub fn to_lowercase(env: &Env, s: &String) -> String {
    let len: u32 = s.len() as u32;
    let mut buffer = [0u8; 1024]; 
    let slice = &mut buffer[..len as usize];
    s.copy_into_slice(slice);
    let mut result_bytes = Bytes::new(env);

    for byte in slice.iter() {
        if *byte >= b'A' && *byte <= b'Z' {
            result_bytes.push_back(*byte + (b'a' - b'A'));
        } else {
            result_bytes.push_back(*byte);
        }
    }

    let mut result = [0u8; 1024]; 
    let new_slice = &mut result[..len as usize];
    result_bytes.copy_into_slice(new_slice);
    String::from_bytes(env, &new_slice)
}

pub fn u32_to_string(env: &Env, n: u32) -> String {
        // Simple conversion: handle 0 and build digits
        let mut len = 0;

        if n == 0 {
            return String::from_str(env, "0");
        }
        let mut digits = Vec::<u32>::new(env);
        let mut num = n;
        while num > 0 {
            len += 1;
            let digit = (num % 10) as u8;
            digits.push_front((b'0' + digit).into());
            num /= 10;
        }
        let mut bytes = Bytes::new(env);
        for digit in digits.iter() {
            bytes.push_back(digit.try_into().unwrap());
        }

        let mut result = [0u8; 1024]; 
        let new_slice = &mut result[..len as usize];
        bytes.copy_into_slice(new_slice);
        String::from_bytes(env, &new_slice)
}

pub fn trim(env: &Env, s: &String) -> String {
    // Create a fixed-size buffer for the string's bytes
    let len = s.len() as usize;
    let mut byte_array: [u8; 1024] = [0u8; 1024]; 
    if len > byte_array.len() {
        panic!("String too long for fixed-size buffer");
    }
    s.copy_into_slice(&mut byte_array[..len]);

    // Create a Bytes object from the buffer
    let bytes = Bytes::from_slice(env, &byte_array[..len]);

    // Find the first non-whitespace character
    let mut start = 0;
    while start < bytes.len() {
        let mut byte_buffer: [u8; 1] = [0u8; 1];
        bytes.slice(start..start + 1).copy_into_slice(&mut byte_buffer);
        let byte = byte_buffer[0];
        if byte != 32 && byte != 9 {
            break;
        }
        start += 1;
    }

    // Find the last non-whitespace character
    let mut end = bytes.len();
    while end > start {
        let mut byte_buffer: [u8; 1] = [0u8; 1];
        bytes.slice(end - 1..end).copy_into_slice(&mut byte_buffer);
        let byte = byte_buffer[0];
        if byte != 32 && byte != 9 {
            break;
        }
        end -= 1;
    }

    // Create a trimmed Bytes object
    let trimmed_bytes = bytes.slice(start as u32..end as u32);

    let mut result = [0u8; 1024]; 
    let new_slice = &mut result[..(end - start) as usize];
    trimmed_bytes.copy_into_slice(new_slice);
    String::from_bytes(env, &new_slice)
}

pub fn concat_strings(env: &Env, strings: Vec<String>) -> String {
    let mut result_byte = Bytes::new(env);
    let mut total_len = 0;

    for s in strings {
        let s_len = s.len() as usize;
        total_len += s_len;
        let mut buffer = [0u8; 1024]; 
        let mut new_slice = &mut buffer[..s_len as usize];
        s.copy_into_slice(&mut new_slice);  
        result_byte.extend_from_slice(&new_slice);
    }

    let mut result = [0u8; 1024]; 
    let new_slice = &mut result[..total_len as usize];
    result_byte.copy_into_slice(new_slice);
    String::from_bytes(env, &new_slice)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schema::{Course};

    use crate::CourseRegistry;
    use soroban_sdk::{testutils::Address as _, Address, Env, vec};

    fn create_test_course(env: &Env, id: &str) -> Course {
        Course {
            id: String::from_str(env, id),
            title: String::from_str(env, "Test Course"),
            description: String::from_str(env, "Test Description"),
            creator: Address::generate(env),
            price: 1000,
            category: None,
            language: None,
            thumbnail_url: None,
            published: false,
            prerequisites: Vec::new(&env),
            is_archived: false,

            duration_hours: Some(1),
            level: Some(String::from_str(env, "entry"))

        }
    }

    #[test]
    fn test_delete_course_success() {
        let env = Env::default();
        let _contract_id = env.register(CourseRegistry, {});
        let course_id = String::from_str(&env, "course_1");
        let _course = create_test_course(&env, "course_1.");
        let course_id2 = String::from_str(&env, "    course_1.  ");

        let counter = 5;

        let arr = vec![
            &env, String::from_str(&env, "module_"), 
            course_id.clone(), 
            String::from_str(&env, "_"),
            u32_to_string(&env, counter),
            String::from_str(&env, "_0"),
            ];   

        let module_id = concat_strings(&env, arr);

        let count = u32_to_string(&env, counter);
        let lowercase_result = to_lowercase(&env, &course_id);
        let trim_result = trim(&env, &course_id2);
        
        // You can add assertions here if needed for testing
        assert!(!count.is_empty());
        assert!(!module_id.is_empty());
        assert!(!lowercase_result.is_empty());
        assert!(!trim_result.is_empty());
    }
}