// All user movement/navigation of subreddit, submission, comments
// handled here. Mutable references to variables declared and maintained
// in main are used.

use crate::get_reddit;
use crate::string_helpers::{self,UserChoices};

pub enum MoveLevel {
    Forward,
    Stay,
    Back,
}



// In: https://www.reddit.com
// Out: https://www.reddit.com/r/Fitness.json?limit=5
// Out: https://www.reddit.com/r/trendingsubreddits.json
pub fn subreddits_logic(
    my_subreddit: &mut String, choice_set: &Vec<UserChoices>,
) -> MoveLevel {
    let n_choices: usize = choice_set.len();
    let choice: usize = string_helpers::prompt_user_choice(&choice_set);

    if choice == 0 {
        *my_subreddit = string_helpers::prompt_read_string("Subreddit? ");
        return MoveLevel::Forward
    } else if (1 <= choice) && (choice <= n_choices-2) {
        *my_subreddit = "trendingsubreddits".to_string();
        return MoveLevel::Stay
    } else {
        return MoveLevel::Back
    }

}


// In: https://www.reddit.com/r/Fitness.json?limit=5
// Out: https://www.reddit.com/r/Fitness/comments/bpiq02.json?limit=5
// Out: https://reddit.com/r/Fitness/new.json?limit=5
pub fn submissions_logic(
    my_submission: &mut String, submission_rank: &mut String,
    view_count: &mut usize, choice_set: &Vec<UserChoices>,
) -> (MoveLevel, get_reddit::RedditApiCall) {
    let n_choices: usize = choice_set.len();
    let choice: usize = string_helpers::prompt_user_choice(&choice_set);

    if choice == 0 {
        *my_submission = string_helpers::prompt_read_string("Submission ID? ");
        return (MoveLevel::Forward, get_reddit::RedditApiCall::Submission)
    } else if (1 <= choice) && (choice <= n_choices-3) {
        *submission_rank = choice_set[choice].url_segment.to_string();
        return (MoveLevel::Stay, get_reddit::RedditApiCall::Subreddit)
    } else if choice == n_choices-2 {
        *view_count = string_helpers::prompt_read_idx("Count? ");
        return (MoveLevel::Stay, get_reddit::RedditApiCall::Subreddit)
    } else {
        return (MoveLevel::Back, get_reddit::RedditApiCall::Subreddit)
    }


}


// In: https://www.reddit.com/r/Fitness/comments/bpiq02.json?limit=5
// https://www.reddit.com/r/Fitness/comments/bpiq02/entsz9a.json?limit=5
// https://www.reddit.com/r/Fitness/comments/bpiq02.json?limit=5&sort=hot
pub fn comments_logic(
    my_comment: &mut String, comment_rank: &mut String, 
    view_count: &mut usize, choice_set: &Vec<UserChoices>,
) -> (MoveLevel, get_reddit::RedditApiCall) {
    let n_choices: usize = choice_set.len();
    let choice: usize = string_helpers::prompt_user_choice(&choice_set);

    if choice == 0 {
        *my_comment = string_helpers::prompt_read_string("Comment ID? ");
        return (MoveLevel::Forward, get_reddit::RedditApiCall::Comment)
    } else if (1 <= choice) && (choice <= n_choices-3) {
        *comment_rank = choice_set[choice].url_segment.to_string();
        return (MoveLevel::Stay, get_reddit::RedditApiCall::Submission)
    } else if choice == n_choices-2 {
        *view_count = string_helpers::prompt_read_idx("Count? ");
        return (MoveLevel::Stay, get_reddit::RedditApiCall::Submission)
    } else {
        return (MoveLevel::Back, get_reddit::RedditApiCall::Subreddit)
    }
}


pub fn comment_interact_logic(choice_set: &Vec<UserChoices>) -> MoveLevel {
    let _choice: usize = string_helpers::prompt_user_choice(&choice_set);
    println!("These features are not yet implemented \
        because they require OAuth2 to make POST requests \
        to the Reddit API");
    MoveLevel::Back
}



