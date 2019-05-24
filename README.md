# Rust Reddit CLI

## Project Description
A command line interface for a user to interact with Reddit. Users can ask for popular subreddits, look at the submissions within any subreddit,
and rank comments for each submission. This uses the `hyper`, `hyper_tls` and `serde_json` packages to send and deserialize GET requests from the Reddit API.

The documentation for the Reddit API can be found [here](https://www.reddit.com/dev/api).

* **Subreddit** - r/Fitness
* **Submission** -  Muscles can handle more weight, but joints cannot. What do I do?
* **Comments** 
  * Muscles tissue adapts faster than that of the joints. Don't rush it as this is a recipe for injury (271 points)
  * What is your volume and frequency? Maybe try dialing it back some for a bit. Possibly developing some tendonitis??? (65 points)

## Project Structure
* `main.rs`: Contains all variables describing the URI and looping for user movement
* `moves.rs`:  All functions for user movement/navigation of subreddit, submission, comments handled here. Mutable references to variables declared and maintained in main are used.
* `string_helpers.rs`: Helper functions to manipulate strings, read input from the command line, and read in files
* `get_reddit.rs`: Prints output (subreddits, submissions, comments) when given a URI. Utilizes the `hyper` and `hyper_tls` packages to submit GET requests. Also, utilizes `serde_json` to unpack the JSON responses from the API into structs in Rust.

* `post_reddit.rs` (dead code): Send POST Requests to Reddit API. Code not used because it is incomplete without being able to perform OAuth2 for Reddit API
* `all_choices.json`: All possible user options/paths delineated here.
* `sample_reddit_response/`: Directory of sample JSON responses from the Reddit API
* `pseudocode_reddit_cli.py`: Working Python code for the user navigation and URI creation

## Initial Project Ambitions & Post-Project Reflections
The initial goals for the project were to implement the following feature on the Reddit API.

☑️ View subreddits, submission and comments

❌ Upvote, downvote on specific comments

❌ Post reply to a specific submission and/or comment on users account

In hindsight, I had come to the following realizations: 1) It was a mistake to protype in Python because of the big difference how variables can (should) be used in the 2 languages. The most drastic difference was the mutable references to `String` that is possible in Rust. The creation of URI in my Python code was mostly based on string detection, insertions, deletions. After implementing these operations in Rust, I realized that `String` operations were only possible on mutable references. This required me to scrap most of my code at that point. 2) I should have put more thought into the underlying data structures that I used. In this project I have `enum MoveLevel`, `enum RedditApiCall`, `struct UserChoices`, `structs Subreddit, SubredditData, Submission, etc.`. There is a lot of overlap in the enums I used to navigate the menu. Also, the decoding using `serde_json` was difficult because the responses were deeply nested, which made it necessary to create a long chain; however, I believe this could be done in a cleaner way. 3) I should have been writing tests all throughout. I'd been struggling with parsing through datatypes throughout the entire duration of the project and more testing could have helped with that.

Overall, I had a tedious but rewarding 1st project in Rust. I should have attempted to code smaller projects to get more familiar with the Rust's quirks and tricks before tackling this large project. Now, I feel that I can easily take on a project of similar magnitude and more intuitively implement the advanced Rust features described in the textbook.



## Example Usage
*The example below is one continuous session. User is expected to follow the prompts*

### Getting & Running Files
```
git clone https://github.com/fred_corpuz06/reddit_cli.git
cd reddit_cli
cargo run
```

