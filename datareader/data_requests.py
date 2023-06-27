# script for querying the database once it is set up


def fetch_tweets(query, count=100):
    tweets = api_twitter.search(query, count=count)
    df = pd.DataFrame(columns=["text", "created_at"])
    for tweet in tweets:
        df = df.append({"text": tweet.text, "created_at": tweet.created_at}, ignore_index=True)
    return df

# Define a query for fetching tweets
query = "forex trading"

# Fetch tweets
tweets_df = fetch_tweets(query, count=100)