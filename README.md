# gotemplate-rs

Rust library for rendering Go templates. Uses FFI to call Go's `text/template` library directly.
Includes [Sprig][1] functions in the template scope. Data to inject must be passed as a JSON string.

Mostly an experiment to play around with CGo and Rust. Use at your own risk!

## Example

```rust
#[macro_use]
extern crate json_str;
extern crate gotemplate;

use std::borrow::Borrow;

fn main() {
    let template_body = "Distance: {{ .query.filtered.filter.geo_distance.distance }}";
    let json_data = json_str!({
         "query": {
             "filtered": {
                 "query": {
                     "match_all": {}
                 },
                 "filter": {
                     "geo_distance": {
                         "distance": "20km",
                         "location": {
                             "lat": 37.776,
                             "lon": -122.41
                         }
                     }
                 }
             }
        }
    });

    println!("Rendering a Go template in Rust!\n");
    println!("With template:\n```\n{}\n```\n", template_body);
    println!("With JSON data:\n```\n{}\n```\n", json_data);

    let rendered = gotemplate::render_go_template(template_body, json_data.borrow()).unwrap();
    println!("Successful render! Result:\n```\n{}\n```", rendered);
}
```


[1]: https://masterminds.github.io/sprig/
