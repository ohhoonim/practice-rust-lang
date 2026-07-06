// Filter와 Map 체이닝

use std::io::{self, BufRead, BufReader, Read, Write};

fn process_numbers(numbers: Vec<i32>) -> Vec<i32> {
    numbers
        .into_iter()
        .filter(|n| n % 2 == 0)
        .map(|n| n * n)
        .collect()
}

// FlatMap과 Collect

struct Department {
    name: String,
    employees: Vec<String>,
}

fn gather_all_employees(departments: Vec<Department>) -> Vec<String> {
    departments.into_iter().flat_map(|e| e.employees).collect()
}

// Reduting
struct Product {
    name: String,
    price: u32,
}
fn calculate_total_price(products: &[Product]) -> u32 {
    products.iter().fold(0, |acc, p| acc + p.price)
}

// String과 &str 변환
fn generate_uppercase_fullname(first: &str, last: &str) -> String {
    format!("{} {}", first, last).to_uppercase()
}

// BufRead
fn read_lines_from_stream<R: Read>(stream: R) -> io::Result<Vec<String>> {
    let reader = BufReader::new(stream);
    let mut lines_vec = Vec::new();

    for line in reader.lines() {
        lines_vec.push(line?);
    }

    Ok(lines_vec)
}

// Write 트레이트

fn write_items<W: Write>(writer: &mut W, items: &[&str]) -> io::Result<()> {
    for item in items {
        writeln!(writer, "{}", item)?;
    }
    Ok(())
}

#[cfg(test)]
#[path = "iterator_pipeline_test.rs"]
mod iterator_pipeline_test;
