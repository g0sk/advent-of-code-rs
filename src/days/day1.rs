//Gets a text line and retrieves the first and last number inside it
pub fn get_calibration_value(line: String) -> String {
  let mut value: String = String::new();
  let mut first_occurrence: bool = true;
  let mut first_found_value: Option<char> = None;
  let mut last_found_value: Option<char> = None;

  for c in line.chars() {
    if c.is_numeric() {
      if first_occurrence {
        value.push(c);
        first_occurrence = false;
        first_found_value = Some(c);
      } else {
        last_found_value = Some(c);
      }
    }
  }
  if last_found_value.is_some() {
    value.push(last_found_value.unwrap());
  } else {
    value.push(first_found_value.unwrap())
  }
  value
}