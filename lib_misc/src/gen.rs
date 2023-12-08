use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

/*
    function that generates a random name with a prefix of 'wo_'
*/
pub fn name() -> Result<String, String> {
    let generate_name: String = thread_rng()
        .sample_iter(Alphanumeric)
        .take(10)
        .map(char::from)
        .collect();

    let name = format!("wo_{}", generate_name);

    if name.starts_with("wo_")
        && name.len() == 13
        && name[3..].chars().all(|c| c.is_ascii_alphanumeric())
    {
        Ok(name)
    } else {
        Err(String::from("failed to generate a temporary filename")) 
    }
}

/*
    UNIT-TESTS SECTION BEGINS
*/
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_name() {
        let random_name = name().unwrap();

        assert!(random_name.starts_with("wo_"));
        assert_eq!(random_name.len(), 13);
        assert!(random_name[3..].chars().all(|c| c.is_ascii_alphanumeric()));
    }
}
