use itertools::Itertools;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

pub fn mk_ops(symbols: &[char], n: usize) -> Vec<String> {
    if n == 0 {
        return vec![String::new()];
    }

    let mut result = vec![];

    for &symbol in symbols {
        for perm in mk_ops(symbols, n - 1) {
            result.push(format!("{}{}", symbol, perm));
        }
    }

    result
}

pub fn prepare(s: &str) -> Vec<String> {

    let mut result = vec![];
    let ops = mk_ops(&['+', '-', '*', '/'], 4);
    
    for digit in s.chars().permutations(s.len()) {
        for op_seq in &ops {
            let mut s = String::new();
            let mut it_op = op_seq.chars();
            for d in digit.iter() {
                s.push(*d);
                if let Some(op) = it_op.next() {
                    s.push(op);
                }
            }
            result.push(s);
        }
    }   
    result
}

#[test]
fn test_mk_ops() {
    let symbols = vec!['+', '-', '*', '/'];
    let n = 4;
    let result = mk_ops(&symbols, n);
    assert_eq!(result.len(), symbols.len().pow(n as u32));

    let res = prepare("23423");
    println!("{} {:?}", res.len(), res.iter().take(n).collect::<Vec<_>>());
}

pub fn calculate(s: &[String]) -> i32 {
    // Assumiamo che ogni stringa sia una singola espressione, es: "2+3*4-5"
    // Prendiamo la prima stringa dello slice (se presente)
    if let Some(expr) = s.get(0) {
        let mut nums = Vec::new();
        let mut ops = Vec::new();
        let mut num = String::new();

        // Separiamo numeri e operatori
        for c in expr.chars() {
            if c.is_ascii_digit() {
                num.push(c);
            } else {
                if !num.is_empty() {
                    nums.push(num.parse::<i32>().unwrap());
                    num.clear();
                }
                ops.push(c);
            }
        }
        if !num.is_empty() {
            nums.push(num.parse::<i32>().unwrap());
        }

        // Prima passata: gestiamo * e /
        let mut i = 0;
        while i < ops.len() {
            match ops[i] {
                '*' => {
                    let res = nums[i] * nums[i + 1];
                    nums[i] = res;
                    nums.remove(i + 1);
                    ops.remove(i);
                }
                '/' => {
                    let res = nums[i] / nums[i + 1];
                    nums[i] = res;
                    nums.remove(i + 1);
                    ops.remove(i);
                }
                _ => i += 1,
            }
        }

        // Seconda passata: gestiamo + e -
        let mut result = nums[0];
        for (i, op) in ops.iter().enumerate() {
            match op {
                '+' => result += nums[i + 1],
                '-' => result -= nums[i + 1],
                _ => {}
            }
        }
        result
    } else {
        0
    }
}

pub fn verify(v: &[String]) -> Vec<String> {
    use std::thread;
    let n_threads = 4;
    let result = Arc::new(Mutex::new(Vec::new()));
    let div = v.len() / n_threads;
    let mut threads = Vec::new();

    for i in 0..n_threads {
        let r = result.clone();
        let start = i * div;
        let end = if i == n_threads - 1 { v.len() } else { start + div };
        let slice = v[start..end].to_vec();

        threads.push(thread::spawn(move || {
            let mut local_result = r.lock().unwrap();
            for expr in &slice {
                if calculate(&[expr.clone()]) == 10 {
                    local_result.push(expr.clone());
                }
            }
        }));
    }

    for t in threads {
        t.join().unwrap();
    }

    let result_vec = result.lock().unwrap().clone();
    result_vec
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_add_mul() {
        let v = vec!["2+3*4".to_string()];
        assert_eq!(calculate(&v), 14); // 2 + (3*4)
    }

    #[test]
    fn test_calculate_sub_div() {
        let v = vec!["8-6/2".to_string()];
        assert_eq!(calculate(&v), 5); // 8 - (6/2)
    }

    #[test]
    fn test_calculate_all_ops() {
        let v = vec!["2+3*4-6/2".to_string()];
        assert_eq!(calculate(&v), 11); // 2 + (3*4) - (6/2)
    }

    #[test]
    fn test_calculate_equals_10() {
        let v = vec!["2*3+4".to_string()];
        assert_eq!(calculate(&v), 10); // (2*3) + 4
    }

    #[test]
    fn test_verify_finds_10() {
        let v = vec![
            "2+2+2+2+2".to_string(), // 10
            "1+2+3+4".to_string(),   // 10
            "2*3+4".to_string(),     // 10
            "5*2".to_string(),       // 10
        ];
        let result = verify(&v);
        assert_eq!(result.len(), 4);
    }
}