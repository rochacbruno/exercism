pub fn raindrops(number: i32) -> String {
    let drops = [
        (3, "Pling"),
        (5, "Plang"),
        (7, "Plong"),
    ];

    let ret = drops.iter()
        .filter(|(factor, _)| number % factor == 0)
        .map(|(_, sound)| (*sound).to_string())
        .collect::<Vec<String>>();

    if !ret.is_empty() {
        ret.join("")
    } else {
        number.to_string()
    }

}
