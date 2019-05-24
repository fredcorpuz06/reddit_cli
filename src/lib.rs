// Attempt to write tests
#[cfg(test)]
mod tests{
    use crate::string_helpers;
    // mod string_helpers;
    #[test]
    fn it_clears_strings() {
        assert_eq!(
            string_helpers::clear_string_vals(vec![String::from("hello"), String::from("hi")]),
            vec![String::new(), String::new()]
        );
    }

    use crate::get_reddit;

    #[test]
    fn it_unwraps_str() {
        assert_eq!(
            get_reddit::unwrap_mystr(Option<&"Fred"),
            "Fred");
        assert_eq!(
            get_reddit::unwrap_mystr(Option<&""),
            "'NA'");
    }

    use crate::moves;
    use crate::string_helpers::UserChoices;
    #[test]
    fn logic_works() {
        let uc1 = UserChoices {
            prompt: String::from("prompt?"),
            url_segment: String::from("urlseg"),
            next_prompts: vec![String::from("hello?"), String::from("hi?")]
        };
        let uc2 = UserChoices {
            prompt: String::from("prompt?"),
            url_segment: String::from("urlseg"),
            next_prompts: vec![String::from("hello?"), String::from("hi?")]
        };

        assert_eq!(
            moves::comment_interact_logic(&vec![uc1, uc2]),
            moves::MoveLevel::Back
        );
    }
}