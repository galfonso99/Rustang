/* Timed Reading is an educational tool used in many schools to improve and advance reading skills. A young elementary student has just finished his very first timed reading exercise. Unfortunately he's not a very good reader yet, so whenever he encountered a word longer than maxLength, he simply skipped it and read on.

Help the teacher figure out how many words the boy has read by calculating the number of words in the text he has read, no longer than maxLength.
Formally, a word is a substring consisting of English letters, such that characters to the left of the leftmost letter and to the right of the rightmost letter are not letters. */

fn timedReading(max: i32, text: String) -> i32 {
    text.split(|c:char| !c.is_ascii_alphanumeric()).filter(|s| s.len() >0 && s.len() <= max as usize).count() as i32
}
