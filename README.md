Working through [Server-Driven Web Apps with htmx](https://pragprog.com/titles/mvhtmx/server-driven-web-apps-with-htmx/) using Rust

```
cargo watch -c -x run
```

Then browse to <http://localhost:3000>

## Book notes

### Chapter 1

First part of "Jumping in" is in `main.rs` and also tagged as `p01-hello-world` in this repo.

The second part of chapter 1, "Creating a CRUD application", is in `dogs.rs` (and `.html`, and `.css`) files. Tagged here as `p02-crud`.

### Chapter 3

"Developing endponts", follows the patterm of a module, html page with names to match the sections: 

- "oob" (`oob.html`, `oob/mod.rs`) for the "Performing Out-of-Band Swaps"
- "triggers" for the event triggers.
- the "dogs" example was updated in this chapter to support update.

Taggeg in git as `ch-3`.

### Chapter 4

`apps` are:

- "lazy" for lazy loading.
- "validate" for input validation. Note that the [online example](https://github.com/mvolkmann/htmx-examples/blob/f394f778794b21f6cfd58c474ccd3f75c6972a45/input-validation/src/server.tsx) is considerably more involved that the example in the book.


## Libraries used

- https://docs.rs/axum/latest/axum/index.html
- https://maud.lambda.xyz

