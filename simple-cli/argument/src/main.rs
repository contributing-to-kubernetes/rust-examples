//use std::collections::HashMap;
use std::env;
use std::process;

const SHORT_ARG_PREFIX: &str = "-";
const ARG_PREFIX: &str = "--";

/*
 * String literals are strings that are part of the program. String literals
 * are immutable because they are slices pointing to a specific point of our
 * program.
 * `str` is a string slice. You will most likely see them in their "borrowed"
 * form (`&str`). String slices are immutable references.
 * `String`s, on the other hand, are growable, mutable, owned, utf-8
 * strings (they can be found in the heap).
 *
 * ref: https://doc.rust-lang.org/book/ch08-02-strings.html and
 * https://doc.rust-lang.org/book/ch04-03-slices.html and
 * https://doc.rust-lang.org/1.7.0/book/strings.html
 *
 * If we had to chose between &String and &str, chose &str. Passing a string
 * slice allows for both string slices and string literals (we can always pass
 * a slice of the entire String).
 */

fn split_args(args: &[String]) -> (&[String], &[String]) {
    // In case you wanted to see how to search for something quickly.
    // https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.position
    if let Some(index) = args.iter().position(|arg| arg == "--") {
        return (&args[..index], &args[index + 1..]);
    }

    (&args, &[])
}

#[derive(Debug)]
enum Value {
    Bool(String),
    String(String),
}

// If we try to specify an Argument as one would in any other language, the
// compiler will advice us to "consider introducing a named lifetime parameter".
//
// https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html
#[derive(Debug)]
struct Argument<'a> {
    name: &'a str,
    required: bool,
    help: Option<&'a str>,
    takes_value: bool,
    default_value: Option<Value>,
    user_value: Option<Value>,
}

impl<'a> Argument<'a> {
    fn new(name: &'a str) -> Argument {
        Argument {
            name,
            required: false,
            help: None,
            takes_value: false,
            default_value: None,
            user_value: None,
        }
    }
}

#[derive(Debug)]
enum CMDError {
    UnexpectedArgument(String),
}

// When validating command line arguments, we want to get a descriptive error
// if something is wrong, but we don't really care about the Ok value.
fn validate_arg(arg: &str) -> Result<(), CMDError> {
    if !arg.starts_with(ARG_PREFIX) && !arg.starts_with(SHORT_ARG_PREFIX) {
        return Err(CMDError::UnexpectedArgument(arg.to_string()));
    }

    let arg_name = if arg.starts_with(ARG_PREFIX) {
        &arg[ARG_PREFIX.len()..]
    } else {
        &arg[SHORT_ARG_PREFIX.len()..]
    };
    println!("arg name: {}", arg_name);

    Ok(())
}

fn parse(args: &[String]) -> Result<(), CMDError> {
    let mut iter = args.iter();

    while let Some(arg) = iter.next() {
        validate_arg(arg)?;
    }

    Ok(())
}

fn main() {
    // std::env::args will give us an iterator over the arguments of a process.
    // https://doc.rust-lang.org/std/env/struct.Args.html
    // We can use the collect() method of the iterator to transform the
    // iterator into a collection.
    // https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.collect
    let args: Vec<String> = env::args().collect();
    println!("process arguments: {:?}", args);

    // For the sake of experimenting, let's say that we want to call out to
    // "somthing" else. Let's use the linux mentality and use `--` to signal the
    // end of the options for this command and the begining for the options of
    // the command we will call out to. (This will give us another excuse to
    // use iterators).
    // We will also skip the very first value since that is the name of this
    // program.
    let (args, extra_args) = split_args(&args[1..]);
    println!("args: {:?}\nextra args: {:?}", args, extra_args);

    // split_args shows how to search and find where something is inside of a
    // collection. But, let's say you just want to know if something is inside
    // of the collection.
    // In this case, slices (like many other collections) have a `contains`
    // method.
    // https://doc.rust-lang.org/std/primitive.slice.html#method.contains
    //
    // Also note that the contains method expects as argument something of the
    // same type as what it is in the slice. Recall that we have slices of
    // Strings, so we must cast the string literal "--help" into a String type.
    if args.contains(&"--help".to_string()) {
        println!("we need help!");
    }

    // If we wanted to avoid casting our string literal into a String we could
    // also done the previous step as follows
    if args.iter().any(|arg| arg == "--help") {
        println!("yup, we need help");
        process::exit(0);
    }

    let argument = Argument::new("my-arg");
    println!("my argument is: {:?}", argument);

    match parse(args) {
        Err(err) => {
            println!("oops: {:?}", err);
            process::exit(1);
        }
        _ => {
            println!("we parsed your arguments!");
        }
    }
}
