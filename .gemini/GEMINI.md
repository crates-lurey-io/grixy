# Guidelines for Gemini and LLMs

## Prefer following the [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/checklist.html)

Naming (crate aligns with Rust naming conventions)
- Casing conforms to RFC 430
- Ad-hoc conversions follow as_, to_, into_ conventions
- Getter names follow Rust convention
- Methods on collections that produce iterators follow iter, iter_mut, into_iter
- Iterator type names match the methods that produce them
- Feature names are free of placeholder words
- Names use a consistent word order

Interoperability (crate interacts nicely with other library functionality)
- Types eagerly implement common traits

Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Display, Default
- Conversions use the standard traits From, AsRef, AsMut
- Collections implement FromIterator and Extend
- Data structures implement Serde's Serialize, Deserialize
- Types are Send and Sync where possible
- Error types are meaningful and well-behaved
- Binary number types provide Hex, Octal, Binary formatting
- Generic reader/writer functions take R: Read and W: Write by value

Macros (crate presents well-behaved macros)
- Input syntax is evocative of the output
- Macros compose well with attributes
- Item macros work anywhere that items are allowed
- Item macros support visibility specifiers
- Type fragments are flexible

Documentation (crate is abundantly documented)
- Crate level docs are thorough and include examples
- All items have a rustdoc example
- Examples use ?, not try!, not unwrap
- Function docs include error, panic, and safety considerations
- Prose contains hyperlinks to relevant things
- Cargo.toml includes all common metadata
authors, description, license, homepage, documentation, repository, keywords, categories
- Release notes document all significant changes
- Rustdoc does not show unhelpful implementation details

Predictability (crate enables legible code that acts how it looks)
- Smart pointers do not add inherent methods
- Conversions live on the most specific type involved
- Functions with a clear receiver are methods
- Functions do not take out-parameters
- Operator overloads are unsurprising
- Only smart pointers implement Deref and DerefMut
- Constructors are static, inherent methods

Flexibility (crate supports diverse real-world use cases)
- Functions expose intermediate results to avoid duplicate work
- Caller decides where to copy and place data
- Functions minimize assumptions about parameters by using generics
- Traits are object-safe if they may be useful as a trait object

Type safety (crate leverages the type system effectively)
- Newtypes provide static distinctions
- Arguments convey meaning through types, not bool or Option
- Types for a set of flags are bitflags, not enums
- Builders enable construction of complex values

Dependability (crate is unlikely to do the wrong thing)
- Functions validate their arguments
- Destructors never fail
- Destructors that may block have alternatives

Debuggability (crate is conducive to easy debugging)
- All public types implement Debug
- Debug representation is never empty

Future proofing (crate is free to improve without breaking users' code)
- Sealed traits protect against downstream implementations
- Structs have private fields
- Newtypes encapsulate implementation details
- Data structures do not duplicate derived trait bounds

Necessities (to whom they matter, they really matter)
- Public dependencies of a stable crate are stable
- Crate and its dependencies have a permissive license
