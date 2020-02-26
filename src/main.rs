use std::fs::File;
use std::io::BufReader;

fn main() {
    println!("Hello, world!");

    let file_in: File = File::open(&"in.yml").expect("Could not load file");
    let file_out: File = File::create(&"out.json").expect("Could not load file");

    let reader = BufReader::new(file_in);
    let value: serde_yaml::Value = serde_yaml::from_reader(reader).unwrap();

    serde_json::to_writer_pretty(file_out, &value);
}

// fn transcoded() {
//     // A JSON input with plenty of whitespace.
//     let input: &str = r#"
//       {
//         "a boolean": true,
//         "an array": [3, 2, 1]
//       }
//     "#;

//     // A JSON deserializer. You can use any Serde Deserializer here.
//     let mut deserializer = serde_json::Deserializer::from_str(input);

//     // A compacted JSON serializer. You can use any Serde Serializer here.
//     let mut serializer = serde_json::Serializer::new(io::stdout());

//     // Prints `{"a boolean":true,"an array":[3,2,1]}` to stdout.
//     // This line works with any self-describing Deserializer and any Serializer.
//     serde_transcode::transcode(&mut deserializer, &mut serializer).unwrap();
// }
