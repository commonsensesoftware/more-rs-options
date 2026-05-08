{{#include guide/links.md}}

# Getting Started

The simplest way to get started is to install the crate using the default features.

```bash
cargo add more-options
```

## Example

The options pattern uses structures to provide strongly typed access to groups of related settings. Options also provide
a mechanism to validate configuration data.

Let's build the ubiquitous _Hello World_ application. The first thing we need to do is define a couple of structures.

```rust
use options::Options;
use std::rc::Rc;

pub struct SpeechSettings {
    pub text: String,
    pub language: String,
}

pub struct Person {
    speech: Rc<SpeechSettings>,
}

impl Person {
    pub fn speak(&self) -> &str {
        &self.speech.text
    }
}
```

You may be wondering why you need `Rc`. In many practical applications, you'll have a group of immutable settings which
will be used in many places. `Rc` (or `Arc`) allows a single instance of settings to be shared throughout the
application. You can also use the [Ref] type alias to switch between them depending on whether the **async** feature is
enabled.

Now that we have some options, we can put it together in a simple application.

```rust
use crate::options;

fn main() {
    let options = SpeechSettings {
        text: "Hello world!".into(),
        language: "en".into(),
    };
    let person = Person { speech: options.into() };

    println!("{}", person.speak());
}
```