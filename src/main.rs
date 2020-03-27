use std::cmp;

pub fn main() {
  println!("{}", jaro_wrinker_distance("abc", "acd"));
}

pub fn jaro_wrinker_distance(s1: &str, s2: &str) -> f32 {

    let mut m: f32 = 0.0;

    let s1_length = s1.len();
    let s2_length = s2.len();

    if s1_length == 0 && s2_length == 0 {
        return 0.0;
    }
    if s1 == s2 {
        return 1.0;
    }
    let range: usize = (cmp::max(s1_length, s2_length)/2) - 1;
    let mut s1_matches = vec![false; s1_length];
    let mut s2_matches = vec![false; s2_length];

    for i in 0..s1_length {

        let low = if i >= range {i - range} else { 0 };
        let high = if i + range <= s2_length { i + range } else {s2_length - i};

        for j in low..high+1 {
          if s1_matches[i] != true &&
            s2_matches[j] != true &&
            s1.chars().nth(i).unwrap()== s2.chars().nth(j).unwrap() {
            m = m + 1.0;
            s1_matches[i] = true;
            s2_matches[j] = true;
            break;
          }
        }
    }

    if m == 0.0 {
      return 0.0;
    }

    let mut k = 0;
    let mut n_trans : f32 = 0.0;

    for i in 0..s1_length {
      if s1_matches[i] == true {
        let mut j = k;
        for j1 in k ..s1_length {
          if s2_matches[j] == true {
            k = j + 1;
            j = j1;
            break
          }
        }

        if s1.chars().nth(i).unwrap() != s2.chars().nth(j).unwrap() {
          n_trans = n_trans + 1.0;
        }
      }
    }

    println!("{}", k);

    let mut weight : f32 = ( m / s1_length as f32 + m / s2_length as f32 + (m - n_trans / 2.0) / m ) / 3.0;
    let mut l = 0.0;
    let p = 0.1;

    println!("{}", weight);
    if weight > 0.7 {
      loop {
        if s1.chars().nth(l as usize).unwrap() == s2.chars().nth(l as usize).unwrap() && l < 4.0 {
          break;
        }
        l = l + 1.0;
      };

      weight = weight + l * p * ( 1.0 - weight);
    }

    return weight;
}