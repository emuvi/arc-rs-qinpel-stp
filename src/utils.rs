use std::env;

pub fn get_os<'a>() -> &'a str {
  let os = env::consts::OS;
  if os.len() < 3 {
      panic!("Error: Operation system is not supported.");
  }
  let result = &os[..3];
  match result {
      "lin" | "mac" | "win" => result,
      _ => panic!("Error: Operation system is not supported."),
  }
}

pub fn get_arch<'a>() -> &'a str {
  match std::mem::size_of::<&char>() {
      4 => "32",
      8 => "64",
      _ => panic!("Error: System architecture is not supported."),
  }
}