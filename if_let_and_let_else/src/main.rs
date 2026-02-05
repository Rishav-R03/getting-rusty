fn main() {
    println!("If let and let else");

    //1. If-let
    // if let Coin::Quarter(state) = coin {
    //     println!("State quarter from {:?}!", state);
    // }else{
    //     count+=1;
    // }
    let id_card = Some(18);
    get_voter_age(id_card);
}
    //2. let else

fn get_voter_age(id_card: Option<u32>) {
    //1. Guard the function
    let Some(age) = id_card else {
        return;
    };
    if age>= 18 {
        println!("You can vote!");
    }
}
