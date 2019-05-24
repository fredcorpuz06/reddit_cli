// This program is a CLI for the Reddit API that allows a user to 
// parse through subreddits, submissions and comments by following
// a series of prompts that appear on the command line.

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

mod moves;
mod get_reddit;
mod string_helpers;
use moves::MoveLevel;
use get_reddit::RedditApiCall;
use string_helpers::UserChoices;
use std::path::Path;

// mod csv_writer;

fn main() {
    // List all user choices at each branch
    let all_choices = Path::new("all_choices.json");
    let user_choices0: Vec<UserChoices> = string_helpers::read_json_file(&all_choices);
    
    // Default values
    let mut subreddit = String::new(); // Fitness
    let mut submission = String::new(); // bpiq02
    let mut submission_rank = String::new(); // /hot
    let mut comment = String::new(); // entsz9a
    let mut comment_rank = String::new(); // &sort=confidence
    let mut view_preference: usize = 5;
    // let mut records: Vec<Vec<str>> = Vec::new();
    // csv_writer::write_to_csv(records);
    run(user_choices0,
        &mut subreddit, &mut submission, &mut submission_rank,
        &mut comment, &mut comment_rank, &mut view_preference)
}


fn run(
    user_choices0: Vec<UserChoices>,
    subreddit: &mut String, 
    submission: &mut String, submission_rank: &mut String,
    comment: &mut String, comment_rank: &mut String,
    view_preference: &mut usize
) {
    loop {
        // Subreddit level
        // Acquire user input + Print response
        let move_subreddit: MoveLevel = moves::subreddits_logic(
            subreddit, &user_choices0);
        let mut base_uri = format!("https://www.reddit.com/r/{}.json?limit={}", 
            subreddit, view_preference);
        println!("Subreddit Level: {}", base_uri); // https://www.reddit.com/r/Fitness.json?limit=5
        get_reddit::print_reddit_rez(
            &base_uri, get_reddit::RedditApiCall::Subreddit);
        
        // Navigate menu logic
        match move_subreddit {
            MoveLevel::Back => break,
            MoveLevel::Stay => continue,
            MoveLevel::Forward => loop {
                // Submission level: Set our default values 
                string_helpers::clear_string_vals(vec![submission, submission_rank, 
                    comment, comment_rank]);
                // Acquire user input + Print response  
                let user_choices1 = &user_choices0[0].next_prompts;  
                let (move_submission, rez_type): (MoveLevel, get_reddit::RedditApiCall) =
                    moves::submissions_logic(
                        submission, submission_rank,
                        view_preference, &user_choices1);
                base_uri = match rez_type {
                    RedditApiCall::Subreddit =>
                        format!("https://www.reddit.com/r/{}{}.json?limit={}",
                            subreddit, submission_rank, view_preference),
                    RedditApiCall::Submission => 
                        format!("https://www.reddit.com/r/{}/comments/{}.json?limit={}",
                            subreddit, submission, view_preference),
                    _ => String::new()
                };                       
                println!("Submissions Level: {}", base_uri); // https://www.reddit.com/r/Fitness/comments/bpiq02.json?limit=5
                get_reddit::print_reddit_rez(&base_uri, rez_type);

                match move_submission {
                    MoveLevel::Back => break,
                    MoveLevel::Stay => continue,
                    MoveLevel::Forward => loop {
                        // Comment level: Set our default values 
                        string_helpers::clear_string_vals(vec![submission_rank, 
                            comment, comment_rank]);
                        // Acquire user input + Print response
                        let user_choices2 = &user_choices1[0].next_prompts;
                        let (move_comments, rez_type): (MoveLevel, get_reddit::RedditApiCall) = 
                            moves::comments_logic(
                                comment, comment_rank,
                                view_preference, &user_choices2);
                        base_uri = match rez_type {
                            RedditApiCall::Subreddit =>
                                format!("https://www.reddit.com/r/{}?limit={}",
                                    subreddit, view_preference),
                            RedditApiCall::Submission =>
                                format!("https://www.reddit.com/r/{}/comments/{}.json?limit={}{}",
                                    subreddit, submission, view_preference, comment_rank),
                            RedditApiCall::Comment => 
                                format!("https://www.reddit.com/r/{}/comments/{}/{}.json?limit={}",
                                    subreddit, submission, comment, view_preference),
                        };
                        println!("Comments Level: {}", base_uri);
                        get_reddit::print_reddit_rez(&base_uri, rez_type);
                        
                        match move_comments {
                            MoveLevel::Back => break,
                            MoveLevel::Stay => continue,
                            MoveLevel::Forward => loop {
                                let user_choices3 = &user_choices2[0].next_prompts;
                                let move_interact: MoveLevel = 
                                    moves::comment_interact_logic(&user_choices3);
                                match move_interact {
                                    MoveLevel::Back => break,
                                    _ => continue,
                                }
                            }
                            
                        }

                    }
                }
            },
        }

    }

}

