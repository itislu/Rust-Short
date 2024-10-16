use std::cmp::Ordering;

fn main() {
    let target = ftkit::random_number(0..=1000);

    println!("Me and my infinite wisdom have found an appropriate secret you shall yearn for.");
    loop {
        let guess = ftkit::read_number();
        match guess.cmp(&target) {
            Ordering::Less => println!("This student might not be as smart as I was told. This answer is obviously too weak."),
            Ordering::Equal => {
                println!("That is right! The secret was indeed the number {}, which you have brilliantly discovered!", target);
                break;
            }
            Ordering::Greater => println!("Sometimes I wonder whether I should retire. I would have guessed higher."),
        }
    }
}
