async fn say_world() {
    println!(" world!");
}

fn main() {
   let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        // insert typical async main contents here:
        // (typically mut is required for rt, but this is
        //  too simple of an example.
            let op = say_world();

            print!("hello");
            op.await;
        // end of insert
    })
}

/*
// the follwoing is the way to do it using the macro:
#[tokio::main]
async fn main() {
    // Calling `say_world()` does not execute the body of `say_world()`.
    let op = say_world();

    // This println! comes first
    print!("hello");

    // Calling `.await` on `op` starts executing `say_world`.
    op.await;
}
*/
