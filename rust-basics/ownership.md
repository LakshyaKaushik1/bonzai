# Ownership and Borrowing

In Rust, there are two kinds of values:
- Copy: the ones that get duplicated once the value is passed to someone else. Think of it as you want to send your friend the group photo that you took on the trip. So you will send a copy of that photo while keeping the one with you. You don't send and delete the photo on your device. You'll have the photo as well as your friend.

- Here the data is stored on stack.

Datatypes that are copy type:
- All primitive integers: i32, u64, etc.
- Floating point numbers : f32, f64
- Booleans
- Character

Non-copy datatypes
- There are datatypes that completely transfer the ownership to the next variable. They have their memory on heap. And once the data is transferred, it drops the data from the existing variable. Think of it as your great grandfater. When his friend asked him for a photo of his wedding, he gave that hard copy to him. Now the owner of that photo is his friend and not your grandfater. If anyone will ask him for the photo, then he won't be able to give it.

- Non copy datatypes are mostly large chunks. Duplicating them will be wasteful and slow.

Datatypes that are non-copy:
- String
- Vectors
- Smart Pointers
- Muatble References

## Borrowing
- This is the process of letting a function/variable use the value for sometime without actually giving up the complete ownership of that value.

- Real life example: You lend a book to your friend. He can read it. But he don't actually own it. They will give it back to you so that you can keep it.

- You can borrow the data by using an & (ampersand) operator before a variable without taking ownership of it.

- Ex: 

fn main(){

    let a = String::from("Hello");

    // Borrow it
    let b = &a;

    println!("{b} is same as {a});
}

- You can borrow immutably by just using & <variable_name>
- You can borrow mutably by using &mut <variable_name>
- You can borrow almost all the datatypes in rust

## 3 Rules of Ownership
1. Each value will have a single owner.
2. Only one owner can exist at a time for the respective value. You either completely pass it or lend it to another variable.
3. Once the ownership is transferred, the data of existing variable is deleted and it no longer stores that value.

Note: We use borrowing so that the original owner doesn't lose its ownership.

## 2 Rules of Borrowing
1. You can have ANY number of immutable (read-only) borrows (using &)
2. You can only have ONE mutable (read-write) borrow at a time.

For ex:

First rule (Safe because no one is modifying the data)

fn main() {

    let book = String::from("Rust Basics");

    let reader1 = &book;
    let reader2 = &book;
    // You can have n numbers of read-only references for a piece of data at the same time.

    println!("{reader1} and {reader2}");
}

Second Rule (as you're modifying the data, no one else can read and write simultaneously)

fn main() {

     let writer1 = &mut score; // First read-write borrow
    let writer2 = &mut score; // ERROR: Cannot borrow `score` as mutable more than once

    writer1 = 20;
    writer2 = 30;

}


Note: you cannot mix mutable and immutable variables together.

Ex:

fn main() {
    let mut score = 10;

    let reader = &score;      // Read-only borrow
    let writer = &mut score;  // ERROR: Cannot borrow `score` as mutable because it is already borrowed as immutable

    println!("{}", reader); 
    writer = 20;
}
