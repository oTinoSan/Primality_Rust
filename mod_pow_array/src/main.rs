
fn main() {
    
    let size: i32 =37;

    let mut array = Vec::new();

    array = mod_pow(array, size);

    //print array
    for row in array {
        for int in row {
            print!("{number:>4}",number=int);
        }
        println!();
    }

}

fn mod_pow(mut array: Vec<Vec<i32>>, size: i32) -> Vec<Vec<i32>>{

    let size: i32 = size;
    let mut row_num = 0;

    for _n in 1..size {
        row_num = row_num + 1;
        let mut row: Vec<i32> = Vec::new();
        let mut int = row_num;
        row.push(int);
        for _n in 1..size -1{
            int = (int * row_num) % size;
            row.push(int);
        }
        array.push(row);
    }
    return array;
}