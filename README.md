# all-variants

Provides an EveryVariant trait that provides the every_variant() method on
types. Allows you to easily generate every combination of variants in
structures that contains Enums, or in nested enum trees. This to allow for
additional testing of codepaths where nested enum trees are used.

The derive macro EveryVariant will provide the every_variant() method for you,
with some preset values for the std types such as floats, integers and strings,

The generated data inside at the lowest level is currently fixed to specific
values. If you have types that are dependend on strings with a specific format
on the strings for example, I suggest that those be made into their own types and every_variant implemented manually for those.


## Example:

``` rust

use all_variants::EveryVariant;

/// Type of the message
#[derive(EveryVariant, Debug, Clone)]
enum MessageType {
    Codified,
    Markdown,
    Html,
}

/// This type should generate 4 different variant
#[derive(EveryVariant, Debug, Clone)]
struct FormattedMessage {
    /// Enum dictating how to render the string, None means its hidden
    rendermethod: Option<MessageType>,
    /// The optional content of the message
    text: String,
}

fn main() {
    let all_diferent_messages = FormattedMessage::every_variant();
    println!("{:#?}", all_diferent_messages);
}


```

the output will be:

```
[
    FormattedMessage {
        rendermethod: None,
        text: "example String",
    },
    FormattedMessage {
        rendermethod: Some(
            Codified,
        ),
        text: "example String",
    },
    FormattedMessage {
        rendermethod: Some(
            Markdown,
        ),
        text: "example String",
    },
    FormattedMessage {
        rendermethod: Some(
            Html,
        ),
        text: "example String",
    },
]
```

