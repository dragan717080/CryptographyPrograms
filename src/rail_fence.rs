fn main() {
    let message = "AMICUSVERUSESTRARAVIS";

    let encrypted_message = rail_fence_encrypt(message, 2);
    println!("Encrypted message: {}", encrypted_message);
}

fn rail_fence_encrypt(message: &str, rails: usize) -> String {
    let mut fence: Vec<Vec<char>> = vec![vec![' '; message.len()]; rails];
    let mut rail = 0;
    let mut direction = 1; // 1: down, -1: up

    for c in message.chars() {
        let position = fence[rail].iter().position(|&x| x == ' ').unwrap();
        fence[rail][position] = c;
        
        if rail == rails - 1 || rail == 0 {
            direction = -direction;
        }
        rail = ((rail as isize + direction + rails as isize) % rails as isize) as usize;
    }

    let encrypted_message: String = fence
        .into_iter()
        .flat_map(|row| row.into_iter())
        .filter(|&c| c != ' ')
        .collect();

    encrypted_message
}
