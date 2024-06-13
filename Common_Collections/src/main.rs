// use std::collections::BinaryHeap;
// use std::collections::HashMap;
use std::collections::HashSet;

fn main() {

    /*
     *  Vector
     */
    // let mut nums: Vec<i32> = vec![];
    // nums.push(1);
    // nums.push(2);
    // nums.push(3);
    // nums.push(4);
    // println!("{:?}", nums);

    // let pop = nums.pop(); //Option<T>, return Some(T) or None 
    // println!("{:?}", nums);
    // println!("{:?}", pop);

    // let two = nums[1]; // It is copy.
    // // &nums[1], create a referance if copy is not availible
    // println!("{}", two);
    // println!("{}", nums[1]);

    // let one = nums.first(); // return an Option<T>, so None if vec is empty, or Some<T> on index 0;
    // println!("{:?}", one);

    // Other methods in Vector:
    // .last, 
    // .first_mut and .last_mut, so will borrow mutable referances

    // println!("{}", nums.len()); //return size of vector
    // println!("{}", nums.is_empty()); // return bool

    // nums.insert(0, 100);
    // println!("{:?}", nums);
    // nums.insert(3, 1000);
    // println!("{:?}", nums);
    // nums.insert(2, 10000);
    // println!("{:?}", nums);

    // nums.remove(3);
    // println!("{:?}", nums);
    // nums.sort();
    // println!("{:?}", nums);
    // nums.reverse();
    // println!("{:?}", nums);

    /*
     *  Binary Heap
     */

    // let mut bheap = BinaryHeap::new();

    // bheap.push(1);
    // bheap.push(18);
    // bheap.push(20);
    // bheap.push(5);
    // println!("{:?}", bheap);

    // bheap.pop();
    // println!("{:?}", bheap);
    // println!("{:?}", bheap.peek()); // peek is going to return Option<T>, return None if heap is empty, or Some<T> otherwise.
    // println!("{:?}", bheap); // peek will not remove fist item in heap.

    /*
     *  Map
    */

    // let mut hm = HashMap::new();

    // hm.insert(1, 1);
    // hm.insert(5, 2);
    // hm.insert(30, 3);
    // hm.insert(50, 4);
    // println!("{:?}", hm);

    // let old = hm.insert(30, 7); // key is going to update the old value 3 to the new value 7, also, going yo return old value.
    // println!("{:?}", hm);
    // println!("{:?}", old);

    // println!("{}", hm.contains_key(&50));
    // println!("{}", hm.contains_key(&100));

    // println!("{:?}", hm.get(&50)); //return Option<T>
    // println!("{:?}", hm.get(&100));

    // let mut rm = hm.remove(&50); // it will return the value.
    // println!("{:?}", hm);
    // println!("{:?}", rm);

    // let mut rm_e = hm.remove_entry(&30); // it will return key and value.
    // println!("{:?}", hm);
    // println!("{:?}", rm_e);

    // hm.clear();
    // println!("{}", hm.is_empty());

    /*
     *  Set
     */

    let mut hs = HashSet::new();

    hs.insert(1);
    hs.insert(2);
    hs.insert(3);
    hs.insert(4);
    hs.insert(100);
    println!("{:?}", hs);
    println!("{}", hs.len());
    println!("{}", hs.is_empty());
    for x in hs.iter(){
        println!("Inter: {}", x);
    }

    // hs.remove(&2);
    // println!("{:?}", hs);
    // println!("{}", hs.len());
    // println!("{}", hs.is_empty());
    // for x in hs.iter(){
    //     println!("Inter: {}", x);
    // }

    let mut hs2 = HashSet::new();
    hs2.insert(1);
    hs2.insert(3);
    hs2.insert(5);
    hs2.insert(7);
    hs2.insert(9);

    for x in hs.intersection(&hs2){
        println!("Intersection: {}", x);
    }

    let intersecotin = &hs & &hs2; // shorthand intersection using the binary bitwise & operater
    println!("{:?}", intersecotin);
    for x in intersecotin{
        println!("In short hand way: {}", x);
    }

    println!("HashSet1: {:?}", hs);
    println!("HashSet2: {:?}", hs2);
    let union_between_hs_hs2 = &hs | & hs2; // HashSet Union method
    for x in union_between_hs_hs2{
        println!("Union: {}", x);
    }

    let diff = &hs - &hs2; // HashSet different method
    for x in &diff{
        println!("Different: {}", x);
    }
    


}
