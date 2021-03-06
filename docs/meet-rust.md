# Meet Rust

[Rust](https://www.rust-lang.org) is a general purpose programming language with an emphasis on expressiveness and safety.

Rust is a multi-paradigm programming langage that emphasys imperative programming, object oriented programming and functionnal programming styles.

Rust is a statically typed language : the compiler checks that every possible path of execution will use values only in ways consistent with their types. The compiler can usually infer what type we want to use based on the value and how we use it. Rust's type system is inflenced by [ML (Meta Language)](<https://en.wikipedia.org/wiki/ML_(programming_language)>), the first version of Rust was written in OCaml before it was rewritten in Rust in 2011.

Rust was create by Mozilla to write [servo](https://servo.org/). It have a growing fast community and a swaggy mascot : [Ferris](http://www.rustacean.net/)

```

                   _~^~^~_
               \) /  o o  \ (/
                 '_   u   _'
                 \ '-----' /
```

Rust features in "short" :

- Memory Safe
- Compiler that block lot of runtime errors
- Interface with C/C++
- Generic
- Polymorphism
- No garabage collector
- No manual memory allocation / desallocation
- No segmentation fault
- No data race
- Amazing toolchain
- Compile to native apps, libs, webassembly or even node native addons.

## First tour

Open the [try site](https://play.integer32.com/) and try each piece of code bellow, step by step

Rust code uses snake case as the conventional style for function and variable names. In snake case, all letters are lowercase and underscores separate words.

### Binding

Variable declaration and assignement use `let` binding :

```Rust
fn main() {
    let greeting: &str = "hello";
    // prefix variable with _ avoid warnings from compiler for unused variable
    let _greeting = "hello"; //Rust do inference
    let score: i16 = 10;

        println!("{} with {} kisses",greeting, score)
}
```

Bidings are immutables by default, but you can make them mutable by adding `mut` in front of the variable name.
In addition to allowing this value to change, `mut` conveys intent to future readers of the code by indicating that other parts of the code will be changing this variable value.

```Rust
fn main() {
    let mut score = 10;
    println!("{} points", score);
    score = 11;
    println!("{} points", score)
}
```

However, you may create a new binding of the same name which shadows the previous binding :

```Rust
fn main() {
    let greeting = "hello";
    println!("{}", greeting);
    let greeting = "bye";
    println!("{}", greeting)
}
```

### Ownership

When you assign a variable to another some language _copy_ the value (like C++), some other _copy_ the reference (like python). Rust use a different mechanism.
Rust’s central feature to manage memory is ownership. Although the feature is straightforward to explain, it has deep implications for the rest of the language.

First, let’s take a look at the ownership rules. Keep these rules in mind :

- Each value in Rust has a variable that’s called its owner.
- There can only be one owner at a time. So when you _use_ a variable you **move** ownership.
- When the owner goes out of scope, the value will be dropped.

Assigment move ownership of a variable :

```Rust
fn main() {
    let greeting = String::from("hello"); // in fact "hello" was a string slice
    let greeting2 = greeting; // ownership move to greeting2
    println!("{}", greeting) // error greeting does not exist any more
}
```

Pass a function's parameter move ownership of a variable :

```Rust
fn main() {
   let greeting = String::from("hello");
   println!("{}", add_world(greeting)); // ownership move to addWorld function
   println!("{}", greeting) // error greeting does not exist any more
}

fn add_world (s: String) -> String {
    s + &(String::from(" world")) // last expression is an implicit return. No semicolon !!!
}

fn useless_add_world (s: String) -> () {
    s + &(String::from(" world")); // Semicolon ends an expression. In this case the last expression is implicitily () that have unit type (similar to void in C/C++)
}
```

To reuse variable, you can clone them :

```Rust
fn main() {
    let greeting = String::from("hello"); // in fact "hello" was a string slice
    let greeting2 = greeting.clone(); // make a copy of greeting
    println!("{}", greeting); // error greeting does not exist any more
}
```

You can also **borrow** a reference to the variable :

```Rust
fn main() {
   let greeting = String::from("hello");
   let greeting2 = &greeting; // &greeting  lets us create a reference that refers to the value of greeting but does not own it
   println!("{}", greeting2);
   println!("{}", greeting); // works !
}
```

```Rust
fn main() {
   let greeting = String::from("hello");
   let greeting2 = add_world(&greeting); // &greeting  lets us create a reference that refers to the value of greeting but does not own it
   println!("{}", greeting2);
   println!("{}", greeting); // works !
}

fn add_world (s: &String) -> String {
    let s_world = s.clone() + &(String::from(" world")); // +infix operator (+) concat String with &str (string slice)
    s_world
}
```

This introduce some restrictions like a borrowed content cannot be moved :

```Rust
fn main() {
   let greeting = String::from("hello");
   let greeting2 = add_all(&greeting);
}

fn add_all (s: &String) -> String {
    let s_world = *s + &(String::from(" all")); // compilation error
    s_world
}
```

You can notice that `&` give a reference to a value, the opposite is dereference operator is accomplished by `*`

There is an exception to the move of ownership mechanism, the type of the variable implement the _Copy_ trait. This is the case by default for primitive like _bool_, integer types (_i16_, _u32_, ...), floating types (_f64_, ...), _char_, ... and tuples which only contain types with Copy trait : _(i32, u16)_ does Copy, but _(i32, String)_ doesn't Copy

```Rust
fn main() {
    let score = 10;
    let score2 = score; // score2 copy score
    println!("{}", score); // print 10
}
```

By having this model, Rust prevents several errors, like seg faults. It is similar to Read-Writers lock :

- Many readers at once **OR** a single writer with exclusive access
- Read only do not require exclusive access
- Exclusive access do not allow other readers

If it is a new concept for you, you should [read this](https://www.oreilly.com/library/view/programming-rust/9781491927274/ch04.html)

### Types

The type system is completely "sound" by default. This means that, as long as your code compiles fine in safe mode, every type guarantees that it's not lying about itself. In a conventional, best-effort type system, just because the type says it's e.g. "an integer that's never null", doesn't mean it's actually never null. In contrast, a pure Rust program has no null bugs.

This code gives a copilation time error :

```Rust
fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String { // dangle returns a reference to a String

    let s = String::from("hello"); // s is a new String

    &s // we return a reference to the String, s
} // Here, s goes out of scope, and is dropped. Its memory goes away.
```

Rust types can be inferred. The type system deduces the types for you even if you don't manually write them down.

```Rust
let greeting = "hello";
let score = 10;
```

Alias types can refer to a type by a different name. They'll be equivalent:

```Rust
type Score = i16;
let s1 : Score = 10;
```

Rust provides two list primitives : Array and Vector. Arrays are useful when you want your data allocated on the stack rather than the heap. Arrays are homogeneous, immutable, fixed length.

```Rust
    let heroes_array : [&str; 3] = ["Carl", "Rick", "Michonne"];
    let first = heroes_array[0]; // accessing first element
    let second = heroes_array[1];
```

A vector is a similar collection type provided by the standard library that is allowed to grow or shrink in size.

```Rust
    let heroes_vector : Vec<String> = vec![String::from("Carl"),String::from( "Rick"), String::from("Michonne")];
    let first = &heroes_vector[0];
    assert_eq!(first, "Carl");
    let middle = &heroes_vector[1..3];
    assert_eq!(middle, [ "Rick", "Michonne"]);
    let copy_heroes = heroes_vector.clone(); // clone heroes_vector which cannot be move because previously borrowed
    let mut more_heroes = vec![String::from("Negan")];
    more_heroes.extend(copy_heroes);// copy_heroes is moved and can't be used anymore
    assert_eq!(more_heroes, [ "Negan", "Carl", "Rick", "Michonne"]);
```

####Product types

```Rust
/* Tuples are immutable, ordered, fix-sized at creation time heterogeneous */
let name_heart: (&str, i32) = ("Negan", 10);
type coord3d = (i32, i32, i32);
let warehouseCoord: coord3d = (1, 4, 18);

/* Records are immutable by default and fixed in field names and types */
struct Hero {
  heart: i32,
  name: String,

}
let negan = Hero { heart : 10, name : String::from("Negan")};
let name = String::from("Carl");
let carl = Hero {
    name, // punning
    heart: 10,
};
```

####Variant types

```Rust
enum LifeVariant {
    Alive(Hero),
    Dead,
    Zombi
}

let how_is_Negan = LifeVariant::Alive(negan);
let how_is_Lori = LifeVariant::Dead;
```

Alive, Dead and Zombi are called "constructors" (or "tag"). A variant's constructors need to be capitalized. Type constructor may have parameters

Algebraic data types come with one of the most important features : **pattern matching**

```Rust
fn get_message(how_are_you: LifeVariant) -> String{
     match how_are_you {
      LifeVariant::Zombi => String::from("Aaaaaarg !"),
      LifeVariant::Dead => String::from("!!!"),
      LifeVariant::Alive(h) => String::from("Great! ") + &h.name + " is alive",
    }

};
assert_eq!(get_message(how_is_Negan), String::from("Great! Negan is alive"));
assert_eq!(get_message(how_is_Lori), String::from("!!!"));
```

For some use cases, when matching enums, match is awkward. `if let` is cleaner for some use case and in addition allows various failure options to be specified:

```Rust
let how_is_zombi = LifeVariant::Zombi;

fn get_message2(how_are_you: LifeVariant) -> String{
    // The `if let` construct reads: "if `let` destructures `how_are_you` into
    // `LifeVariant::Alive(h)`, evaluate the block (`{}`).
    if let LifeVariant::Alive(h) = how_are_you  {
         String::from("Great! ") + &h.name + " is alive"
    } else{
        String::from("No more hope")
    }
};
assert_eq!(get_message2(how_is_zombi), String::from("No more hope"));
```

### Options

Rust itself doesn't permit the notion of null or undefined in safe code. This is a great thing, as it wipes out an entire category of bugs. No more undefined is not a function, and cannot access foo of undefined!

We represent the existence and nonexistence of a value by wrapping it with the option type. Here's its definition from the standard library:

```Rust
enum Option<T> {
   None,
   Some(T)
}
```

It means "a value of type option is either None (nothing) or that actual value wrapped in a Some".
It's easy to unwrap values from option type using pattern matching.

```Rust
type Weapon =  Option<String>;

fn is_dangerous(w: Weapon) -> String {
  match w {
   None  => String::from("not dangerous"),
   Some(a) => String::from("is armed with a ") + &a,

  }
};

let unarmed: Weapon = None;
let knife: Weapon = Some(String::from("knife"));
assert_eq!(is_dangerous(unarmed), String::from("not dangerous"));
assert_eq!(is_dangerous(knife), String::from("is armed with a knife"));
```

There is a shorter way to [unwrap](https://doc.rust-lang.org/std/option/enum.Option.html#method.unwrap_or) a value from an Option<T> in Rust :

```Rust
type Weapon =  Option<String>;

let unarmed: Weapon = None;
let knife: Weapon = Some(String::from("knife"));

assert_eq!(knife.unwrap(), String::from("not dangerous"));//unwrap panic if None
assert_eq!(knife.unwrap_or("not dangerous"), String::from("knife"));/unwrap_or provide a default value
assert_eq!(unarmed.unwrap_or("not dangerous"), String::from("not dangerous"));/

```

### Control flow

In Rust `if` is an expression, we can use it on the right side of a let binding or a function's return

```Rust
fn main() {
    println!("{}", get_person_label(10));
}

fn get_person_label(age: i32) -> String {
    if age < 10 {
        String::from("child")
    }else if age < 18 {
        String::from("teenage")
    }else{
       String::from("adult")
    }
}

```

### Functions

Functions are declared with start with `fn` and have a set of parentheses after the function name.
The curly brackets tell the compiler where the function body begins and ends.
Rust also have lambdas.

```Rust
 fn add (x:i32, y:i32) -> i32 {
      x+y
 }
 let add_lambda = |x: i32, y: i32| -> i32 {x + y}; // this is lambda syntax
 let add1 = |x| add(1, x); // You may use lambda to do partial application
 assert_eq!(add_lambda(1,2),add(1,2));
 assert_eq!(add1(2),add(1,2));
```

Rust's function are first class order : functions can be pass as parameters or return from another function.

```Rust
 fn add (x:i32, y:i32) -> i32 {
      x+y
 }

// High Order Function
 fn calculate<A>( operation: A, x:i32, y:i32)  -> i32
                    where A: Fn(i32, i32) -> i32 {
    operation(x, y)
}

 assert_eq!(add(1, 2),calculate(add,1,2)); // with named function as parameter
 assert_eq!(add(1, 2),calculate(|x: i32, y: i32| -> i32 {x + y},1,2)); // with lambda as parameter
```

The above code shows a higher order function. The interesting piece in code is the type of firts parameter `operation`. `operation` is of generic type `A` which is defined in the `where` clause.<br/>
`where` clause in rust is used for type bound on [generics](https://doc.rust-lang.org/book/ch10-01-syntax.html). The type bound in our example says, A is of type Fn, one of the trait for function type, which takes two i32 values and returns i32.

As we can pass a function as parameter to a function, we can return a function from another function. This is quite simple in most of the functional programming languages, but it’s its little bit tricky in rust.

```Rust
 fn add (x:i32, y:i32) -> i32 {
      x+y
 }

// High Order Function
fn gen_add<'a>(x:& 'a i32) ->
                            Box<Fn(i32) -> i32 + 'a > {
       Box::new(move |y:i32| y+x)
}
let add1 = gen_add(&1);

assert_eq!(add(1, 2), add1(2));
```

The above code looks complicated. The reason to have the complications is the way rust works with lifetimes. Let’s try to understand what’s going on here.

In our code, we are defining a function which take i32 value as parameter. Then the function should return a function which wraps this value with logic to increment given value with the parameter. The question we ask ourselves is how long this function lives?

In garbage collected language like Javascript, it’s easy as garbage collector take care of this issue. But rust doesn’t have a gc. So rust has to determine the lifetime of the function in the compile time only.

In our example, we are defining a scope `‘a` which is associate a scope with input value. So we are saying here to compiler, keep lifetime of function as long as value `x` exist. Lifetimes in rust can only exist with references. So in our example we will take `&i32` rather than `i32`. Also we create to reference to `Fn` using [Box]().

Now we understand the lifetime of formal parameters and return type. But what about `move` in our implementation? The value we take as parameter is created in stack. So when function returns the `x` is destroyed. So `move` says move the ownership of `x` as part of closure.
