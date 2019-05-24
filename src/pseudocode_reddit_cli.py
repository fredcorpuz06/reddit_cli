import sys
import re



current_uri = "https://reddit.com"

def prompt_user_choice(user_choices):
    print()
    print("="*15)
    print("=== Choices ===")
    print("="*15)
    for i, u in enumerate(user_choices):
        print(f"[{i}] {u['prompt']}")
    print("-" * 5)
    return int(input("Index of chosen? "))

def prompt_user_string(prompt):
    return str(input(prompt))

def make_query_json(uri):
    params_start = uri.find("?limit")
    print(params_start)
    if params_start == -1:
        return f"{uri}.json"
    return f"{uri[:params_start]}.json{uri[params_start:]}"

def remove_segment(uri, seg):
    seg_start = uri.find(seg)
    seg_end = seg_start + len(seg)
    return f"{uri[:seg_start]}{uri[seg_end:]}"

def view_submissions(subreddit, n):
    for s in submissions:
        print(s)


def add_before_params(uri, segment):
    params_start = uri.find("?limit=")
    return f"{uri[:params_start]}{segment}{uri[params_start:]}"

def add_as_params(uri, segment):
    return f"{uri}{segment}"

def view_more(uri, segment, n=10):
    view_param_start = uri.find("?limit=")
    view_param_end = uri.find("&")
    limit_param = f"?limit={n}"
    if view_param_end == -1:
        view_param_end = len(uri)
    return f"{uri[:view_param_start]}{limit_param}{uri[view_param_end:]}"
    

def comment_interact_logic(base_uri):
    choice_set = user_choices3 # choosing an action, etc.

    current_uri = remove_segment(base_uri, ".json")
    n_choices = len(choice_set)
    choice = prompt_user_choice(choice_set) 

    if choice in range(0, n_choices-1): 
        print("ACTION: ", choice_set[choice]["prompt"])
        return (0, make_query_json(current_uri))
    elif choice == n_choices: 
        return (-1, current_uri)
    

# In: https://www.reddit.com/r/Fitness/comments/bpiq02.json?limit=5
# https://www.reddit.com/r/Fitness/comments/bpiq02/entsz9a.json?limit=5
# https://www.reddit.com/r/Fitness/comments/bpiq02.json?limit=5&sort=hot
def comments_logic(base_uri):
    choice_set = user_choices2 # choosing a comment, etc.

    current_uri = remove_segment(base_uri, ".json")
    n_choices = len(choice_set)
    choice = prompt_user_choice(choice_set)

    if choice == 0: 
        submission = prompt_user_string("Comment ID? ")
        current_uri = add_before_params(
            current_uri,
            choice_set[choice]['url_segment'].replace("COMMENTID", submission)
        )
        return (1, make_query_json(current_uri)) # print comments
    elif choice in range(1, n_choices-2): # arrange by (hot, new) => reprint comments
        current_uri = add_as_params(current_uri, choice_set[choice]['url_segment'])
        return (0, make_query_json(current_uri))
    elif choice == n_choices-2: # see more submissions
        current_uri = view_more(current_uri, choice_set[choice]['url_segment'], 10)
        return (0, make_query_json(current_uri))
    elif choice == n_choices-1: # back
        return (-1, current_uri)

# In: https://www.reddit.com/r/Fitness.json?limit=5
# Out: https://www.reddit.com/r/Fitness/comments/bpiq02.json?limit=5
# Out: https://reddit.com/r/Fitness/new.json?limit=5
def submissions_logic(base_uri):
    choice_set = user_choices1 # choosing a submission, etc.
    
    current_uri = remove_segment(base_uri, ".json")
    # current_ranking = choice_set[2]['url_segment']
    n_choices = len(choice_set)
    choice = prompt_user_choice(choice_set) 

    if choice == 0: 
        submission = prompt_user_string("Submission ID? ")
        current_uri = add_before_params(
            current_uri,
            choice_set[choice]['url_segment'].replace("SUBMISSIONID", submission)
        )
        return (1, make_query_json(current_uri)) # print comments
    elif choice in range(1, n_choices-2): # params (hot, limit) => reprint submissions
        current_uri = add_before_params(current_uri, choice_set[choice]['url_segment'])
        return (0, make_query_json(current_uri))
    elif choice == n_choices-2: # see more submissions
        current_uri = view_more(current_uri, choice_set[choice]['url_segment'], 10)
        return (0, make_query_json(current_uri))
    elif choice == n_choices-1: # back
        return (-1, current_uri)


