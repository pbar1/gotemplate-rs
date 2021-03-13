use std::borrow::Borrow;

#[macro_use]
extern crate json_str;
extern crate gotemplate;

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

    let rendered = gotemplate::render(template_body, json_data.borrow()).unwrap();
    println!("Successful render! Result:\n```\n{}\n```", rendered);
}
