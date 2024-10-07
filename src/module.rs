use self::functions::Function;

mod functions;

struct Module<'a> {
    functions: Vec<Function<'a>>,
}
