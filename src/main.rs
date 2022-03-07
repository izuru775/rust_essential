use std::collections::HashMap;

fn main() {
    let mut mission_flown = HashMap::new();
    mission_flown.insert("Hadfield",3);
    mission_flown.insert("Hurley", 3);
    mission_flown.insert("Barron", 0);
    mission_flown.insert("Barron", 1);
    mission_flown.entry("Sodi").or_insert(2);
    let kyla = mission_flown.entry("Barron").or_insert(0);
    *kyla+=1;
    println!("mission_flown is {:?}", mission_flown);

    let barron_mission = mission_flown.get("Barron");
    println!("barron_mission is {:?}",barron_mission);
}
