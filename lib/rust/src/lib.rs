pub fn process(v: &[i8], base: u8) -> Vec<u8> {
    let mut n = vec![base];
    for &x in v {
        let last = *n.last().unwrap_or(&base);
        n.push(if x == 1 { last + 4 } else if x == -1 { last.wrapping_sub(4) } else { last });
    }
    n
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn same_output() {
        let v = [1,0,-1,1,0,-1,1,1];
        assert_eq!(process(&v, 60), vec![60,64,64,60,64,64,60,64,68]);
    }
}
