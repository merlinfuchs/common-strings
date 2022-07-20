### Macro for storing common strings as enum variants

This crate provides a single macro for storing common strings as enum variants. If you are storing a lot of
strings where a few values often occur, this can reduce the memory usage.

You can enable the `serde` feature to add serialization and deserialization support.

#### Example

An example for this are Discord channel names. Names like `general`, `offtopic`, `support`, `staff`, ... make up a large
percentage of all channel names. Storing a string for each channel called `general` wastes a lot of resources.

```rust
use common_strings::{common_strings, CommonStrings};

common_strings!(
    #[derive(Clone, Debug)]
    pub enum ChannelName {
        const General = "general";
        const Offtopic = "offtopic";
        const Support = "support";
        const Staff = "staff";
    }
);

fn main() {
    let channel_name = ChannelName::General;
    println!("{}", channel_name.as_ref()); // general

    let channel_name = ChannelName::Other(String::from("my-channel"));
    println!("{}", channel_name); // my-channel

    let channel_name = ChannelName::from_cow("offtopic".into());
    println!("{}", channel_name.into_string()); // offtopic
}
```