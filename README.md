# routem: a type-aware route parsing library

`routem` is a Rust crate which allows you to match paths to specs for different
routes. The main use is to allow you to specify that a path only matches if its
parameters are of the correct type.


# Installation

To add `routem` to your Rust project, install it using Cargo:

```
cargo add routem
```

It compiles with stable and has been tested with Cargo 1.74.1.



# Usage and examples

The core structs in `routem` are `Route`, `Routes`, and `Parser`.

- `Route` is a spec for a particular route in your application. It contains
  a name (useful to finding associated data after you find the matching route)
  and a spec for the route. You can use it to query whether or not a particular
  path matches this spec.
- `Routes` contains multiple `Route`s and can be used to check which of all of
  your configured routes matches a path. Useful for finding a handler for
  incoming requests, or simply verifying that a match exists at all.
- `Parser` is used to construct routes. By default it is configured with three
  parameter types (string, 64-bit int, and UUID)


Here's an end-to-end example of creating a parser, configuring routes, then
finding the matching route (if any) for a given path.

```rust
use routem::Parser;

let parser = Parser::default();

let user_route = parser.route("user-by-id", "/user/<id:uuid>/").unwrap();
let club_route = parser.route("club-by-id", "/user/<id:int>/").unwrap();

routes.add(user_route);
routes.add(club_route);


routes.find("/user/36be8705-6c31-45d7-9321-d56cc07b50d9/") // Some(user_route)
routes.find("/club/123/"); // Some(club_route)

routes.find("/user/123/"); // None
routes.find("/club/36be8705-6c31-45d7-9321-d56cc07b50d9/") // None
routes.find("/club/123"); // None
```

Here's an example where we add a custom parameter type.
Adding custom types is optional.

```rust
use routem::{Parser, ParamType};

fn is_palindrome(ident: &str) -> bool {
    ident.chars().rev().collect::<String>() == ident
}

let custom_type = ParamType::new("palindrome", is_palindrome);

let parser = Parser::default();
parser.add_param_type(custom_type);


let club_route = parser.route("club-by-id", "/club/<id:palindrome>/").unwrap();

assert_eq!(None, routes.find("/club/myclub/"));
assert_eq!(Some(&club_route), routes.find("/club/radar/"));
```


# Roadmap

There are a few nice-to-have features currently missing from routem.
Here's what is currently planned to be developed eventually:

- [ ] Query parameter validation and parsing
- [ ] Union types in route and query parameters




# License

Copyright on the initial code is held by [Remesh](https://remesh.ai). Any external
contributors retain their copyright; we do not seek copyright assignment.

This software is [dual-licensed](https://en.wikipedia.org/wiki/Multi-licensing)
under the Apache v2 and MIT licenses.


See [LICENSE-APACHE](LICENSE-APACHE) and [LICENSE-MIT](LICENSE-MIT) for
details.


