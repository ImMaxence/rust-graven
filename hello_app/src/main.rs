fn main() {
    let mut age = 24;
    let pseudo = String::from("Max");
    age = 54;
    println!("J'ai {} ans et je me nomme {}", age, pseudo);

    let age2: u8 = 77;
    println!("{}", age2);

    let mut message = String::from("Bienvenue");
    message = String::from("Coucou");

    let message2 = &mut message;
    message2.push_str(" ici message 2");

    let message3 = &message;
    println!("{} - {}", message, message3);

    let text = if age >= 18 { "adulte" } else { "enfant" };
    println!("{}", text);

    if age < 13 {
        println!("enfant");
    } else if age >= 13 && age < 18 {
        println!("ado");
    } else if age >= 18 {
        println!("adulte");
    }

    let switch_case = match age {
        0..=12 => "enfant",
        13..=17 => "ado",
        _ => "adulte",
    };

    println!("{}", switch_case);

    let mut life: u8 = 100;

    while life > 0 && pseudo == "Max" {
        life = life - 1;
        println!("Il vous reste : {} vies", life);
    }

    for i in 1..=5 {
        println!("FOR LOOP : {}", i);
    }

    for i in (0..=100).step_by(5) {
        println!("FOR LOOP STEP: {}", i);
    }

    // pas dans tuto mais équivalent stream java
    let numbers = vec![1, 2, 3, 4, 5];

    let result: Vec<i32> = numbers
        .iter()
        .filter(|&n| n % 2 == 0)
        .map(|n| n * 2)
        .collect();
    println!("{:?}", result);
}
