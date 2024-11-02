fn fib_N(n : u8) -> u32 {
    let (mut a, mut b, mut k) = (0, 1, 3);

    assert!(n > 0);

    if n == 1 {
        return 0;
    } else if n == 2 {
        return 1;
    }
    
    loop {
        if k == n {
            break a + b;
        }
        let tmp = b;
        b += a;
        a = tmp;
        k += 1;
    }

    // equal to the sum of the previous two ... keep track of those and the 
}

struct User {   // define struct
    active: bool,
    email: String, // use String (vs &str) so that Struct owns all of its contents
    sign_in_count: u64,
}

fn struct_test() {
    
    let user1 = User {  // instantiate struct
        active: true,
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
    
    print!("{}", user1.active); // dot notation to access fields
    // user1.active = false;  // C_ERROR, but would work if `let mut user1 ...` was used above
    
    let user2 = User {
        active:false,
        ..user1           // struct update ... use values from user1 for all fields not explicitly defined
    };

    print!("{}", user1.active);  // fine, active has Copy trait
    print!("{}", user1.email);  // C_ERROR, ownership moved to user2
    print!("{}", user1);  // C_ERROR, the trait `std::fmt::Display` is not implemented for `User`
    println!("user1 is {user1:?}");  // C_ERROR,  :? -> print Debug format
    println!("user1 is {user1:#?}");  // C_ERROR,  #:? -> pretty print Debug format
    dbg!(&user1)    // print file, line number, and Debug format to stderr
}

fn main(){
    // println!("{} {} {}", fib_N(1), fib_N(4), fib_N(13));
    struct_test();
    // assert_eq!(fib_N(13), 144);
}


#[test]
fn fib_test(){
    assert_eq!(fib_N(4), 2);
    assert_eq!(fib_N(1), 0);
    assert_eq!(fib_N(13), 144);
}