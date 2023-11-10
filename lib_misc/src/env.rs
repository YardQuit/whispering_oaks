use std::env;

pub fn editor_selection() -> String {
    if let Ok(editor) = env::var("WHISPERING_OAKS") {
        return editor;
    }

    if let Ok(editor) = env::var("EDITOR") {
        return editor;
    }

    panic!("\nerror: there is no editor set as either prffered or default)");    
}

/*
    UNIT-TESTS SECTION BEGINS
*/
// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_env_variable_for_editor() {}
// }
