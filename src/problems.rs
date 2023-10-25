use std::collections::HashMap;

fn convert_decimal_to_binary(number: u32) -> String {
    format!("{:b}", number)
}

fn convert_nums_to_binary(nums: Vec<u32>) -> Vec<String> {
    let mut numbs_convert: Vec<String> = vec![];
    for num in 0..nums.len() {
        numbs_convert.insert(num, convert_decimal_to_binary(nums[num]));
    }
    numbs_convert
}

fn count_times_one(binary: &String) -> HashMap<String, usize> {
    let binary_vector = binary.chars().map(|x| x.to_string()).collect::<Vec<String>>();
    let mut map_count: HashMap<String, usize> = HashMap::new();
    for index in 0..binary_vector.len() {
        if !map_count.contains_key(&*binary_vector[index]) {
            map_count.insert(binary_vector[index].clone(), 1);
        } else {
            map_count.entry(binary_vector[index].clone()).and_modify(|x| *x += 1);
        }
    }
    map_count
}

fn sanitize_information(nums_binary: Vec<String>, k: usize) -> Vec<String> {
    let bit_one: u8 = 1;
    let mut elements: Vec<String> = vec![];
    let mut index: usize = 0;
    for i in 0..nums_binary.len() {
        let count_element = count_times_one(&nums_binary[i]);
        if count_element.contains_key(&*bit_one.to_string()) {
            let element = count_element.get_key_value(&*bit_one.to_string()).unwrap();
            if *element.1 == k {
                elements.insert(index, nums_binary[i].clone());
                index += 1;
            }
        }
    }
    elements
}

pub fn sum_finally_binary(numbs: Vec<String>) -> String {
    let mut result: usize = 0;
    for index in 0..numbs.len() {
        result += usize::from_str_radix(&*numbs[index], 2).unwrap();
    }
    format!("{:b}", result)
}

