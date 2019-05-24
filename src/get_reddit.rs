// Prints output (subreddits, submissions, comments) when given a URI 

use hyper::Client;
use hyper::rt::{self, Future, Stream};
use hyper_tls::HttpsConnector;
use serde_json::{self, Value};



// Triage pretty print functions
pub fn print_reddit_rez(raw_uri: &str, rez_type: RedditApiCall) {
    match rez_type {
        RedditApiCall::Subreddit => print_subreddit(raw_uri),
        RedditApiCall::Submission => print_submission(raw_uri),
        RedditApiCall::Comment => print_comment(raw_uri),
    }
}


#[derive(Deserialize, Debug)]
pub struct Subreddit {
    kind: Value,
    data: SubredditData,
}

#[derive(Deserialize, Debug)]
pub struct SubredditData {
    modhash: Value,
    dist: Value,
    children: Vec<Submissions>

}

#[derive(Deserialize, Debug)]
pub struct Submissions {
    data: SubmissionData
}

#[derive(Deserialize, Debug, Serialize)]
pub struct SubmissionData {
    #[serde(default)]
    id: Value, 
    #[serde(default)]
    author: Value,
    #[serde(default)]    
    ups: Value,
    #[serde(default)]
    downs: Value,
    #[serde(default)]
    num_comments: Value,
    #[serde(default)]
    body: Value,
    #[serde(default)]
    selftext: Value,
    #[serde(default)]
    title: Value,
}


pub enum RedditApiCall {
    Subreddit,
    Submission,
    Comment,
}


// Define a type so we can return multiple types of errors
pub enum FetchError {
    Http(hyper::Error),
    Json(serde_json::Error),
}

impl From<hyper::Error> for FetchError {
    fn from(err: hyper::Error) -> FetchError {
        FetchError::Http(err)
    }
}

impl From<serde_json::Error> for FetchError {
    fn from(err: serde_json::Error) -> FetchError {
        FetchError::Json(err)
    }
}

// Run future to grab Subreddit from Reddit API + Pretty Print
pub fn print_subreddit(raw_uri: &str) {
    let uri = raw_uri.parse().unwrap();
    let future = fetch_json_single(uri)
        .map(|subreddit| {
            println!("\n");
            println!("   id              Author     # Comments              Submission Title");
            for submission in &subreddit.data.children {
                let id1 = unwrap_mystr(submission.data.id.as_str());
                let title1 = unwrap_mystr(submission.data.title.as_str());
                let author1 = unwrap_mystr(submission.data.author.as_str());
                let n1 = unwrap_myint(submission.data.num_comments.as_u64());
                
                println!("[{id}]{author:>width$}  {n:<width2$}{title}", 
                    id = id1, title = title1, author = author1, 
                    n = n1, width = 20, width2 = 10)
            }
        })
        .map_err(|e| {
            match e {
                FetchError::Http(e) => eprintln!("http error: {}", e),
                FetchError::Json(e) => eprintln!("json parsing error: {}", e),
            }
        });
    rt::run(future);
}

// Run future to grab Submissions from Reddit API + Pretty Print
pub fn print_submission(raw_uri: &str) {
    let uri = raw_uri.parse().unwrap();
    let future = fetch_json_vec(uri)
        .map(|rez| {
            let submission = &rez[0];
            let comments = &rez[1];
            println!("== SUBMISSION ==");
            println!("[{id}] {n_comments} comments {n_ups} upvotes\nTitle: {title}\n{body}\n", 
                id = unwrap_mystr(submission.data.children[0].data.id.as_str()),
                title = unwrap_mystr(submission.data.children[0].data.title.as_str()),
                body = unwrap_mystr(submission.data.children[0].data.selftext.as_str()),
                n_comments = unwrap_myint(submission.data.children[0].data.num_comments.as_u64()),
                n_ups = unwrap_myint(submission.data.children[0].data.ups.as_u64()),
            );
            println!("== TOP COMMENTS ==");
            println!("   id              Author     # Replies              Comment");
            for comment in &comments.data.children {
                println!("[{id}]{author:>width$}  {n:<width2$}{body}", 
                    id = unwrap_mystr(comment.data.id.as_str()),
                    body = unwrap_mystr(comment.data.body.as_str()),
                    author = unwrap_mystr(comment.data.author.as_str()),
                    n = unwrap_myint(comment.data.ups.as_u64()) as usize,
                    width = 20,
                    width2 = 10)
            }
        })
        .map_err(|e| {
            match e {
                FetchError::Http(e) => eprintln!("http error: {}", e),
                FetchError::Json(e) => eprintln!("json parsing error: {}", e),
            }
        });
    rt::run(future);
}



pub fn print_comment(raw_uri: &str) {
    let uri = raw_uri.parse().unwrap();
    let future = fetch_json_vec(uri)
        .map(|rez| {;
            let comments = &rez[1];
            println!("== COMMENTS ==");
            println!("   id              Author     # Replies              Comment");
            for comment in &comments.data.children {
                println!("[{id}]{author:>width$}  {n:<width2$}{body}", 
                    id = unwrap_mystr(comment.data.id.as_str()),
                    body = unwrap_mystr(comment.data.body.as_str()),
                    author = unwrap_mystr(comment.data.author.as_str()),
                    n = unwrap_myint(comment.data.ups.as_u64()) as usize,
                    width = 20,
                    width2 = 10)
            }
        })
        .map_err(|e| {
            match e {
                FetchError::Http(e) => eprintln!("http error: {}", e),
                FetchError::Json(e) => eprintln!("json parsing error: {}", e),
            }
        });
    rt::run(future);
}


// Handle cases where string values are missing 
// from Reddit API response
pub fn unwrap_mystr(myobj: Option<&str>) -> &str {
    match myobj {
        Some(x) => x,
        None => "'NA'"
    }
}

// Handle cases where numeric values are missing 
// from Reddit API response
pub fn unwrap_myint(myobj: Option<u64>) -> u64 {
    let zero: u64 = 0;
    match myobj {
        Some(x) => x,
        None => zero
    }
}

// Parse single dictionary response from Reddit API
pub fn fetch_json_single(uri: hyper::Uri) -> impl Future<Item=Subreddit, Error=FetchError> {
    // Make sure function handles TLS 
    let https = HttpsConnector::new(4).expect("TLS initialization failed");
    let client = Client::builder()
        .build::<_, hyper::Body>(https);

    client
        .get(uri)
        .and_then(|res| {
            // asynchronously concatenate chunks of the body
            res.into_body().concat2()
        })
        .from_err::<FetchError>()
        .and_then(|body| {
            let users = serde_json::from_slice(&body)?; // handles errors using From
            Ok(users)
        })
        .from_err()
}

// Parse list of dictionaries from Reddit API
pub fn fetch_json_vec(uri: hyper::Uri) -> impl Future<Item=Vec<Subreddit>, Error=FetchError> {
    // Make sure function handles TLS 
    let https = HttpsConnector::new(4).expect("TLS initialization failed");
    let client = Client::builder()
        .build::<_, hyper::Body>(https);
    client
        .get(uri)
        .and_then(|res| {
            // asynchronously concatenate chunks of the body
            res.into_body().concat2()
        })
        .from_err::<FetchError>()
        .and_then(|body| {
            let users = serde_json::from_slice(&body)?; // handles errors using From
            Ok(users)
        })
        .from_err()
}

