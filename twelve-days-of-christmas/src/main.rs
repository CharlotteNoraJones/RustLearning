const GIFTS: [&str; 12] = [
    "a partridge in a pear tree",
    "two turtle doves",
    "three french hens",
    "four calling birds",
    "five golden rings",
    "six geese a-laying",
    "seven swans a-swimming",
    "eight maids a-milking",
    "nine ladies dancing",
    "ten lords a-leaping",
    "eleven pipers piping",
    "twelve drummers drumming",
];

fn main() {
    for number in 0..GIFTS.len() {
        println!("On the {} day of Christmas by true love gave to me {}", get_num_with_suffix(number + 1), get_gifts_on_day(number))
    }
}

fn get_num_with_suffix(num: usize) -> String {
    let num_string = num.to_string();
    if num >= 10 && num < 20 {
        return num_string + "th";
    } else if num % 10 == 1 {
        return num_string + "st";
    } else if num % 10 == 2 {
        return num_string + "nd";
    } else {
        return num_string + "th";
    }
}

fn get_gifts_on_day(day: usize) -> String {

    if day == 0 {
        return String::from(GIFTS[0]);
    }

    // Assumes day is 0 indexed. 
    let mut todays_gifts: String = String::new();
    for num in (1..day+1).rev() {
        todays_gifts.push_str(GIFTS[num]);
        if day == 1 {
            todays_gifts.push_str(" ")
        } else {
            todays_gifts.push_str(", ");
        }
        
    }

    todays_gifts.push_str("and ");
    todays_gifts.push_str(GIFTS[0]);

    return todays_gifts;
}