// Attempt to write Reddit JSON response to CSV
use std::error::Error;
use std::io;
use std::process;

pub fn writer(records: Vec<&Vec<str>>) -> Result<(), Box<Error>> {
    let mut wtr = csv::Writer::from_writer(io::stdout());

    // When writing records without Serde, the header record is written just
    // like any other record.
    for r in records {
        wtr.write_record(r)?;
    }
    // wtr.write_record(&["city", "region", "country", "population"])?;
    // wtr.write_record(&["Southborough", "MA", "United States", "9686"])?;
    // wtr.write_record(&["Northbridge", "MA", "United States", "14061"])?;
    wtr.flush()?;
    Ok(())
}

pub fn write_to_csv(records: Vec<&Vec<str>>) {
    if let Err(err) = writer(records) {
        println!("error running example: {}", err);
        process::exit(1);
    }
}



// Run future to grab Subreddit from Reddit API + Pretty Print
pub fn csv_subreddit(raw_uri: &str, records: &mut Vec<Vec<String>>) {
    let uri = raw_uri.parse().unwrap();
    let future = fetch_json_single(uri)
        .map(|subreddit| {
            println!("\n");
            println!("   id              Author     # Comments              Submission Title");
            for submission in &subreddit.data.children {
                let id1 = unwrap_mystr2(submission.data.id.as_str());
                let title1 = unwrap_mystr2(submission.data.title.as_str());
                let author1 = unwrap_mystr2(submission.data.author.as_str());
                let n1 = unwrap_mystr2(submission.data.num_comments.as_u64());
                
                records.push(vec![id1, title1, author1, n1])
                // println!("[{id}]{author:>width$}  {n:<width2$}{title}", 
                //     id = id1, title = title1, author = author1, 
                //     n = n1, width = 20, width2 = 10)

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

pub fn unwrap_mystr2<T>(myobj: Option<&T>) -> String {
    match myobj {
        Some(x) => String::from(x),
        None => String::from("'NA'")
    }
}