### Subreddits Level
```
===============
=== Choices ===
===============

[0] Choose subreddit
[1] View hot subreddits
[2] Exit program
-----
Index of chosen? 0
Subreddit? Fitness
Subreddit Level: https://www.reddit.com/r/Fitness.json?limit=5


   id              Author     # Comments              Submission Title
[brdxec]      purplespengler  79        Community Campfire: Managing Sweat, BO, and Sweaty Clothes
[bs12hk]       AutoModerator  964       Daily Simple Questions Thread - May 23, 2019
[bs3d3s]           Spare_toe  106       Pros and cons of linear progression vs whatever my friend is recommending
[bs5hi3]          oceanic231  74        What is the best 5/3/1 template for getting stronger as a late stage beginner?
[bs988h]           Amerine66  44        Cardio
[brpiuq]    MythicalStrength  504       "6 Uncomfortable Thoughts About Exercise Form: Perfect Form is a Newbie Myth" by Charles Staley
[bryf6i]     Nightmare_Tonic  117       Muscles can handle more weight, but joints cannot! What do I do?

===============
=== Choices ===
===============
[0] Choose subreddit
[1] View hot subreddits
[2] Exit program
-----
Index of chosen? 1
Subreddit Level: https://www.reddit.com/r/trendingsubreddits.json?limit=5


   id              Author     # Comments              Submission Title
[brylff]              reddit  13        Trending Subreddits for 2019-05-23: /r/visualization, /r/AliensAmongUs, /r/PeopleWhoWorkAt, /r/Cooking, /r/behindthephoto
[brk9lq]              reddit  6         Trending Subreddits for 2019-05-22: /r/EatCheapAndHealthy, /r/formula1, /r/michaelbaygifs, /r/ChernobylTV, /r/SneakyAnimals
[br5vkw]              reddit  44        Trending Subreddits for 2019-05-21: /r/westworld, /r/asoiaf, /r/gameofthrones, /r/PlayfulAnimals, /r/Undergrounds
[bqqpnn]              reddit  14        Trending Subreddits for 2019-05-20: /r/AccidentalArtGallery, /r/worldcup, /r/Borderlands, /r/casualnintendo, /r/Cyberpunk
[bqdn92]              reddit  9         Trending Subreddits for 2019-05-19: /r/WhatsInThisThing, /r/Dashcam, /r/nonononoyesno, /r/PastAndPresentPics, /r/UnresolvedMysteries

```
### Submission Level
```
===============
=== Choices ===
===============
[0] Choose submission
[1] Rank by hot
[2] Rank by new
[3] Rank by controversial
[4] Rank by random
[5] See more submissions
[6] BACK
-----
Index of chosen? 3
Submissions Level: https://www.reddit.com/r/Fitness/controversial.json?limit=5


   id              Author     # Comments              Submission Title
[brzdlx]          sood571456  24        Cardio twice 7 days a week
[bs3ww5]             WashUPT  10        Supplemental/Assistance Exercise Programming
[bs3abq]         ObiWanQdoba  33        (Beating a dead horse) Morning workouts and the spine
[bs9qfv]         Foxsundance  2         Losing weight but gaining strength?
[bry510]        DankDefusion  2         What size exercise band for glute activation exercises?

===============
=== Choices ===
===============
[0] Choose submission
[1] Rank by hot
[2] Rank by new
[3] Rank by controversial
[4] Rank by random
[5] See more submissions
[6] BACK
-----
Index of chosen? 0
Submission ID? bs3abq
Submissions Level: https://www.reddit.com/r/Fitness/comments/bs3abq.json?limit=5
== SUBMISSION ==
[bs3abq] 33 comments 12 upvotes
Title: (Beating a dead horse) Morning workouts and the spine
I read that morning workouts, specifically heavy spinal load exercises such as squats, deadlifts, rows, etc. need to be avoided right after waking, but I was wondering if 2 hours post waking is sufficient for the spine to be ready for such loads? Say a person wakes up at 4AM and lifts at 6AM, is that enough time for the spine to be in optimal shape, or is it really best to just lift in the evening?

== TOP COMMENTS ==
   id              Author     # Replies              Comment
[eoii2n5]          mattricide  37        thats what warming up is for
[eoijybf]  lordandmasterbator  24        Serious question, why do you think the spine is weaker when you've just woken up? If anything, my back feels stronger in the morning/after working out than it does after sitting at a desk all day.
[eoiqh23]           wistoon33  11        &gt;I read that morning workouts, specifically heavy spinal load exercises such as squats, deadlifts, rows, etc.

Where? Links?
[eoihysr]      milla_highlife  23        Plenty of people work out after waking up with no issue.
[eok3gdd]                'NA'  0         'NA'
```
### Comments Level
```
===============
=== Choices ===
===============
[0] Choose comment
[1] Sort by best
[2] Sort by top
[3] Sort by new
[4] Sort by controversial
[5] Sort by random
[6] See more comments
[7] BACK
-----
Index of chosen? 6
Count? 30
Comments Level: https://www.reddit.com/r/Fitness/comments/bs3abq.json?limit=30
== SUBMISSION ==
[bs3abq] 33 comments 12 upvotes
Title: (Beating a dead horse) Morning workouts and the spine
I read that morning workouts, specifically heavy spinal load exercises such as squats, deadlifts, rows, etc. need to be avoided right after waking, but I was wondering if 2 hours post waking is sufficient for the spine to be ready for such loads? Say a person wakes up at 4AM and lifts at 6AM, is that enough time for the spine to be in optimal shape, or is it really best to just lift in the evening?

== TOP COMMENTS ==
   id              Author     # Replies              Comment
[eoii2n5]          mattricide  38        thats what warming up is for
[eoijybf]  lordandmasterbator  24        Serious question, why do you think the spine is weaker when you've just woken up? If anything, my back feels stronger in the morning/after working out than it does after sitting at a desk all day.
[eoiqh23]           wistoon33  11        &gt;I read that morning workouts, specifically heavy spinal load exercises such as squats, deadlifts, rows, etc.

Where? Links?
[eoihysr]      milla_highlife  22        Plenty of people work out after waking up with no issue.
[eoii4x6]    AerusFlameweaver  21        I wake up at 3:30 and often deadlift at 4. Would love to see some research backing up this nonsense.
[eoiw684]         coach_chief  5         It has to do with your vertebrae essentially swelling over night as a consequence of laying down for an extended period of time. It’s not an inflammatory response they just absorb excess fluid while they’re not under any sort of compression from gravity.

It doesn’t take long for you back to reset. My motto is drink your coffee first, I’m talking 15 minutes tops, a warm up should be long enough to reduce any risk associated with this.
[eoinyri]            Ohmy2383  4         My deadlift days are 30 minutes after waking, DeFranco’s Limber 11, some back extensions and some really light warm up sets &amp; never once had an issue
[eoisd0d]          Zerocoolx1  3         The main problem with early morning workouts is people not warming up properly. If you do that right you'll be fine.
[eoisfg5]          ibexlifter  3         My squat always feel stronger in my morning session

I’m interested in their reasoning as to why the spine wouldn’t be ready to handle loads first thing in the morning.
[eoiuc2e]               CBR85  3         I get up at 4:20 and am in the gym doing Bench, DL and Squats by 4:40. Lift when it fits into your schedule.
[eoj3y85]      CorneliusNepos  3         &gt; I read that morning workouts, specifically heavy spinal load exercises such as squats, deadlifts, rows, etc. need to be avoided right after waking,

I guess I'm fucked then since I wake up at 5am and have the barbell on my back by 5:30 at the latest. I've been doing this for more than three years now.
[eok3gdd]            MEINCOMP  3         I hurt my back deadlifting during 6am workouts in college- needed surgery, then another surgery 3 years later. The reason it happened: it was 6am on a Friday, tired, bad form, didn’t care. Went to lift up my warm up set and popped a disk out. No more squats, deadlifts, hang cleans, etc for me...ever. And that’s ok.
[eoitdfp]      isthisallforme  2         Can you provide a source? This sounds like BS.
[eojdujd]   thisesmeaningless  2         Where did you read this? I've never heard anything like this and have lifted in the morning many times with no issues.
[eoiiamk]             buds510  1         Do a whole body joint mobility daily and you should be ok with good form and adding weights slowly
[eoiuqht]           Nutmagnus  1         Got any links, my guy?
[eoja0f3]     Reading_is_Cool  1         And I'm over here just finally finishing planning my new "wake up at 5:00am hit gym at 5:30am" morning routine consisting of deadlifts, barbell rows, squats, and overhead presses [like ] (https://imgur.com/gallery/WRuZAlY)
[eojhrwr]     worksucksiknow1  1         Personally if i do a morning workout i need like 20-30 minutes to warm up and I just dont have the will to wake up at 4 to do that
[eoj1002]                'NA'  0         'NA'

===============
=== Choices ===
===============
[0] Choose comment
[1] Sort by best
[2] Sort by top
[3] Sort by new
[4] Sort by controversial
[5] Sort by random
[6] See more comments
[7] BACK
-----
Index of chosen? 0
Comment ID? eoii2n5
Comments Level: https://www.reddit.com/r/Fitness/comments/bs3abq/eoii2n5.json?limit=30
== COMMENTS ==
   id              Author     # Replies              Comment
[eoii2n5]          mattricide  38        thats what warming up is for
[eoijybf]  lordandmasterbator  23        Serious question, why do you think the spine is weaker when you've just woken up? If anything, my back feels stronger in the morning/after working out than it does after sitting at a desk all day.
[eoiqh23]           wistoon33  10        &gt;I read that morning workouts, specifically heavy spinal load exercises such as squats, deadlifts, rows, etc.

Where? Links?
[eoihysr]      milla_highlife  22        Plenty of people work out after waking up with no issue.
[eoii4x6]    AerusFlameweaver  22        I wake up at 3:30 and often deadlift at 4. Would love to see some research backing up this nonsense.
[eoiw684]         coach_chief  6         It has to do with your vertebrae essentially swelling over night as a consequence of laying down for an extended period of time. It’s not an inflammatory response they just absorb excess fluid while they’re not under any sort of compression from gravity.

It doesn’t take long for you back to reset. My motto is drink your coffee first, I’m talking 15 minutes tops, a warm up should be long enough to reduce any risk associated with this.
[eoinyri]            Ohmy2383  4         My deadlift days are 30 minutes after waking, DeFranco’s Limber 11, some back extensions and some really light warm up sets &amp; never once had an issue
[eoisd0d]          Zerocoolx1  3         The main problem with early morning workouts is people not warming up properly. If you do that right you'll be fine.
[eoisfg5]          ibexlifter  3         My squat always feel stronger in my morning session

I’m interested in their reasoning as to why the spine wouldn’t be ready to handle loads first thing in the morning.
[eoiuc2e]               CBR85  3         I get up at 4:20 and am in the gym doing Bench, DL and Squats by 4:40. Lift when it fits into your schedule.
[eoj3y85]      CorneliusNepos  3         &gt; I read that morning workouts, specifically heavy spinal load exercises such as squats, deadlifts, rows, etc. need to be avoided right after waking,

I guess I'm fucked then since I wake up at 5am and have the barbell on my back by 5:30 at the latest. I've been doing this for more than three years now.
[eok3gdd]            MEINCOMP  3         I hurt my back deadlifting during 6am workouts in college- needed surgery, then another surgery 3 years later. The reason it happened: it was 6am on a Friday, tired, bad form, didn’t care. Went to lift up my warm up set and popped a disk out. No more squats, deadlifts, hang cleans, etc for me...ever. And that’s ok.
[eoitdfp]      isthisallforme  2         Can you provide a source? This sounds like BS.
[eojdujd]   thisesmeaningless  2         Where did you read this? I've never heard anything like this and have lifted in the morning many times with no issues.
[eoiiamk]             buds510  1         Do a whole body joint mobility daily and you should be ok with good form and adding weights slowly
[eoiuqht]           Nutmagnus  1         Got any links, my guy?
[eoja0f3]     Reading_is_Cool  1         And I'm over here just finally finishing planning my new "wake up at 5:00am hit gym at 5:30am" morning routine consisting of deadlifts, barbell rows, squats, and overhead presses [like ] (https://imgur.com/gallery/WRuZAlY)
[eojhrwr]     worksucksiknow1  1         Personally if i do a morning workout i need like 20-30 minutes to warm up and I just dont have the will to wake up at 4 to do that
[eoj1002]                'NA'  0         'NA'
```
### Comment Interaction Level
```
===============
=== Choices ===
===============
[0] Upvote
[1] Downvote
[2] Reply
[3] BACK
-----
Index of chosen? 0
These features are not yet implemented because they require OAuth2 to make POST requests to the Reddit API
```

## Contributors
* Frederick Corpuz, student at Wesleyan University '20, designed and built the Reddit CLI in Rust