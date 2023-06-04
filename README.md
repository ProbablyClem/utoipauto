<div align="center">
  <h1>utoipa_auto_discovery</h1>
  <p>
    <strong>Rust Macros to automate the addition of Paths/Schemas to Utoipa crate, simulating Reflection during the compilation phase</strong>
  </p>
  <p>

![MSRV](https://img.shields.io/badge/rustc-1.69+-ab6000.svg)

  </p>
</div>

# Crate presentation

Utoipa is a great crate for generating documentation (openapi/swagger) via source code.

But since Rust is a static programming language, we don't have the possibility of automatically discovering paths and dto in runtime and adding them to the documentation,

for APIs with just a few endpoints, it's not that much trouble to add controller functions one by one, and DTOs one by one.

if you have hundreds or even thousands of endpoints, the code becomes very verbose and difficult to maintain.

ex :

```rust

...

#[derive(OpenApi)]
#[openapi(
    paths(
        // <================================ all functions  1 to N
        test_controller::service::func_get_1,
        test_controller::service::func_get_2,
        test_controller::service::func_get_3,
        test_controller::service::func_get_4,
       ....
       ....
       ....
        test_controller::service::func_get_N,

    ),
    components(
        // <====================== All DTO one by one
        schemas(TestDTO_1,  TestDTO_2, ........ , TestDTO_N)
    ),
    tags(
        (name = "todo", description = "Todo management endpoints.")
    ),
    modifiers(&SecurityAddon)
)]
pub struct ApiDoc;

...

```

The aim of crate **utoipa_auto_discovery** is to propose a macro that automates the detection of methods carrying Utoipa macros (`#[utoipa::path(...]`), and adds them automatically. (it also detects sub-modules.)

# how to use it

simply add the crate `utoipa_auto_discovery` to the project

```
cargo add utoipa_auto_discovery
```

then add the `#[utoipa_auto_discovery]` macro just before the `#[openapi]` macro.

```rust
#[utoipa_auto_discovery(paths = "MODULE_TREE::MODULE_NAME | MODULE_SRC_FILE_PATH; MODULE_TREE::MODULE_NAME | MODULE_SRC_FILE_PATH; ... ;")]
```

the paths receives a String which must respect this structure :

`"{MODULE_TREE_PATH} | {MODULE_SRC_FILE_PATH} ;"`

you can add several pairs (Module Path | Src Path ) by separating them with a semicolon ";".

Here's an example of how to add all the methods contained in the test_controller and test2_controller modules.
you can also combine automatic and manual addition, as here we've added a method manually to the documentation "other_controller::get_users".

```rust
...

use utoipa_auto_discovery::utoipa_auto_discovery;

...

#[derive(OpenApi)]
#[utoipa_auto_discovery(
  paths = "crate::rest::test_controller | ./src/rest/test_controller.rs ; crate::rest::test2_controller | ./src/rest/test2_controller.rs")]
#[openapi(
    paths(

        crate::rest::other_controller::get_users,
    ),
    components(
        schemas(TestDTO)
    ),
    tags(
        (name = "todo", description = "Todo management endpoints.")
    ),
    modifiers(&SecurityAddon)
)]

pub struct ApiDoc;

...

```

## note

sub-modules within a module containing methods tagged with utoipa::path are also automatically detected.

# Features

- [x] automatic path detection
- [ ] automatic schema detection (in progress)
