pub fn max_difference_2(s: String, k: i32) -> i32 {
    let l = s.len();
    let mut sum = i32::MIN;
    let chs: Vec<char> = s.chars().collect();

    fn bit_calc(i: i32, j: i32) -> i32 {
        /*
        this will return a 0, 1, 2, 3 depending on if:
        if i is odd it will equal 10
        if i is even it will equal 00
        if j is odd it will end in 1
        if j is even it will end in 0

        bitOR will merge any 1 onto a 0 and it will return the bits
        00 = both even = 0
        01 = even odd = 1
        10 = odd even = 2
        11 = odd odd = 3

        the << shifts the first bit digit to the left by 1
        */
        ((i & 1) << 1) | (j & 1)
    }

    for i in '0'..='4' {
        for j in '0'..='4' {
            if i == j {
                continue;
            }
            let mut best = [i32::MAX; 4];
            let mut count_i = 0;
            let mut count_j = 0;
            let mut last_i = 0;
            let mut last_j = 0;
            let mut left: i32 = -1;
            for right in 0..l {
                count_i += if chs[right] == i { 1 } else { 0 };
                count_j += if chs[right] == j { 1 } else { 0 };
                while (right as i32 - left) >= k && (count_j - last_j) >= 2 {
                    let left_bit = bit_calc(last_i, last_j) as usize;
                    best[left_bit] = best[left_bit].min(last_i - last_j);
                    left += 1;
                    last_i += if chs[left as usize] == i { 1 } else { 0 };
                    last_j += if chs[left as usize] == j { 1 } else { 0 };
                }

                let right_bit = bit_calc(count_i, count_j) as usize;
                if best[right_bit ^ 2] != i32::MAX {
                    sum = sum.max(count_i - count_j - best[right_bit ^ 2])
                }
            }
        }
    }
    sum
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn ex1() {
        assert_eq!(max_difference_2("12233".to_string(), 4), -1)
    }

    #[test]
    fn ex2() {
        assert_eq!(max_difference_2("1122211".to_string(), 3), 1)
    }

    #[test]
    fn ex3() {
        assert_eq!(max_difference_2("110".to_string(), 3), -1)
    }

    #[test]
    fn ex4() {
        assert_eq!(max_difference_2("2421".to_string(), 1), -1)
    }

    #[test]
    fn ex5() {
        assert_eq!(max_difference_2("11424400131134301044243113022221300021211202403111430221041004231001341104433213402300432130032340023321404112101131040332302300411112143001002320141132234214142034133010312100021002140010343104212203001141201241430004121034222340002410114141434230244012120030430332300232433303333023142433411014341400021103343131321433023411332142104114334214411012244434011011111444240002124202120103434401211423113120242142424302214112311241201241334323241202422122012100114220421124240121200243441324203440414004431024211232140324301002040143120141403421222430210313032440114143310340442334222144214342301032011113210300110401311402332221311212020243320343244124222240423130413021303214044344023001201430322410411340242242132403310222113133431414313021410414133120010404014302220332142420423232040330204012414431412030440223013434240312343140022212022201224220342313001134413022111241042041112102304222331102200221411224144223011333341412112130313211302141120431".to_string(), 508), 55)
    }

    #[test]
    fn ex6() {
        assert_eq!(max_difference_2("01430122223001242230202331322442423400103130144134012222112300412401410433100313334134402231102111334444430123311044043041432020324030431433042003221133110234311041223142232000043314040400120131304202402432010012342023021332313314302111121012213410040432312440421300104104104444224410444303324013423141334042022441141110322120244041430212344311213234341222030103332024141034113020003423314001423320003434014311340342023033131411302304100331211310302232421202241301201033434203343322144211104301111113233010301424311144024313210203200411241001331031404303204014202111400233003102231243044202301413014041020134004304114403104433000441202102034112232412231023444422201222341141330214430134013332431444034340024322403314011224321420013042421243321143342222304302023134003102224021132433020224104140243024124230003202001244242013234314220020213220213422203023214341141034420313420333002301443340003121".to_string(), 405), 33)
    }
}

/*
initial try.. which worked but slow

use std::collections::{HashMap, HashSet};

pub fn do_calc(ch: &[u8]) -> i32 {
    let mut dict = HashMap::new();
    let mut odds = i32::MIN;
    let mut evens = i32::MAX;

    for c in ch {
        *dict.entry(*c as char).or_default() += 1;
    }
    for i in dict.values() {
        if i % 2 == 0 && *i < evens {
            evens = *i;
        } else if i % 2 != 0 && *i > odds {
            odds = *i;
        }
    }
    if odds == i32::MIN || evens == i32::MAX {
        return i32::MIN;
    }
    odds - evens
}

pub fn max_difference_2(s: String, k: i32) -> i32 {
    let length = s.len() as i32;
    let mut max = k;
    let mut sum = i32::MIN;
    let mut windows = HashSet::new();

    while max <= length {
        for ch in s.as_bytes().windows(max as usize) {
            windows.insert(ch);
        }
        max += 1;
    }
    for window in windows {
        let temp_sum = do_calc(window);
        if temp_sum > sum {
            sum = temp_sum;
        }
    }
    if sum == i32::MIN {
        sum = 0;
    }
    sum
}
*/
