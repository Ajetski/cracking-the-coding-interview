type StringBuffer = [char; 100];

fn urlify(s: &mut StringBuffer, mut len: usize) {
	let mut old_string_idx = len;
	for i in 0..old_string_idx {
		if s[i] == ' ' {
			len += 2;
		}
	}

	s[len] = '\0';

	let mut new_string_idx = len;

	while new_string_idx != old_string_idx {
		if s[old_string_idx] == ' ' {
			s[new_string_idx] = '0';
			new_string_idx -= 1;
			s[new_string_idx] = '2';
			new_string_idx -= 1;
			s[new_string_idx] = '%';
			new_string_idx -= 1;
			old_string_idx -= 1;
		} else {
			s[new_string_idx] = s[old_string_idx];
			old_string_idx -= 1;
			new_string_idx -= 1;
		}
	}
}

#[cfg(test)]
mod tests {
    use super::{urlify, StringBuffer};

    #[test]
    fn problem_1_3() {
			let mut s: StringBuffer = ['\0'; 100];
			let mut i: usize = 0;

			let original_str = "this is a string that has some spaces.";

			for c in original_str.chars() {
				s[i] = c;
				i += 1;
			}

			urlify(&mut s, original_str.len());

			let result: String = s.iter().collect();
			let ans = "this%20is%20a%20string%20that%20has%20some%20spaces.";
			for i in 0..ans.len() {
				assert_eq!(result.chars().nth(i).unwrap(), ans.chars().nth(i).unwrap());
			}
			assert_eq!(result.chars().nth(ans.len()).unwrap(), '\0');
    }
}
