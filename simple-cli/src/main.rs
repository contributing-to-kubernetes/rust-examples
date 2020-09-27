use std::collections::BTreeMap;
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

    fn required(mut self, required: bool) -> Self {
        self.required = required;
        self
    }

    fn takes_value(mut self, takes_value: bool) -> Self {
        self.takes_value = takes_value;
        self
    }

    fn help(mut self, help: &'a str) -> Self {
        self.help = Some(help);
        self
    }
}

#[derive(Debug)]
enum CMDError {
    UnexpectedArgument(String),
    DuplicateArgument(String),
}

#[derive(Debug, Default)]
struct Arguments<'a> {
    args: BTreeMap<&'a str, Argument<'a>>,
}

impl<'a> Arguments<'a> {
    fn new() -> Self {
        Arguments::default()
    }

    fn insert_arg(mut self, argument: Argument<'a>) -> Self {
        // We ignore the returned value of an insert operation - we just want
        // the argument into our BTreeMap.
        // https://doc.rust-lang.org/beta/std/collections/struct.BTreeMap.html#method.insert
        self.args.insert(argument.name, argument);
        self
    }

    // When validating command line arguments, we want to get a descriptive error
    // if something is wrong, but we don't really care about the Ok value.
    //
    // If we make this function not borrow the variable (move the arguments), then
    // we will run into an issue because this function will move the Arguments.
    // See rustc --explain E0507 and try removing the '&' from self.
    fn validate_arg(&self, arg: &str) -> Result<(), CMDError> {
        if !arg.starts_with(ARG_PREFIX) && !arg.starts_with(SHORT_ARG_PREFIX) {
            return Err(CMDError::UnexpectedArgument(arg.to_string()));
        }

        let arg_name = if arg.starts_with(ARG_PREFIX) {
            &arg[ARG_PREFIX.len()..]
        } else {
            &arg[SHORT_ARG_PREFIX.len()..]
        };

        // Check that the argument provided as part of the set we expect.
        let argument = self
            .args
            .get(arg_name)
            .ok_or_else(|| CMDError::UnexpectedArgument(arg_name.to_string()))?;

        if argument.user_value.is_some() {
            return Err(CMDError::DuplicateArgument(arg_name.to_string()));
        }

        Ok(())
    }
}

// About `'static`
// https://doc.rust-lang.org/stable/rust-by-example/scope/lifetime/static_lifetime.html
fn build_cmd_arguments() -> Arguments<'static> {
    Arguments::new()
        .insert_arg(
            Argument::new("id")
                .required(true)
                .takes_value(true)
                .help("jail ID"),
        )
        .insert_arg(Argument::new("daemonize").help("Daemonize the jailer before execing"))
}

fn parse(app_arguments: &Arguments<'static>, args: &[String]) -> Result<(), CMDError> {
    let mut iter = args.iter();

    while let Some(arg) = iter.next() {
        app_arguments.validate_arg(arg)?;
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

    let app_arguments = build_cmd_arguments();
    println!("our parsed arguments are: {:?}", app_arguments);

    match parse(&app_arguments, args) {
        Err(err) => {
            println!("oops: {:?}", err);
            process::exit(1);
        }
        _ => {
            println!("we parsed your arguments!");
        }
    }
}
