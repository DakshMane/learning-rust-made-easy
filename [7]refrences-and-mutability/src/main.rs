fn main() {

    /*
    . A reference is like a pointer in that it’s an address we can follow to access the data stored at that address; that data is owned by some other variable. Unlike a pointer, a reference is guaranteed to point to a valid value 
of a particular type for the life of that reference. */


/*
The Rules of References

Let’s recap what we’ve discussed about references:

    At any given time, you can have either one mutable reference or any number of immutable references.
    References must always be valid.
 */
    let s1 =  String::from("Hello") ; 

    let len = calculate_len(&s1) ; 

    println!("The length of '{}' is {}.", s1, len);

   
}

fn calculate_len(s : &String) -> usize {
    s.len() 

    /*The scope in which the variable s is valid is the same as any function parameter’s scope,
     but the value pointed to by the reference is not dropped when s stops being used, because s doesn’t have ownership. When functions have references as parameters instead of the actual values,
      we won’t need to return the values in order to give back ownership,
     because we never had ownership. */
}
