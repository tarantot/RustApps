/* BASIC */

// fn main() {
//     let a = 2;
//     let result = stack_only(a);
//     dbg!(result);
// }

// fn stack_only(b: i32) -> i32 {
//     let c = 3;
//     return b + c + stack_and_heap();
// }

// fn stack_and_heap() -> i32 {
//     let d = 5;
//     let e = Box::new(7);
//     return d + *e;
// }


/* ADVANCED */
fn main() {
    let a = 2;
    match stack_only(a) {
        Ok(result) => println!("Computation successful: {:?}", result),
        Err(e) => println!("Error: {}", e),
    }
}

fn stack_only(b: i32) -> Result<i32, String> {
    let c = 3;
    let sum = b.checked_add(c).ok_or("Overflow occurred in stack_only")?;
    Ok(sum + stack_and_heap()?)
}

fn stack_and_heap() -> Result<i32, String> {
    let d:i32 = 5;
    let e = Box::new(Some(7));
    let heap_value = e.ok_or("Heap value is None")?;
    let sum = d.checked_add(heap_value).ok_or("Overflow occurred in stack_and_heap")?;
    Ok(sum)
}