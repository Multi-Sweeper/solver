use std::fmt::Debug;

pub fn unflatten<T: Clone + Debug>(flat_vec: Vec<T>, width: u8, height: u8) -> Vec<Vec<T>> {
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

    assert!(out.len() == height as usize, "mismatch height");

    out
}
