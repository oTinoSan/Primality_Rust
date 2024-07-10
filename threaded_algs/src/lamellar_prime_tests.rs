use lamellar::array::prelude::*;

fn main(){
    let world = lamellar::LamellarWorldBuilder::new().build();
    let my_pe = world.my_pe();
    let block_array = AtomicArray::<usize>::new(&world, 1000, Distribution::Block); //we also support Cyclic distribution.
    block_array.dist_iter_mut().enumerate().for_each(move |(i,elem)| elem.store(i) ); //simultaneosuly initialize array accross all pes, each pe only updates its local data
    block_array.wait_all();
    block_array.barrier();
    if my_pe == 0{
        for (i,elem) in block_array.onesided_iter().into_iter().enumerate(){ //iterate through entire array on pe 0 (automatically transfering remote data)
            println!("i: {} = {})",i,elem);
        }
    }
}