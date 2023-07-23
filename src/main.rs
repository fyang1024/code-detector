/**
 * You are a detective trying to crack a complex code left by a suspect. The code consists of 8 
 * characters, which can be any combination of uppercase letters, lowercase letters, digits (0-9), 
 * and special symbols (there are 33 special symbols). However, you do not know the order of 
 *the characters. You have collected some information that may help you crack the code:
 * • The code does not contain any repeated characters.
 * • The sum of the digits in the code is 20.
 * • The code contains at least one uppercase letter, one lowercase letter, one digit, and one 
 *   special symbol. • The second and fourth characters are vowels (either uppercase or lowercase).
 * • The third, sixth and seventh characters are digits.
 * • The fifth character is a special symbol. • The last character is a lowercase letter.
 * Your task is to determine the number of possible codes that meet these conditions.
 * 
 * All the 95 chars are ascii codes between 32 and 126 inclusive
 * The following program uses 2 different methods to calculate the number of possible codes
 * The results are both 145862640
 * The first method is significantly faster because it cut off the impossible codes as early as possible
 */
pub fn main() {
    let vowels = [65, 69, 73, 79, 85, 97, 101, 105, 111, 117];
    let special_char_ranges = [(32..=47), (58..=64), (91..=96), (123..=126)];
    let mut special_chars = Vec::<i32>::new();
    for range in special_char_ranges {
        for i in range {
            special_chars.push(i);
        }
    }

    let mut count = 0u64;

    for c2 in vowels {
        for c4 in vowels {
            if c4 == c2 { // chars must be different
                continue;
            }

            for c3 in 48..=57 {
                for c6 in 48..=57 {
                    if c6 == c3 { // digits must be different and the 3 digits
                        continue;
                    }
                    for c7 in 48..= 57 {
                        if c3 == c7 || c6 == c7 || c3 + c6 + c7 > 164 { // 3 digits must be different and add up to max 20
                            continue;
                        }
                        for c8 in 97..=122 { // c8 is lower case
                            if c2 == c8 || c4 == c8 { // chars must be different
                                continue;
                            }
                            for c5 in &special_chars {
                                for c1 in 32..=126 {
                                    if c1 >= 48 && c1 <= 57 { // digit
                                        if c1 + c3 + c6 + c7 == 212 && (c2 < 90 || c4 < 90) &&  c1 != c3 && c1 != c6 && c1 != c7 {
                                            count += 1;
                                        }
                                    } else if c1 >= 65 && c1 <= 90 { // upper case
                                        if c1 != c2 && c1 != c4 && c3 + c6 + c7 == 164 {
                                            count += 1;
                                        }
                                    } else if c1 >= 97 && c1 <= 122 { // lower case
                                        if c1 != c2 && c1 != c4 && c1 != c8 && (c2 < 90 || c4 < 90) && c3 + c6 + c7 == 164 {
                                            count += 1;
                                        }
                                    } else { // special char
                                        if c1 != *c5 && (c2 < 90 || c4 < 90) && c3 + c6 + c7 == 164 {
                                            count += 1;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    println!("{}", count);

    count = 0;

    for c1 in 32..=126 {
        for c2 in vowels {
            for c3 in 48..=57 {
                for c4 in vowels {
                    for c5 in &special_chars {
                        for c6 in 48..=57 {
                            for c7 in 48..=57 {
                                for c8 in 97..=122 {
                                    if special_chars.contains(&c1) {
                                        // c1 is special character
                                        if c3 + c6 + c7 == 164 // 3 digits add up to 20
                                            && (c2 < 90 || c4 < 90) // at least one upper case
                                            && c1 != *c5 // 2 special chars different
                                            && c3 != c6 // 3 digits different
                                            && c3 != c7 
                                            && c6 != c7
                                            && c2 != c4 // 3 letters different
                                            && c2 != c8 
                                            && c4 != c8
                                        {
                                            count += 1;
                                        }
                                    } else if c1 >= 48 && c1 <= 57 {
                                        // c1 is digit
                                        if c1 + c3 + c6 + c7 == 212 // 4 digits add up to 20
                                            && (c2 < 90 || c4 < 90) // at least one upper case
                                            && c1 != c3 // 4 digits different
                                            && c1 != c6
                                            && c1 != c7
                                            && c3 != c6
                                            && c3 != c7
                                            && c6 != c7
                                            && c2 != c4 // 3 letters different
                                            && c2 != c8
                                            && c4 != c8
                                        {
                                            count += 1;
                                        }
                                    } else if c1 >= 65 && c1 <= 90 { // c1 is upper case
                                        // c1 is upper case
                                        if c3 + c6 + c7 == 164 // 3 digits add up to 20
                                            && c1 != c2 // 4 letters different
                                            && c1 != c4
                                            && c1 != c8
                                            && c2 != c4
                                            && c2 != c8
                                            && c4 != c8
                                            && c3 != c6 // 3 digits different
                                            && c3 != c7
                                            && c6 != c7
                                        {
                                            count += 1;
                                        }
                                    } else {
                                        // c1 is lower case
                                        if c3 + c6 + c7 == 164 // 3 digits add up to 20
                                            && (c2 < 90 || c4 < 90) // at least one upper case
                                            && c1 != c2 // 4 chars different
                                            && c1 != c4
                                            && c1 != c8
                                            && c2 != c4
                                            && c2 != c8
                                            && c4 != c8
                                            && c3 != c6 // 3 digits different
                                            && c3 != c7
                                            && c6 != c7
                                        {
                                            count += 1;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    println!("{}", count);
}
