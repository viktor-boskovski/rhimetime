const vowels: &'static str = "iyɨʉɯuɪʏʊeøɘɵɤoəɛœɜɞʌɔæɐaɶɑɒ";
const syllable_splitters: &'static str = "ˈˌ. ʔ";
const height_match: [[f64;3];3] = [
    [1.033, -3.509, -3.584],
    [-3.509, 0.57, -2.985],
    [-3.584, -2.985, 1.721]];
const frontness_match: [[f64;3];3] = [
    [0.806, -3.823, -3.533],
    [-3.823, 0.733, -3.442],
    [-3.533, -3.442, 1.462]];
const rounding_match: [[f64;2];2] = [
    [0.134, -3.44], // 0 is not rounded
    [-3.44, 1.936]
];
const tensing_match: [[f64;2];2] = [
    [1.477, -3.955],
    [-3.995, 0.246]
];
const stress_match: [[f64;3];3] = [
    [0.105, -4.023, -1.358], // null
    [-4.023, 0.57, -3.138], // primary
    [-1.358, -3.138, 1.198] // secondary
];

// height, frontness, rounding, tensing
pub fn classify_vowel(c: char) -> (usize, usize, usize, usize) {
    (
        if "iyɨʉɯuɪʏʊ".contains(c) {
            1 // high
        } else if "eøɘɵɤoəɛœɜɞʌɔ".contains(c) {
            2 // mid 
        } else {
            3 // low
        },
        if "iyɪʏeøɛœæaɶ".contains(c) {
            1 // front
        } else if "ɨʉɘɵəɜɞɐ".contains(c) {
            2 // mid 
        } else {
            3 // back
        },
        "yʉuʏʊøɵoœɞɔɐɶɒ".contains(c) as usize,
        "ɪʏʊɘɵəɜɞɐæɛœʌɔ".contains(c) as usize
    )
}

pub fn score_vowel(x: char, y: char, xs: usize, ys: usize) -> f64 {
    let (xh, xf, xr, xt) = classify_vowel(x);
    let (yh, yf, yr, yt) = classify_vowel(y);
    0.355 * frontness_match[xf][yf] + 0.921 * height_match[xh][yh]
        + 0.979 * rounding_match[xr][yr] + 0.053 * tensing_match[xt][yt] 
        + 0.398 * stress_match[xs][ys]
}

pub fn score(a: String, b: String) -> f64 {
    let a_stress_index = a.find('ˈ').unwrap_or(0);
    let a_stress_tail = &a[a_stress_index..];
    unimplemented!()
}


