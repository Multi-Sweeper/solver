use std::fmt::Debug;

pub fn unflatten<T: Clone + Debug>(
    flat_vec: Vec<T>,
    width: u8,
    height: u8,
) -> Result<Vec<Vec<T>>, &'static str> {
    let mut out: Vec<Vec<T>> = vec![Vec::new()];

    for elem in flat_vec {
        if let Some(row) = out.last_mut() {
            if row.len() >= width as usize {
                out.push(Vec::new())
            }

            let row = out.last_mut().unwrap();
            row.push(elem)
        }
    }

    // assert!(out.len() == height as usize, "mismatch height");

    if out.len() != height as usize {
        return Err("mismatch height");
    }

    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unflatten_1() {
        let flat = vec![
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ];

        let grid = vec![
            vec![0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0],
        ];

        assert_eq!(unflatten(flat, 5, 5).unwrap(), grid);
    }

    #[test]
    fn unflatten_2() {
        let flat = vec![
            0, 0, 4, 0, 0, 0, 1, 0, 0, 0, 3, 0, 7, 6, 0, 0, 2, 0, -2, 5, 0, 0, 5, 0, 0,
        ];

        let grid = vec![
            vec![0, 0, 4, 0, 0],
            vec![0, 1, 0, 0, 0],
            vec![3, 0, 7, 6, 0],
            vec![0, 2, 0, -2, 5],
            vec![0, 0, 5, 0, 0],
        ];

        assert_eq!(unflatten(flat, 5, 5).unwrap(), grid);
    }

    #[test]
    fn unflatten_3() {
        let flat = vec![
            0, 0, 4, 0, 0, 0, 1, 0, 0, 0, 3, 0, 7, 6, 0, 0, 2, 0, -2, 5, 0, 0, 5, 0, 0,
        ];

        assert!(unflatten(flat, 5, 6).is_err());
    }
}
