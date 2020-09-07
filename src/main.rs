use std::io::Read;

fn main() {
  let mut input = String::new();

  std::io::stdin().read_to_string(&mut input).unwrap();
  let bytes = input.bytes().collect::<Vec<_>>();

  println!(
    "let v = vec!{:?}; println!(\"{{}}\", unsafe {{ String::from_utf8_unchecked(v) }});",
    bytes
  );
}
