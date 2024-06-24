use primality::*;

fn main() {

    // let result = trial_vect(1000);
    // println!("{:?}", result);
    // let result = trial(1000);
    // println!("{:?}", result);

    // let result = trials::trial_vect_2(1000);
    // println!("{:?}", result);
    // let result = trials::trial_2(1000);
    // println!("{:?}", result);

    // let result = sieve::sieve(1000);
    // println!("{:?}", result);
    // let result = seive::sieve_vect(1000);
    // println!("{:?}", result);

    // let result = wheel::wheel_facts(1000);
    // println!("{:?}", result);
    // let result = wheel::wheel(1000);
    // println!("{:?}", result);

    // let result = miller_rabine::miller_rabin(1000);
    // println!("{:?}", result);

    // let result = solovay_strassen::solovay_strassen(479001599);
    // println!("{:?}", result);

    let rows = vec![
        vec![0, 1, 1, 1, 0],
        vec![0, 0, 1, 1, 1],
        vec![1, 0, 0, 1, 0],
        vec![0, 0, 1, 0, 1],
        vec![1, 0, 0, 0, 0],
    ];

    let adj_matrix = array2d::Array2D::from_rows(&rows).unwrap();
    let csr = compressed_sparse_rows::Csr::from_adjacency(&adj_matrix);

    // Use the CSR structure, for example, print its contents
    println!("Row Offset: {:?}", csr.row_offset);
    println!("Column Indices: {:?}", csr.col_indices);

}