**Under heavy research and development, please don't use this yet!**

# rsx
[![License: MPL 2.0](https://img.shields.io/badge/License-MPL%202.0-brightgreen.svg)](https://opensource.org/licenses/MPL-2.0)
[![Build Status](https://travis-ci.org/victorporof/rsx.svg?branch=master)](https://travis-ci.org/victorporof/rsx)

A compiler plugin for using RSX (JSX-like syntax) as advanced templating and metaprogramming in Rust.

Made possible by the [Self Tokenize](https://github.com/victorporof/rsx) library, a trait derive for transferring data structures outside of procedural macros from compile-time to run-time.

Take a look at the [RSX DOM](https://github.com/victorporof/rsx-dom) and [RSX Stylesheet](https://github.com/victorporof/rsx-stylesheet) crates for the underlying types and implementations, or the [RSX parser](https://github.com/victorporof/rsx-parser) and [Servo CSS parser](https://github.com/victorporof/servo-css-parser) parsing backends for the parser combinators. To convert these data structures into lower level rendering primitives, see [RSX Layout](https://github.com/victorporof/rsx-layout) and [RSX Primitives](https://github.com/victorporof/rsx-primitives), which integrate with [Facebook's YOGA](https://facebook.github.io/yoga/) library and [Servo's Graphics](https://github.com/servo/servo/tree/89d5780570894a54774542e79585b79ece3f2dce/components/gfx) component for building a Servo [WebRender](https://github.com/servo/webrender)-powered `gfx::display_list::DisplayList`. Finally, rendering to pixels is done via the [RSX Renderers](https://github.com/victorporof/rsx-renderers) crate.

For quick and easy example demos, simply check out [here](https://github.com/victorporof/rsx-demo).

## Purpose
[Documentation](https://victorporof.github.io/rsx)

This compiler plugin allows you to freely intertwine JSX-like syntax anywhere into your Rust code.

RSX implements all of the [JSX](http://facebook.github.io/jsx) grammar. The [purpose and benefits](https://reactjs.co/2015/08/04/advantages-of-jsx/) of JSX and RSX are equivalent.

## How to use
To get access to the `rsx!`, `css!` macros, add this to your `Cargo.toml` file:

```toml
[dependencies]
rsx = { git = "https://github.com/victorporof/rsx.git" }
rsx-primitives = { git = "https://github.com/victorporof/rsx-primitives.git" }
```

Then, simply import the library into your code and use the `rsx!`, `css!` macros to parse RSX and CSS into `rsx_dom::DOMNode`, or `rsx_dom::Stylesheet` data structures respectively.

For example:

```rust
#![feature(proc_macro)]

extern crate rsx;
extern crate rsx_primitives;

use rsx::{rsx, css};
use rsx_primitives::rsx_stylesheet::types::*;
use rsx_primitives::rsx_dom::types::*;

let stylesheet: Stylesheet = css! { .foo { padding: 1px; } };
let node: DOMNode = rsx! { <div>Hello world!</div> };
```

Here's some code rendering the first example from [Facebook's YOGA](https://facebook.github.io/yoga/) library:

```rust
let stylesheet: Stylesheet = css! {
  .root {
    width: 500px;
    height: 120px;
    flex-direction: row;
    padding: 20px;
  }
  .image {
    width: 80px;
    margin-right: 20px;
  }
  .text {
    height: 25px;
    align-self: center;
    flex-grow: 1;
  }
};

let node: DOMNode = rsx! {
  <view style={stylesheet.take(".root")}>
    <image style={stylesheet.take(".image")} src="..." />
    <text style={stylesheet.take(".text")}>
      Hello world!
    </text>
  </view>
};
```

### Composability

- Mixing Rust and RSX is possible
- Stylesheets can be included as separate CSS files.
- Composing components is achieved through simple function calls (for now).

#### example.css
```css
.root {
  width: 500px;
  height: 120px;
  flex-direction: row;
  padding: 20px;
}
.image {
  width: 80px;
  margin-right: 20px;
}
.text {
  height: 25px;
  align-self: center;
  flex-grow: 1;
}
```

#### example.rs
```rust
fn greeting_str(name: &str) -> String {
  format!("Hello {}!", name)
}

fn render_greeting(name: &str) -> DOMNode {
  let stylesheet = css!("example.css");

  rsx! {
    <text style={stylesheet.take(".text")}>
      { greeting_str(name) }
    </text>
  }
}

fn render_children(name: Option<&str>, image: DOMNode) -> DOMNode {
  rsx! {
    <view>
      { image }
      {
        match name {
          Some(ref n) => render_greeting(n),
          None => <text>No greetings!</text>
        }
      }
    </view>
  }
}

fn render_root() -> DOMNode {
  let stylesheet = css!("example.css");

  rsx! {
    <view style={stylesheet.take(".root")}>
      {
        let name = Some("world");
        let image = <image style={stylesheet.take(".image")} src="..." />;
        render_children(name, image)
      }
    </view>
  }
}

let node = render_root();
```

The `css!` macro returns a `rsx_dom::Stylesheet` instance (coming from the [RSX Stylesheet library](https://github.com/victorporof/rsx-stylesheet) re-exported through the [RSX DOM library](https://github.com/victorporof/rsx-dom)), because parsing CSS happens at compile-time.

```rust
let styles: rsx_stylesheet::Stylesheet = css! { ... }
```

The `rsx!` macro returns a `rsx_dom::DOMNode` instance (coming from the [RSX DOM library](https://github.com/victorporof/rsx-dom)). The convertion is automatic between `rsx_parser::RSXElement` abstract syntax trees to the more convenient `rsx_dom::DOMNode` elements, because the AST is directly tokenized into a DOM tree to avoid any runtime work! Templating is thus a zero cost abstraction.

```rust
let node: rsx_dom::DOMNode = rsx! { ... }
```
