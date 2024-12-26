pub fn calculate_entropy(data: &[u8]) -> f64 {
    let mut freq = [0u32; 256];
    for &byte in data {
        freq[byte as usize] += 1;
    }
    let len = data.len() as f64;
    freq.iter().map(|&count| {
        if count == 0 {
            0.0
        } else {
            let p = count as f64 / len;
            -p * p.log2()
        }
    }).sum()
}
