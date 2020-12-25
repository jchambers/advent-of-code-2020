mod door;

fn main() {
    let card_public_key = 8458505;
    let door_public_key = 16050997;

    let _card_loop_size = door::get_loop_size(7, card_public_key);
    let door_loop_size = door::get_loop_size(7, door_public_key);

    println!("Encryption key: {}", door::get_encryption_key(card_public_key, door_loop_size));
}