# https://www.reddit.com/r/Fitness.json?limit=5
# https://www.reddit.com/r/trendingsubreddits.json
def subreddits_logic(base_uri):
    choice_set = user_choices0 # choosing a subreddit, etc.

    current_uri = base_uri
    n_choices = len(choice_set)
    choice = prompt_user_choice(choice_set) 

    if choice == 0:
        subreddit = prompt_user_string("Subreddit? ")
        params = choice_set[choice]['url_segment'].replace("SUBREDDIT", subreddit)
        current_uri += params
        return (1, make_query_json(current_uri)) 
    elif choice in range(1, n_choices-1): # params: print some subreddits
        temp_uri = current_uri
        temp_uri += choice_set[choice]['url_segment']
        return (0, make_query_json(temp_uri))
    elif choice == n_choices-1: # back
        return (-1, None)



def main():
    # print(user_choices0)
    base_uri = "https://reddit.com"
    while True:
        move_subreddit, query_subreddit = subreddits_logic(base_uri) 
        print("Subreddit Level: ", query_subreddit) # https://www.reddit.com/r/Fitness.json?limit=5
        if move_subreddit == 1:
            while True:
                move_submission, query_submission = submissions_logic(query_subreddit)
                print("Submissions Level:", query_submission) # https://www.reddit.com/r/Fitness/comments/bpiq02.json?limit=5
                if move_submission == 1:
                    while True:
                        level_comments, query_comments = comments_logic(query_submission)
                        print("Comments Level:", query_comments)
                        if level_comments == 1:
                            level_interact, query_interact = comment_interact_logic(query_comments)
                            print("Comment Interact Level:", query_interact)
                            while True:
                                if level_interact == 0:
                                    continue
                                elif level_interact == 1:
                                    break
                        if level_comments == 0:
                            continue
                        if level_comments == -1:
                            break

                if move_submission == 0:
                    continue
                if move_submission == -1:
                    break

        elif move_subreddit == 0:
            continue
        elif move_subreddit == -1:
            break                        
    sys.exit()


    # let my_subreddit: String = choose_subreddit(); // Fitness
    # view_submissions(my_subreddit, 5); // print 5 submissions
    # let submission: String = choose_submission(); // bpqi102
    # view_comments(submission); // print 5 comments
    # // let comment: String = choose_comment(); // entsz9a
    # // view_comment(comment);


submissions = [
    {
        'title': 'sub1',
        'ups': 5
    },
    {
        'title': 'sub2',
        'ups': 5
    }
]


# 1 comment within a submission chosen
user_choices3 = [
    {
        "prompt": "Upvote",
    },
    {
        "prompt": "Downvote",
    },
    {
        "prompt": "Reply",
    },
    {
        "prompt": "BACK",
    },            
]
# 1 submission chosen
# https://www.reddit.com/r/Fitness/comments/bpiq02/entsz9a.json?limit=5
user_choices2 = [
    {
        "prompt": "Choose comment",
        "url_segment": "/COMMENTID",
        "next_prompts": user_choices3,
    },
    {
        "prompt": "Sort by best",
        "url_segment": "&sort=confidence"
    },
    {
        "prompt": "Sort by top",
        "url_segment": "&sort=top"
    },
    {
        "prompt": "Sort by new",
        "url_segment": "&sort=new"
    },
    {
        "prompt": "Sort by controversial",
        "url_segment": "&sort=controversial"
    },    
    {
        "prompt": "Sort by random",
        "url_segment": "&sort=random"
    },
    {
        "prompt": "See more comments",
        "url_segment": "?limit=NUMBER",
    },
    {
        "prompt": "BACK",
        "url_segment": "",
    },   
]
# I subreddit chosen
# https://www.reddit.com/r/Fitness/comments/bpiq02.json?sort=hot&limit=1
user_choices1 = [
    {
        "prompt": "Choose submission",
        "url_segment": "/comments/SUBMISSIONID",
        "next_prompts": user_choices2,
    },
    {
        "prompt": "Rank by hot",
        "url_segment": "/hot",
    },
    {
        "prompt": "Rank by new",
        "url_segment": "/new",
    },
    {
        "prompt": "Rank by controversial",
        "url_segment": "/controversial",
    },
    {
        "prompt": "Rank by random",
        "url_segment": "/random",
    },
    {
        "prompt": "See more submissions",
        "url_segment": "?limit=NUMBER",
    },
    {
        "prompt": "BACK",
        "url_segment": "",
    },    
]
# No input yet
user_choices0 = [
    {
        "prompt": "Choose subreddit",
        "url_segment": "/r/SUBREDDIT?limit=5",
        "next_prompts": user_choices1
    },
    {
        "prompt": "View hot subreddits",
        "url_segment": "/r/trendingsubreddits",
    },
    {
        "prompt": "Exit program",
        "url_segment": "",
    },
]


if __name__ == "__main__":
    main()

