# `xyz.taluslabs.social.twitter.get-tweet@1`

Standard Nexus Tool that retrieves a single tweet from the Twitter API. Twitter api [reference](https://developer.twitter.com/en/docs/twitter-api/tweets/lookup/api-reference/get-tweets-id)

## Input

**`bearer_token`: [`String`]**

The bearer token for the user's Twitter account.

**`tweet_id`: [`String`]**

The ID of the tweet to retrieve.

## Output Variants & Ports

**`ok`**

The tweet was retrieved successfully.

- **`ok.id`: [`String`]** - The tweet's unique identifier
- **`ok.text`: [`String`]** - The tweet's content text
- **`ok.author_id`: [`Option<String>`]** - The ID of the tweet's author
- **`ok.created_at`: [`Option<String>`]** - The timestamp when the tweet was created
- **`ok.username`: [`Option<String>`]** - The username of the tweet's author
- **`ok.attachments`: [`Option<Attachments>`]** - Media and polls attached to the tweet
- **`ok.community_id`: [`Option<String>`]** - Community ID if the tweet belongs to a community
- **`ok.context_annotations`: [`Option<Vec<ContextAnnotation>>`]** - Annotations about the tweet content
- **`ok.conversation_id`: [`Option<String>`]** - ID of the conversation this tweet belongs to
- **`ok.edit_controls`: [`Option<EditControls>`]** - Controls for editing the tweet
- **`ok.edit_history_tweet_ids`: [`Option<Vec<String>>`]** - IDs of tweets in the edit history
- **`ok.entities`: [`Option<Entities>`]** - Entities in the tweet (hashtags, mentions, URLs)
- **`ok.geo`: [`Option<Geo>`]** - Geographic information
- **`ok.in_reply_to_user_id`: [`Option<String>`]** - ID of the user being replied to
- **`ok.lang`: [`Option<String>`]** - Language of the tweet
- **`ok.non_public_metrics`: [`Option<NonPublicMetrics>`]** - Private metrics about the tweet
- **`ok.note_tweet`: [`Option<NoteTweet>`]** - Extended note content
- **`ok.organic_metrics`: [`Option<OrganicMetrics>`]** - Organic engagement metrics
- **`ok.possibly_sensitive`: [`Option<bool>`]** - Whether the tweet might contain sensitive content
- **`ok.promoted_metrics`: [`Option<PromotedMetrics>`]** - Metrics from promoted content
- **`ok.public_metrics`: [`Option<PublicMetrics>`]** - Public engagement metrics (likes, retweets, etc.)
- **`ok.referenced_tweets`: [`Option<Vec<ReferencedTweet>>`]** - Tweets referenced by this tweet
- **`ok.reply_settings`: [`Option<String>`]** - Who can reply to this tweet
- **`ok.scopes`: [`Option<Scopes>`]** - Visibility scopes
- **`ok.source`: [`Option<String>`]** - Source of the tweet (client application)
- **`ok.withheld`: [`Option<Withheld>`]** - Withholding information
- **`ok.includes`: [`Option<Includes>`]** - Additional entities related to the tweet:
  - `media`: Images and videos
  - `places`: Geographic locations
  - `polls`: Twitter polls
  - `topics`: Related topics
  - `tweets`: Referenced tweets
  - `users`: Mentioned users
- **`ok.meta`: [`Option<Meta>`]** - Metadata about the tweet request:
  - `newest_id`: Newest tweet ID in a collection
  - `next_token`: Pagination token for next results
  - `oldest_id`: Oldest tweet ID in a collection
  - `previous_token`: Pagination token for previous results
  - `result_count`: Number of results returned

**`err`**

The tweet was not retrieved due to an error.

- **`err.reason`: [`String`]** - A detailed error message describing what went wrong
- **`err.kind`: [`TwitterErrorKind`]** - The type of error that occurred. Possible values:
  - `network` - A network-related error occurred when connecting to Twitter
  - `connection` - Could not establish a connection to Twitter
  - `timeout` - The request to Twitter timed out
  - `parse` - Failed to parse Twitter's response
  - `auth` - Authentication or authorization error
  - `not_found` - The requested tweet or resource was not found
  - `rate_limit` - Twitter's rate limit was exceeded
  - `server` - An error occurred on Twitter's servers
  - `forbidden` - The request was forbidden
  - `api` - An API-specific error occurred
  - `unknown` - An unexpected error occurred
- **`err.status_code`: [`Option<u16>`]** - The HTTP status code returned by Twitter, if available. Common codes include:
  - `401` - Unauthorized (authentication error)
  - `403` - Forbidden
  - `404` - Not Found
  - `429` - Too Many Requests (rate limit exceeded)
  - `5xx` - Server errors

---

# `xyz.taluslabs.social.twitter.get-user-tweets@1`

Standard Nexus Tool that retrieves tweets from a user's Twitter account. Twitter api [reference](https://developer.twitter.com/en/docs/twitter-api/tweets/timelines/api-reference/get-users-id-tweets)

## Input

**`bearer_token`: [`String`]**

The bearer token for the user's Twitter account.

**`user_id`: [`String`]**

The ID of the User to retrieve tweets from.

_opt_ **`since_id`: [`Option<String>`]** _default_: [`None`]

The minimum Post ID to be included in the result set. Takes precedence over start_time if both are specified.

_opt_ **`until_id`: [`Option<String>`]** _default_: [`None`]

The maximum Post ID to be included in the result set. Takes precedence over end_time if both are specified.

_opt_ **`exclude`: [`Option<Vec<ExcludeField>>`]** _default_: [`None`]

The set of entities to exclude (e.g. 'replies' or 'retweets').

_opt_ **`max_results`: [`Option<i32>`]** _default_: [`None`]

The maximum number of results to retrieve (range: 5-100).

_opt_ **`pagination_token`: [`Option<String>`]** _default_: [`None`]

Used to get the next 'page' of results.

_opt_ **`start_time`: [`Option<String>`]** _default_: [`None`]

The earliest UTC timestamp (YYYY-MM-DDTHH:mm:ssZ) from which the Posts will be provided.

_opt_ **`end_time`: [`Option<String>`]** _default_: [`None`]

The latest UTC timestamp (YYYY-MM-DDTHH:mm:ssZ) to which the Posts will be provided.

_opt_ **`tweet_fields`: [`Option<Vec<TweetField>>`]** _default_: [`None`]

A list of Tweet fields to display.

_opt_ **`expansions`: [`Option<Vec<ExpansionField>>`]** _default_: [`None`]

A list of fields to expand.

_opt_ **`media_fields`: [`Option<Vec<MediaField>>`]** _default_: [`None`]

A list of Media fields to display.

_opt_ **`poll_fields`: [`Option<Vec<PollField>>`]** _default_: [`None`]

A list of Poll fields to display.

_opt_ **`user_fields`: [`Option<Vec<UserField>>`]** _default_: [`None`]

A list of User fields to display.

_opt_ **`place_fields`: [`Option<Vec<PlaceField>>`]** _default_: [`None`]

A list of Place fields to display.

## Output Variants & Ports

**`ok`**

The tweets were retrieved successfully.

- **`ok.data`: [`Vec<Tweet>`]** - The collection of tweets from the user's timeline.
- **`ok.includes`: [`Option<Includes>`]** - Additional data included in the response (users, media, polls, etc.)
- **`ok.meta`: [`Option<Meta>`]** - Metadata about the response (result_count, newest_id, oldest_id, next_token, etc.)

**`err`**

The tweets could not be retrieved due to an error.

- **`err.reason`: [`String`]** - A detailed error message describing what went wrong
- **`err.kind`: [`TwitterErrorKind`]** - The type of error that occurred. Possible values:
  - `network` - A network-related error occurred when connecting to Twitter
  - `connection` - Could not establish a connection to Twitter
  - `timeout` - The request to Twitter timed out
  - `parse` - Failed to parse Twitter's response
  - `auth` - Authentication or authorization error
  - `not_found` - The requested tweet or resource was not found
  - `rate_limit` - Twitter's rate limit was exceeded
  - `server` - An error occurred on Twitter's servers
  - `forbidden` - The request was forbidden
  - `api` - An API-specific error occurred
  - `unknown` - An unexpected error occurred
- **`err.status_code`: [`Option<u16>`]** - The HTTP status code returned by Twitter, if available. Common codes include:
  - `401` - Unauthorized (authentication error)
  - `403` - Forbidden
  - `404` - Not Found
  - `429` - Too Many Requests (rate limit exceeded)
  - `5xx` - Server errors

---

# `xyz.taluslabs.social.twitter.get-recent-search-tweets@1`

Standard Nexus Tool that retrieves tweets from the recent search API. Twitter api [reference](https:/developer.twitter.com/en/docs/twitter-api/tweets/search/api-reference/get-tweets-search-recent)

## Input

**`bearer_token`: [`String`]**

The bearer token for the user's Twitter account.

**`query`: [`String`]**

Search query for matching tweets.

_opt_ **`start_time`: [`Option<String>`]** _default_: [`None`]

The oldest UTC timestamp from which the tweets will be provided (YYYY-MM-DDTHH:mm:ssZ).

_opt_ **`end_time`: [`Option<String>`]** _default_: [`None`]

The newest UTC timestamp to which the tweets will be provided (YYYY-MM-DDTHH:mm:ssZ).

_opt_ **`since_id`: [`Option<String>`]** _default_: [`None`]

Returns results with a tweet ID greater than (more recent than) the specified ID.

_opt_ **`until_id`: [`Option<String>`]** _default_: [`None`]

Returns results with a tweet ID less than (older than) the specified ID.

_opt_ **`max_results`: [`Option<i32>`]** _default_: [`None`]

The maximum number of search results to be returned (between 10 and 100).

_opt_ **`next_token`: [`Option<String>`]** _default_: [`None`]

Token for pagination to get the next page of results.

_opt_ **`pagination_token`: [`Option<String>`]** _default_: [`None`]

Alternative parameter for pagination (same as next_token).

_opt_ **`sort_order`: [`Option<SortOrder>`]** _default_: [`None`]

Order in which to return results. Available values:

- `recency`: Return results in order of newest to oldest
- `relevancy`: Return results in order of relevance score

_opt_ **`tweet_fields`: [`Option<Vec<TweetField>>`]** _default_: [`None`]

A list of Tweet fields to display.

_opt_ **`expansions`: [`Option<Vec<ExpansionField>>`]** _default_: [`None`]

A list of fields to expand.

_opt_ **`media_fields`: [`Option<Vec<MediaField>>`]** _default_: [`None`]

A list of Media fields to display.

_opt_ **`poll_fields`: [`Option<Vec<PollField>>`]** _default_: [`None`]

A list of Poll fields to display.

_opt_ **`user_fields`: [`Option<Vec<UserField>>`]** _default_: [`None`]

A list of User fields to display.

_opt_ **`place_fields`: [`Option<Vec<PlaceField>>`]** _default_: [`None`]

A list of Place fields to display.

## Output Variants & Ports

**`ok`**

The search results were retrieved successfully.

- **`ok.data`: [`Vec<Tweet>`]** - The collection of tweets matching the search query.
- **`ok.includes`: [`Option<Includes>`]** - Additional data included in the response (users, media, polls, etc.)
- **`ok.meta`: [`Option<Meta>`]** - Metadata about the response (result_count, newest_id, oldest_id, next_token, etc.)

**`err`**

The search results could not be retrieved due to an error.

- **`err.kind`: [`TwitterErrorKind`]** - The type of error that occurred. Possible values:

  - `network` - A network-related error occurred when connecting to Twitter
  - `connection` - Could not establish a connection to Twitter
  - `timeout` - The request to Twitter timed out
  - `parse` - Failed to parse Twitter's response
  - `auth` - Authentication or authorization error
  - `not_found` - The requested tweet or resource was not found
  - `rate_limit` - Twitter's rate limit was exceeded
  - `server` - An error occurred on Twitter's servers
  - `forbidden` - The request was forbidden
  - `api` - An API-specific error occurred
  - `unknown` - An unexpected error occurred

- **`err.reason`: [`String`]** - The reason for the error. This could be:

  - Twitter API error (e.g., "Twitter API returned errors: Invalid Request: One or more parameters to your request was invalid.")
  - Validation error (e.g., "Validation error: max_results must be between 10 and 100")
  - Timestamp format error (e.g., "Invalid start_time format. Expected format: YYYY-MM-DDTHH:mm:ssZ")
  - Sort order error (e.g., "sort_order must be either 'recency' or 'relevancy'")
  - Network error (e.g., "Network error: network error: Connection refused")
  - Response parsing error (e.g., "Response parsing error: expected value at line 1 column 1")
  - Status code error (e.g., "Twitter API status error: 429 Too Many Requests")
  - "No search results found" when the API response doesn't contain tweet data
  - Unauthorized error (e.g., "Unauthorized")
  - Other error types handled by the centralized error handling mechanism

- **`err.status_code`: [`Option<u16>`]** - The HTTP status code returned by Twitter, if available. Common codes include:
  - `401` - Unauthorized (authentication error)
  - `403` - Forbidden
  - `404` - Not Found
  - `429` - Too Many Requests (rate limit exceeded)
  - `5xx` - Server errors

---

# `xyz.taluslabs.social.twitter.post-tweet@1`

Standard Nexus Tool that posts a content to Twitter.
Twitter api [reference](https://docs.x.com/x-api/tweets/post-tweet)

## Input

**Authentication Parameters**

The following authentication parameters are provided as part of the TwitterAuth structure:

- **`consumer_key`: [`String`]** - Twitter API application's Consumer Key
- **`consumer_secret_key`: [`String`]** - Twitter API application's Consumer Secret Key
- **`access_token`: [`String`]** - Access Token for user's Twitter account
- **`access_token_secret`: [`String`]** - Access Token Secret for user's Twitter account

**Additional Parameters**

**`text`: [`String`]**

The text content of the tweet.

_opt_ **`card_uri`: [`Option<String>`]** _default_: [`None`]

Card URI for rich media preview. This is mutually exclusive from Quote Tweet ID, Poll, Media, and Direct Message Deep Link.

_opt_ **`community_id`: [`Option<String>`]** _default_: [`None`]

Community ID for community-specific tweets.

_opt_ **`direct_message_deep_link`: [`Option<String>`]** _default_: [`None`]

Direct message deep link. This is mutually exclusive from Quote Tweet ID, Poll, Media, and Card URI.

_opt_ **`for_super_followers_only`: [`Option<bool>`]** _default_: [`None`]

Whether the tweet is for super followers only.

_opt_ **`geo`: [`Option<GeoInfo>`]** _default_: [`None`]

Geo location information containing:

- `place_id`: Place ID for the location

_opt_ **`media`: [`Option<MediaInfo>`]** _default_: [`None`]

Media information containing:

- `media_ids`: List of media IDs to attach (required)
- `tagged_user_ids`: List of user IDs to tag in the media (optional)

This is mutually exclusive from Quote Tweet ID, Poll, and Card URI.

_opt_ **`nullcast`: [`Option<bool>`]** _default_: [`None`]

Whether the tweet should be nullcast (promoted-only). Nullcasted tweets do not appear in the public timeline and are not served to followers.

_opt_ **`poll`: [`Option<PollInfo>`]** _default_: [`None`]

Poll information containing:

- `duration_minutes`: Duration of the poll in minutes (required, range: 5-10080)
- `options`: List of poll options (required, 2-4 options)
- `reply_settings`: Reply settings for the poll (optional)

This is mutually exclusive from Quote Tweet ID, Media, and Card URI.

_opt_ **`quote_tweet_id`: [`Option<String>`]** _default_: [`None`]

ID of the tweet to quote. This is mutually exclusive from Poll, Media, and Card URI.

_opt_ **`reply`: [`Option<ReplyInfo>`]** _default_: [`None`]

Reply information containing:

- `in_reply_to_tweet_id`: ID of the tweet to reply to (required)
- `exclude_reply_user_ids`: List of user IDs to exclude from replies (optional)

_opt_ **`reply_settings`: [`Option<ReplySettings>`]** _default_: [`None`]

Reply settings for the tweet. Can be one of:

- `Following`: Only followers can reply
- `MentionedUsers`: Only mentioned users can reply
- `Subscribers`: Only subscribers can reply

## Output Variants & Ports

**`ok`**

The tweet was posted successfully.

- **`ok.id`: [`String`]** - The tweet's unique identifier
- **`ok.edit_history_tweet_ids`: [`Vec<String>`]** - List of tweet IDs in the edit history
- **`ok.text`: [`String`]** - The actual content of the tweet

**`err`**

The tweet posting failed.

- **`err.reason`: [`String`]** - The reason for the error. This could be:
  - Twitter API error status (Code/Message format)
  - Twitter API error details (Detail/Status/Title format)
  - Rate limit exceeded (Status: 429)
  - Unauthorized error
  - Invalid JSON response
  - Failed to read Twitter API response
  - Failed to send request to Twitter API
  - Mutually exclusive parameters error (e.g., using both poll and media)
  - "You are not permitted to create an exclusive Tweet" error (when for_super_followers_only is true)

---

# `xyz.taluslabs.social.twitter.delete-tweet@1`

Standard Nexus Tool that deletes a tweet.
Twitter api [reference](https://docs.x.com/x-api/posts/post-delete-by-post-id#post-delete-by-post-id)

## Input

**Authentication Parameters**

The following authentication parameters are provided as part of the TwitterAuth structure:

- **`consumer_key`: [`String`]** - Twitter API application's Consumer Key
- **`consumer_secret_key`: [`String`]** - Twitter API application's Consumer Secret Key
- **`access_token`: [`String`]** - Access Token for user's Twitter account
- **`access_token_secret`: [`String`]** - Access Token Secret for user's Twitter account

**Additional Parameters**

**`tweet_id`: [`String`]**

The ID of the tweet to delete.

## Output Variants & Ports

**`ok`**

The tweet was successfully deleted.

- **`ok.deleted`: [`bool`]** - Confirmation that the tweet was deleted (true)

**`err`**

The tweets could not be deleted due to an error.

- **`err.reason`: [`String`]** - A detailed error message describing what went wrong
- **`err.kind`: [`TwitterErrorKind`]** - The type of error that occurred. Possible
  values:
  - `network` - A network-related error occurred when connecting to Twitter
  - `connection` - Could not establish a connection to Twitter
  - `timeout` - The request to Twitter timed out
  - `parse` - Failed to parse Twitter's response
  - `auth` - Authentication or authorization error
  - `not_found` - The requested tweet or resource was not found
  - `rate_limit` - Twitter's rate limit was exceeded
  - `server` - An error occurred on Twitter's servers
  - `forbidden` - The request was forbidden
  - `api` - An API-specific error occurred
  - `unknown` - An unexpected error occurred
- **`err.status_code`: [`Option<u16>`]** - The HTTP status code returned by Twitter, if available. Common codes include:
  - `401` - Unauthorized (authentication error)
  - `403` - Forbidden
  - `404` - Not Found
  - `429` - Too Many Requests (rate limit exceeded)
  - `5xx` - Server errors

---

# `xyz.taluslabs.social.twitter.like-tweet@1`

Standard Nexus Tool that allows a user to like a specific tweet.
Twitter api [reference](https://docs.x.com/x-api/posts/causes-the-user-in-the-path-to-like-the-specified-post)

## Input

**Authentication Parameters**

The following authentication parameters are provided as part of the TwitterAuth structure:

- **`consumer_key`: [`String`]** - Twitter API application's Consumer Key
- **`consumer_secret_key`: [`String`]** - Twitter API application's Consumer Secret Key
- **`access_token`: [`String`]** - Access Token for user's Twitter account
- **`access_token_secret`: [`String`]** - Access Token Secret for user's Twitter account

**Additional Parameters**

**`user_id`: [`String`]**

The ID of the authenticated user who will like the tweet.

**`tweet_id`: [`String`]**

The ID of the tweet to like.

## Output Variants & Ports

**`ok`**

The tweet was successfully liked.

- **`ok.tweet_id`: [`String`]** - The ID of the tweet that was liked
- **`ok.liked`: [`bool`]** - Confirmation that the tweet was liked (true)

**`err`**

The like operation failed.

- **`err.reason`: [`String`]** - The reason for the error. This could be:
  - Twitter API error status (Code/Message format)
  - Twitter API error details (Detail/Status/Title format)
  - "You have already liked this Tweet" error
  - Unauthorized error
  - Invalid JSON response
  - Failed to read Twitter API response
  - Failed to send like request to Twitter API

---

# `xyz.taluslabs.social.twitter.unlike-tweet@1`

Standard Nexus Tool that unlikes a tweet.
Twitter api [reference](https://docs.x.com/x-api/posts/causes-the-user-in-the-path-to-unlike-the-specified-post#causes-the-user-in-the-path-to-unlike-the-specified-post)

## Input

**Authentication Parameters**

The following authentication parameters are provided as part of the TwitterAuth structure:

- **`consumer_key`: [`String`]** - Twitter API application's Consumer Key
- **`consumer_secret_key`: [`String`]** - Twitter API application's Consumer Secret Key
- **`access_token`: [`String`]** - Access Token for user's Twitter account
- **`access_token_secret`: [`String`]** - Access Token Secret for user's Twitter account

**Additional Parameters**

**`user_id`: [`String`]**

The ID of the authenticated user who will unlike the tweet.

**`tweet_id`: [`String`]**

The ID of the tweet to unlike.

## Output Variants & Ports

**`ok`**

The tweet was successfully unliked.

- **`ok.unliked`: [`bool`]** - Confirmation that the tweet was unliked

**`err`**

The unlike operation failed.

- **`err.reason`: [`String`]** - The reason for the error. This could be:
  - Twitter API error status (Code/Message format)
  - Twitter API error details (Detail/Status/Title format)
  - Unauthorized error
  - Invalid JSON response
  - Failed to read Twitter API response
  - Failed to send unlike request to Twitter API
  - Unexpected response format from Twitter API
  - Twitter API indicated the tweet was already unliked

---

# `xyz.taluslabs.social.twitter.get-mentioned-tweets@1`

Standard Nexus Tool that retrieves tweets mentioning a specific user.
Twitter api [reference](https://docs.x.com/x-api/posts/user-mention-timeline-by-user-id)

## Input

**`bearer_token`: [`String`]**

The bearer token for the user's Twitter account.

**`user_id`: [`String`]**

The ID of the User to lookup for mentions.

_opt_ **`since_id`: [`Option<String>`]** _default_: [`None`]

The minimum Post ID to be included in the result set. Takes precedence over start_time if both are specified.

_opt_ **`until_id`: [`Option<String>`]** _default_: [`None`]

The maximum Post ID to be included in the result set. Takes precedence over end_time if both are specified.

_opt_ **`max_results`: [`Option<i32>`]** _default_: [`None`]

The maximum number of results to retrieve (range: 5-100).

_opt_ **`pagination_token`: [`Option<String>`]** _default_: [`None`]

Used to get the next 'page' of results.

_opt_ **`start_time`: [`Option<String>`]** _default_: [`None`]

The earliest UTC timestamp (YYYY-MM-DDTHH:mm:ssZ) from which the Posts will be provided.

_opt_ **`end_time`: [`Option<String>`]** _default_: [`None`]

The latest UTC timestamp (YYYY-MM-DDTHH:mm:ssZ) to which the Posts will be provided.

_opt_ **`tweet_fields`: [`Option<Vec<TweetField>>`]** _default_: [`None`]

A list of Tweet fields to display.

_opt_ **`expansions`: [`Option<Vec<ExpansionField>>`]** _default_: [`None`]

A list of fields to expand.

_opt_ **`media_fields`: [`Option<Vec<MediaField>>`]** _default_: [`None`]

A list of Media fields to display.

_opt_ **`poll_fields`: [`Option<Vec<PollField>>`]** _default_: [`None`]

A list of Poll fields to display.

_opt_ **`user_fields`: [`Option<Vec<UserField>>`]** _default_: [`None`]

A list of User fields to display.

_opt_ **`place_fields`: [`Option<Vec<PlaceField>>`]** _default_: [`None`]

A list of Place fields to display.

## Output Variants & Ports

**`ok`**

The mentioned tweets were retrieved successfully.

- **`ok.data`: [`Vec<Tweet>`]** - The collection of tweets mentioning the specified user.
- **`ok.includes`: [`Option<Includes>`]** - Additional data included in the response (users, media, polls, etc.)
- **`ok.meta`: [`Option<Meta>`]** - Metadata about the response (result_count, newest_id, oldest_id, next_token, etc.)

**`err`**

The tweet mentions retrieval failed.

- **`err.reason`: [`String`]** - A detailed error message describing what went wrong
- **`err.kind`: [`TwitterErrorKind`]** - The type of error that occurred. Possible values:
  - `network` - A network-related error occurred when connecting to Twitter
  - `connection` - Could not establish a connection to Twitter
  - `timeout` - The request to Twitter timed out
  - `parse` - Failed to parse Twitter's response
  - `auth` - Authentication or authorization error
  - `not_found` - The requested tweet or resource was not found
  - `rate_limit` - Twitter's rate limit was exceeded
  - `server` - An error occurred on Twitter's servers
  - `forbidden` - The request was forbidden
  - `api` - An API-specific error occurred
  - `unknown` - An unexpected error occurred
- **`err.status_code`: [`Option<u16>`]** - The HTTP status code returned by Twitter, if available. Common codes include:
  - `401` - Unauthorized (authentication error)
  - `403` - Forbidden
  - `404` - Not Found
  - `429` - Too Many Requests (rate limit exceeded)
  - `5xx` - Server errors

---

# `xyz.taluslabs.social.twitter.get-tweets@1`

Standard Nexus Tool that retrieves multiple tweets by their IDs from the Twitter API. Twitter api [reference](https://developer.twitter.com/en/docs/twitter-api/tweets/lookup/api-reference/get-tweets)

## Input

**`bearer_token`: [`String`]**

The bearer token for the user's Twitter account.

**`ids`: [`Vec<String>`]**

A list of Tweet IDs to retrieve. Up to 100 are allowed in a single request.

_opt_ **`tweet_fields`: [`Option<Vec<String>>`]** _default_: [`None`]

A list of Tweet fields to display.

_opt_ **`expansions`: [`Option<Vec<String>>`]** _default_: [`None`]

A list of fields to expand.

_opt_ **`media_fields`: [`Option<Vec<String>>`]** _default_: [`None`]

A list of Media fields to display.

_opt_ **`poll_fields`: [`Option<Vec<String>>`]** _default_: [`None`]

A list of Poll fields to display.

_opt_ **`user_fields`: [`Option<Vec<String>>`]** _default_: [`None`]

A list of User fields to display.

_opt_ **`place_fields`: [`Option<Vec<String>>`]** _default_: [`None`]

A list of Place fields to display.

## Output Variants & Ports

**`ok`**

The tweets were retrieved successfully.

- **`ok.data`: [`Vec<Tweet>`]** - The collection of retrieved tweets.
- **`ok.includes`: [`Option<Includes>`]** - Additional data included in the response (users, media, polls, etc.)
- **`ok.meta`: [`Option<Meta>`]** - Metadata about the response.

**`err`**

The tweets could not be retrieved due to an error.

- **`err.kind`: [`TwitterErrorKind`]** - The type of error that occurred. Possible values:

  - `network` - A network-related error occurred when connecting to Twitter
  - `connection` - Could not establish a connection to Twitter
  - `timeout` - The request to Twitter timed out
  - `parse` - Failed to parse Twitter's response
  - `auth` - Authentication or authorization error
  - `not_found` - The requested tweet or resource was not found
  - `rate_limit` - Twitter's rate limit was exceeded
  - `server` - An error occurred on Twitter's servers
  - `forbidden` - The request was forbidden
  - `api` - An API-specific error occurred
  - `unknown` - An unexpected error occurred

- **`err.reason`: [`String`]** - The reason for the error. This could be:

  - Twitter API error (e.g., "Twitter API returned errors: Not Found Error: Could not find tweet with id: [1346889436626259969].")
  - Network error (e.g., "Network error: network error: Connection refused")
  - Response parsing error (e.g., "Response parsing error: expected value at line 1 column 1")
  - Status code error (e.g., "Twitter API status error: 429 Too Many Requests")
  - "No tweet data or errors found in the response" when the API response is empty
  - Unauthorized error (e.g., "Unauthorized")
  - Other error types handled by the centralized error handling mechanism

- **`err.status_code`: [`Option<u16>`]** - The HTTP status code returned by Twitter, if available. Common codes include:
  - `401` - Unauthorized (authentication error)
  - `403` - Forbidden
  - `404` - Not Found
  - `429` - Too Many Requests (rate limit exceeded)
  - `5xx` - Server errors

---

# `xyz.taluslabs.social.twitter.undo-retweet-tweet@1`

Standard Nexus Tool that undoes a retweet.
Twitter api [reference](https://docs.x.com/x-api/posts/causes-the-user-in-the-path-to-unretweet-the-specified-post)

## Input

**Authentication Parameters**

The following authentication parameters are provided as part of the TwitterAuth structure:

- **`consumer_key`: [`String`]** - Twitter API application's Consumer Key
- **`consumer_secret_key`: [`String`]** - Twitter API application's Consumer Secret Key
- **`access_token`: [`String`]** - Access Token for user's Twitter account
- **`access_token_secret`: [`String`]** - Access Token Secret for user's Twitter account

**Additional Parameters**

**`user_id`: [`String`]**

The ID of the authenticated user who will undo the retweet.

**`tweet_id`: [`String`]**

The ID of the tweet to undo retweet.

## Output Variants & Ports

**`ok`**

The retweet was successfully undone.

- **`ok.retweeted`: [`bool`]** - Confirmation that the retweet was undone (false)

**`err`**

The undo retweet operation failed.

- **`err.reason`: [`String`]** - The reason for the error. This could be:
  - Twitter API error status (Code/Message format)
  - Twitter API error details (Detail/Status/Title format)
  - Unauthorized error
  - Invalid JSON response
  - Failed to read Twitter API response
  - Failed to send undo retweet request to Twitter API
  - Unexpected response format from Twitter API
  - Twitter API indicated the tweet was already retweeted

---

# `xyz.taluslabs.social.twitter.get-user-by-id@1`

Standard Nexus Tool that retrieves a user from the Twitter API by their ID. Twitter api [reference](https://developer.twitter.com/en/docs/twitter-api/users/lookup/api-reference/get-users-id)

## Input

**`bearer_token`: [`String`]**

The bearer token for the user's Twitter account.

**`user_id`: [`String`]**

The ID of the User to lookup (e.g. "2244994945").

_opt_ **`user_fields`: [`Option<Vec<UserField>>`]** _default_: [`None`]

A comma separated list of User fields to display.

_opt_ **`expansions_fields`: [`Option<Vec<ExpansionField>>`]** _default_: [`None`]

A comma separated list of fields to expand.

_opt_ **`tweet_fields`: [`Option<Vec<TweetField>>`]** _default_: [`None`]

A comma separated list of Tweet fields to display.

## Output Variants & Ports

**`ok`**

The user was retrieved successfully.

- **`ok.id`: [`String`]** - The user's unique identifier
- **`ok.name`: [`String`]** - The user's display name
- **`ok.username`: [`String`]** - The user's @username
- **`ok.protected`: [`Option<bool>`]** - Whether the user's account is protected
- **`ok.affiliation`: [`Option<Affiliation>`]** - The user's affiliation information
- **`ok.connection_status`: [`Option<Vec<ConnectionStatus>>`]** - The user's connection status
- **`ok.created_at`: [`Option<String>`]** - When the user's account was created
- **`ok.description`: [`Option<String>`]** - The user's profile description/bio
- **`ok.entities`: [`Option<Entities>`]** - Entities found in the user's description (hashtags, mentions, URLs)
- **`ok.location`: [`Option<String>`]** - The user's location
- **`ok.most_recent_tweet_id`: [`Option<String>`]** - ID of the user's most recent tweet
- **`ok.pinned_tweet_id`: [`Option<String>`]** - ID of the user's pinned tweet
- **`ok.profile_banner_url`: [`Option<String>`]** - URL of the user's profile banner image
- **`ok.profile_image_url`: [`Option<String>`]** - URL of the user's profile image
- **`ok.public_metrics`: [`Option<PublicMetrics>`]** - Public metrics about the user:
  - `followers_count`: Number of followers
  - `following_count`: Number of accounts the user is following
  - `tweet_count`: Number of tweets the user has posted
  - `listed_count`: Number of lists the user appears on
- **`ok.receives_your_dm`: [`Option<bool>`]** - Whether the user accepts direct messages
- **`ok.subscription_type`: [`Option<SubscriptionType>`]** - The user's subscription type
- **`ok.url`: [`Option<String>`]** - The user's website URL
- **`ok.verified`: [`Option<bool>`]** - Whether the user is verified
- **`ok.verified_type`: [`Option<VerifiedType>`]** - The user's verification type
- **`ok.withheld`: [`Option<Withheld>`]** - Withholding information for the user

**`err`**

The user was not retrieved due to an error.

- **`err.reason`: [`String`]** - Detailed error message describing what went wrong
- **`err.kind`: [`TwitterErrorKind`]** - The type of error that occurred. Possible values:
  - `network` - A network-related error occurred when connecting to Twitter
  - `connection` - Could not establish a connection to Twitter
  - `timeout` - The request to Twitter timed out
  - `parse` - Failed to parse Twitter's response
  - `auth` - Authentication or authorization error
  - `not_found` - The requested user was not found
  - `rate_limit` - Twitter's rate limit was exceeded
  - `server` - An error occurred on Twitter's servers
  - `forbidden` - The request was forbidden
  - `api` - An API-specific error occurred
  - `unknown` - An unexpected error occurred
- **`err.status_code`: [`Option<u16>`]** - The HTTP status code returned by Twitter, if available. Common codes include:
  - `401` - Unauthorized (authentication error)
  - `403` - Forbidden
  - `404` - Not Found
  - `429` - Too Many Requests (rate limit exceeded)
  - `5xx` - Server errors

It's important to note that some errors may have either a specific error kind (like `NotFound`, `Auth`, or `RateLimit`) or the more general `Api` error kind, and the status code may be a specific value or `None` depending on the error details.

---

# `xyz.taluslabs.social.twitter.get-user-by-username@1`

Standard Nexus Tool that retrieves a user from the Twitter API by username. Twitter api [reference](https://developer.twitter.com/en/docs/twitter-api/users/lookup/api-reference/get-users-by-username-username)

## Input

**`bearer_token`: [`String`]**

The bearer token for the user's Twitter account.

**`username`: [`String`]**

The username to retrieve (without the @ symbol).

_opt_ **`user_fields`: [`Option<Vec<UserField>>`]** _default_: [`None`]

A list of User fields to display.

_opt_ **`expansions_fields`: [`Option<Vec<ExpansionField>>`]** _default_: [`None`]

A list of fields to expand.

_opt_ **`tweet_fields`: [`Option<Vec<TweetField>>`]** _default_: [`None`]

A list of Tweet fields to display.

## Output Variants & Ports

**`ok`**

The user was retrieved successfully.

- **`ok.id`: [`String`]** - The user's unique identifier
- **`ok.name`: [`String`]** - The user's display name
- **`ok.username`: [`String`]** - The user's @username
- **`ok.protected`: [`Option<bool>`]** - Whether the user's account is protected
- **`ok.affiliation`: [`Option<Affiliation>`]** - The user's affiliation information
- **`ok.connection_status`: [`Option<Vec<ConnectionStatus>>`]** - The user's connection status
- **`ok.created_at`: [`Option<String>`]** - When the user's account was created
- **`ok.description`: [`Option<String>`]** - The user's profile description/bio
- **`ok.entities`: [`Option<Entities>`]** - Entities found in the user's description (hashtags, mentions, URLs)
- **`ok.location`: [`Option<String>`]** - The user's location
- **`ok.most_recent_tweet_id`: [`Option<String>`]** - ID of the user's most recent tweet
- **`ok.pinned_tweet_id`: [`Option<String>`]** - ID of the user's pinned tweet
- **`ok.profile_banner_url`: [`Option<String>`]** - URL of the user's profile banner image
- **`ok.profile_image_url`: [`Option<String>`]** - URL of the user's profile image
- **`ok.public_metrics`: [`Option<PublicMetrics>`]** - Public metrics about the user:
  - `followers_count`: Number of followers
  - `following_count`: Number of accounts the user is following
  - `tweet_count`: Number of tweets the user has posted
  - `listed_count`: Number of lists the user appears on
- **`ok.receives_your_dm`: [`Option<bool>`]** - Whether the user accepts direct messages
- **`ok.subscription_type`: [`Option<SubscriptionType>`]** - The user's subscription type
- **`ok.url`: [`Option<String>`]** - The user's website URL
- **`ok.verified`: [`Option<bool>`]** - Whether the user is verified
- **`ok.verified_type`: [`Option<VerifiedType>`]** - The user's verification type
- **`ok.withheld`: [`Option<Withheld>`]** - Withholding information for the user
- **`ok.includes`: [`Option<Includes>`]** - Additional entities related to the user:
  - `users`: Other users referenced by this user
  - `tweets`: Tweets referenced by this user (e.g., pinned tweet)
  - `media`: Media items referenced by this user
  - `places`: Geographic places referenced by this user
  - `polls`: Polls referenced by this user

**`err`**

The user was not retrieved due to an error.

- **`err.reason`: [`String`]** - The reason for the error. This could be:
  - Twitter API error with title and error type (e.g., "Twitter API error: Not Found Error (type: https://api.twitter.com/2/problems/resource-not-found)")
  - Twitter API error with optional detail and message (e.g., "Twitter API error: Not Found Error (type: https://api.twitter.com/2/problems/resource-not-found) - User not found")
  - Network error (e.g., "Network error: network error: Connection refused")
  - Response parsing error (e.g., "Response parsing error: expected value at line 1 column 1")
  - Status code error (e.g., "Twitter API status error: 429 Too Many Requests")
  - User not found error (e.g., "Twitter API error: User not found (code: 50)")
  - Invalid token error (e.g., "Twitter API error: Invalid token (code: 89)")
  - Rate limit exceeded error (e.g., "Twitter API error: Rate limit exceeded (code: 88)")
  - No user data found in the response

---

# `xyz.taluslabs.social.twitter.get-users-by-username@1`

Standard Nexus Tool that retrieves multiple users from the Twitter API by their usernames. Twitter api [reference](https://docs.x.com/x-api/users/user-lookup-by-usernames)

## Input

**`bearer_token`: [`String`]**

The bearer token for the user's Twitter account.

**`usernames`: [`Vec<String>`]**

A list of usernames to retrieve (without the @ symbol).

_opt_ **`user_fields`: [`Option<Vec<UserField>>`]** _default_: [`None`]

A list of User fields to display.

_opt_ **`expansions_fields`: [`Option<Vec<ExpansionField>>`]** _default_: [`None`]

A list of fields to expand.

_opt_ **`tweet_fields`: [`Option<Vec<TweetField>>`]** _default_: [`None`]

A list of Tweet fields to display.

## Output Variants & Ports

**`ok`**

The users were retrieved successfully.

- **`ok.users`: [`Vec<UserData>`]** - Array of user data objects with the following fields:
  - `id`: The user's unique identifier
  - `name`: The user's display name
  - `username`: The user's @username
  - `protected`: Whether the user's account is protected
  - `affiliation`: The user's affiliation information
  - `connection_status`: The user's connection status
  - `created_at`: When the user's account was created
  - `description`: The user's profile description/bio
  - `entities`: Entities found in the user's description
  - `location`: The user's location
  - `most_recent_tweet_id`: ID of the user's most recent tweet
  - `pinned_tweet_id`: ID of the user's pinned tweet
  - `profile_banner_url`: URL of the user's profile banner image
  - `profile_image_url`: URL of the user's profile image
  - `public_metrics`: Public metrics about the user
  - `receives_your_dm`: Whether the user accepts direct messages
  - `subscription_type`: The user's subscription type
  - `url`: The user's website URL
  - `verified`: Whether the user is verified
  - `verified_type`: The user's verification type
  - `withheld`: Withholding information for the user
- **`ok.includes`: [`Option<Includes>`]** - Additional entities related to the users:
  - `users`: Other users referenced by these users
  - `tweets`: Tweets referenced by these users (e.g., pinned tweets)
  - `media`: Media items referenced by these users
  - `places`: Geographic places referenced by these users
  - `polls`: Polls referenced by these users

**`err`**

The users were not retrieved due to an error.

- **`err.reason`: [`String`]** - A detailed error message describing what went wrong
- **`err.kind`: [`TwitterErrorKind`]** - The type of error that occurred. Possible values:

  - `network` - A network-related error occurred when connecting to Twitter
  - `connection` - Could not establish a connection to Twitter
  - `timeout` - The request to Twitter timed out
  - `parse` - Failed to parse Twitter's response
  - `auth` - Authentication or authorization error
  - `not_found` - The requested user was not found
  - `rate_limit` - Twitter's rate limit was exceeded
  - `server` - An error occurred on Twitter's servers
  - `forbidden` - The request was forbidden
  - `api` - An API-specific error occurred
  - `unknown` - An unexpected error occurred

- **`err.status_code`: [`Option<u16>`]** - The HTTP status code returned by Twitter, if available. Common codes include:
  - `401` - Unauthorized (authentication error)
  - `403` - Forbidden
  - `404` - Not Found
  - `429` - Too Many Requests (rate limit exceeded)
  - `5xx` - Server errors

It's important to note that some errors may have either a specific error kind (like `NotFound`, `Auth`, or `RateLimit`) or the more general `Api` error kind, and the status code may be a specific value or `None` depending on the error details.

---

# `xyz.taluslabs.social.twitter.get-users@1`

Standard Nexus Tool that retrieves multiple users by their IDs. Twitter api [reference](https://docs.x.com/x-api/users/user-lookup-by-ids#user-lookup-by-ids)

## Input

**`bearer_token`: [`String`]**

The bearer token for the user's Twitter account.

**`ids`: [`Vec<String>`]**

A list of User IDs to lookup (up to 100). Example: ["2244994945", "6253282", "12"]

_opt_ **`user_fields`: [`Option<Vec<UserField>>`]** _default_: [`None`]

A list of User fields to display. Available values include: affiliation, confirmed_email, connection_status, created_at, description, entities, id, is_identity_verified, location, most_recent_tweet_id, name, parody, pinned_tweet_id, profile_banner_url, profile_image_url, protected, public_metrics, receives_your_dm, subscription, subscription_type, url, username, verified, verified_followers_count, verified_type, withheld

_opt_ **`expansions`: [`Option<Vec<ExpansionField>>`]** _default_: [`None`]

A list of fields to expand. Available values include: affiliation.user_id, most_recent_tweet_id, pinned_tweet_id

_opt_ **`tweet_fields`: [`Option<Vec<TweetField>>`]** _default_: [`None`]

A list of Tweet fields to display when using expansions to include referenced tweets.

## Output Variants & Ports

**`ok`**

The users were retrieved successfully.

- **`ok.users`: [`Vec<UserData>`]** - The collection of user data from the API:
  - `id`: The user's unique identifier
  - `name`: The user's display name
  - `username`: The user's @username
  - `protected`: Whether the user's account is protected
  - `affiliation`: The user's affiliation information
  - `connection_status`: The user's connection status
  - `created_at`: When the user's account was created
  - `description`: The user's profile description/bio
  - `entities`: Entities found in the user's description
  - `location`: The user's location
  - `most_recent_tweet_id`: ID of the user's most recent tweet
  - `pinned_tweet_id`: ID of the user's pinned tweet
  - `profile_banner_url`: URL of the user's profile banner image
  - `profile_image_url`: URL of the user's profile image
  - `public_metrics`: Public metrics about the user
  - `receives_your_dm`: Whether the user accepts direct messages
  - `subscription_type`: The user's subscription type
  - `url`: The user's website URL
  - `verified`: Whether the user is verified
  - `verified_type`: The user's verification type
  - `withheld`: Withholding information for the user
- **`ok.includes`: [`Option<Includes>`]** - Additional entities related to the users:
  - `users`: Other users referenced
  - `tweets`: Tweets referenced (e.g., pinned tweets)
  - `media`: Media items referenced
  - `places`: Geographic places referenced
  - `polls`: Polls referenced

**`err`**

The users were not retrieved due to an error.

- **`err.reason`: [`String`]** - A detailed error message describing what went wrong
- **`err.kind`: [`TwitterErrorKind`]** - The type of error that occurred. Possible values:
  - `network` - A network-related error occurred when connecting to Twitter
  - `connection` - Could not establish a connection to Twitter
  - `timeout` - The request to Twitter timed out
  - `parse` - Failed to parse Twitter's response
  - `auth` - Authentication or authorization error
  - `not_found` - The requested users were not found
  - `rate_limit` - Twitter's rate limit was exceeded
  - `server` - An error occurred on Twitter's servers
  - `forbidden` - The request was forbidden
  - `api` - An API-specific error occurred
  - `unknown` - An unexpected error occurred
- **`err.status_code`: [`Option<u16>`]** - The HTTP status code returned by Twitter, if available. Common codes include:
  - `401` - Unauthorized (authentication error)
  - `403` - Forbidden
  - `404` - Not Found
  - `429` - Too Many Requests (rate limit exceeded)
  - `5xx` - Server errors

It's important to note that some errors may have either a specific error kind (like `NotFound`, `Auth`, or `RateLimit`) or the more general `Api` error kind, and the status code may be a specific value or `None` depending on the error details.

---

# `xyz.taluslabs.social.twitter.create-list@1`

Standard Nexus Tool that creates a new list on Twitter.
Twitter api [reference](https://developer.twitter.com/en/docs/twitter-api/lists/manage-lists/api-reference/post-lists)

## Input

**Authentication Parameters**

The following authentication parameters are provided as part of the TwitterAuth structure:

- **`consumer_key`: [`String`]** - Twitter API application's Consumer Key
- **`consumer_secret_key`: [`String`]** - Twitter API application's Consumer Secret Key
- **`access_token`: [`String`]** - Access Token for user's Twitter account
- **`access_token_secret`: [`String`]** - Access Token Secret for user's Twitter account

**Additional Parameters**

**`name`: [`String`]**

The name of the list.

_opt_ **`description`: [`Option<String>`]** _default_: [`None`]

Description of the list.

_opt_ **`private`: [`Option<bool>`]** _default_: [`None`]

Determines if the list is private (true) or public (false).

## Output Variants & Ports

**`ok`**

The list was created successfully.

- **`ok.result`** - The created list data containing:
  - `id`: The list's unique identifier
  - `name`: The name of the list
  - And other details like description, member count, follower count, etc.

**`err`**

The list creation failed.

- **`err.reason`: [`String`]** - The reason for the error. This could be:
  - Twitter API error (e.g., "Twitter API error: Not Found Error (type: https://api.twitter.com/2/problems/resource-not-found)")
  - Network error (e.g., "Network error: network error: Connection refused")
  - Response parsing error
  - Status code error
  - Other error types handled by the centralized error handling mechanism

---

# `xyz.taluslabs.social.twitter.get-list@1`

Standard Nexus Tool that retrieves a list from Twitter by ID.
Twitter api [reference](https://developer.twitter.com/en/docs/twitter-api/lists/list-lookup/api-reference/get-lists-id)

## Input

**`bearer_token`: [`String`]**

The bearer token for the user's Twitter account.

**`list_id`: [`String`]**

The ID of the list to retrieve.

_opt_ **`list_fields`: [`Option<Vec<ListField>>`]** _default_: [`None`]

A list of List fields to display.

_opt_ **`expansions`: [`Option<Vec<ExpansionField>>`]** _default_: [`None`]

A list of fields to expand.

_opt_ **`user_fields`: [`Option<Vec<UserField>>`]** _default_: [`None`]

A list of User fields to display.

## Output Variants & Ports

**`ok`**

The list was retrieved successfully.

- **`ok.id`: [`String`]** - The list's unique identifier
- **`ok.name`: [`String`]** - The list's name
- **`ok.created_at`: [`Option<String>`]** - The timestamp when the list was created
- **`ok.description`: [`Option<String>`]** - The list's description
- **`ok.follower_count`: [`Option<i32>`]** - Number of followers this list has
- **`ok.member_count`: [`Option<i32>`]** - Number of members in this list
- **`ok.owner_id`: [`Option<String>`]** - The ID of the list's owner
- **`ok.private`: [`Option<bool>`]** - Whether the list is private or public
- **`ok.includes`: [`Option<Includes>`]** - Additional entities related to the list
- **`ok.meta`: [`Option<Meta>`]** - Metadata about the list request

**`err`**

The list retrieval failed.

- **`err.reason`: [`String`]** - A detailed error message describing what went wrong
- **`err.kind`: [`TwitterErrorKind`]** - The type of error that occurred. Possible values:
  - `network` - A network-related error occurred when connecting to Twitter
  - `connection` - Could not establish a connection to Twitter
  - `timeout` - The request to Twitter timed out
  - `parse` - Failed to parse Twitter's response
  - `auth` - Authentication or authorization error
  - `not_found` - The requested list was not found
  - `rate_limit` - Twitter's rate limit was exceeded
  - `server` - An error occurred on Twitter's servers
  - `forbidden` - The request was forbidden
  - `api` - An API-specific error occurred
  - `unknown` - An unexpected error occurred
- **`err.status_code`: [`Option<u16>`]** - The HTTP status code returned by Twitter, if available. Common codes include:
  - `401` - Unauthorized (authentication error)
  - `403` - Forbidden
  - `404` - Not Found
  - `429` - Too Many Requests (rate limit exceeded)
  - `5xx` - Server errors

---

# `xyz.taluslabs.social.twitter.get-list-tweets@1`

Standard Nexus Tool that retrieves tweets from a Twitter list.
Twitter api [reference](https://developer.twitter.com/en/docs/twitter-api/lists/list-tweets/api-reference/get-lists-id-tweets)

## Input

**`bearer_token`: [`String`]**

The bearer token for the user's Twitter account.

**`list_id`: [`String`]**

The ID of the list to retrieve tweets from.

_opt_ **`max_results`: [`Option<i32>`]** _default_: [`None`]

The maximum number of results to retrieve (range: 5-100).

_opt_ **`pagination_token`: [`Option<String>`]** _default_: [`None`]

Used to get the next 'page' of results.

_opt_ **`tweet_fields`: [`Option<Vec<TweetField>>`]** _default_: [`None`]

A list of Tweet fields to display.

_opt_ **`expansions`: [`Option<Vec<ExpansionField>>`]** _default_: [`None`]

A list of fields to expand.

_opt_ **`media_fields`: [`Option<Vec<MediaField>>`]** _default_: [`None`]

A list of Media fields to display.

_opt_ **`poll_fields`: [`Option<Vec<PollField>>`]** _default_: [`None`]

A list of Poll fields to display.

_opt_ **`user_fields`: [`Option<Vec<UserField>>`]** _default_: [`None`]

A list of User fields to display.

_opt_ **`place_fields`: [`Option<Vec<PlaceField>>`]** _default_: [`None`]

A list of Place fields to display.

## Output Variants & Ports

**`ok`**

The list tweets were retrieved successfully.

- **`ok.data`: [`Option<Vec<Tweet>>`]** - The collection of tweets from the list.
- **`ok.includes`: [`Option<Includes>`]** - Additional data included in the response.
- **`ok.meta`: [`Option<Meta>`]** - Metadata about the response.

**`err`**

The list tweets retrieval failed.

- **`err.reason`: [`String`]** - A detailed error message describing what went wrong
- **`err.kind`: [`TwitterErrorKind`]** - The type of error that occurred. Possible values:
  - `network` - A network-related error occurred when connecting to Twitter
  - `connection` - Could not establish a connection to Twitter
  - `timeout` - The request to Twitter timed out
  - `parse` - Failed to parse Twitter's response
  - `auth` - Authentication or authorization error
  - `not_found` - The requested list was not found
  - `rate_limit` - Twitter's rate limit was exceeded
  - `server` - An error occurred on Twitter's servers
  - `forbidden` - The request was forbidden
  - `api` - An API-specific error occurred
  - `unknown` - An unexpected error occurred
- **`err.status_code`: [`Option<u16>`]** - The HTTP status code returned by Twitter, if available

---

# `xyz.taluslabs.social.twitter.get-list-members@1`

Standard Nexus Tool that retrieves members of a Twitter list.
Twitter api [reference](https://developer.twitter.com/en/docs/twitter-api/lists/list-members/api-reference/get-lists-id-members)

## Input

**`bearer_token`: [`String`]**

The bearer token for the user's Twitter account.

**`list_id`: [`String`]**

The ID of the list to retrieve members from.

_opt_ **`max_results`: [`Option<i32>`]** _default_: [`None`]

The maximum number of results to retrieve (range: 1-100).

_opt_ **`pagination_token`: [`Option<String>`]** _default_: [`None`]

Used to get the next 'page' of results.

_opt_ **`user_fields`: [`Option<Vec<UserField>>`]** _default_: [`None`]

A list of User fields to display.

_opt_ **`expansions`: [`Option<Vec<ExpansionField>>`]** _default_: [`None`]

A list of fields to expand.

_opt_ **`tweet_fields`: [`Option<Vec<TweetField>>`]** _default_: [`None`]

A list of Tweet fields to display.

## Output Variants & Ports

**`ok`**

The list members were retrieved successfully.

- **`ok.data`: [`Option<Vec<UserData>>`]** - The collection of users who are members of the list.
- **`ok.includes`: [`Option<Includes>`]** - Additional data included in the response.
- **`ok.meta`: [`Option<Meta>`]** - Metadata about the response.

**`err`**

The list members retrieval failed.

- **`err.reason`: [`String`]** - A detailed error message describing what went wrong
- **`err.kind`: [`TwitterErrorKind`]** - The type of error that occurred. Possible values:
  - `network` - A network-related error occurred when connecting to Twitter
  - `connection` - Could not establish a connection to Twitter
  - `timeout` - The request to Twitter timed out
  - `parse` - Failed to parse Twitter's response
  - `auth` - Authentication or authorization error
  - `not_found` - The requested list was not found
  - `rate_limit` - Twitter's rate limit was exceeded
  - `server` - An error occurred on Twitter's servers
  - `forbidden` - The request was forbidden
  - `api` - An API-specific error occurred
  - `unknown` - An unexpected error occurred
- **`err.status_code`: [`Option<u16>`]** - The HTTP status code returned by Twitter, if available

---

# `xyz.taluslabs.social.twitter.update-list@1`

Standard Nexus Tool that updates an existing Twitter list.
Twitter api [reference](https://developer.twitter.com/en/docs/twitter-api/lists/manage-lists/api-reference/put-lists-id)

## Input

**Authentication Parameters**

The following authentication parameters are provided as part of the TwitterAuth structure:

- **`consumer_key`: [`String`]** - Twitter API application's Consumer Key
- **`consumer_secret_key`: [`String`]** - Twitter API application's Consumer Secret Key
- **`access_token`: [`String`]** - Access Token for user's Twitter account
- **`access_token_secret`: [`String`]** - Access Token Secret for user's Twitter account

**Additional Parameters**

**`list_id`: [`String`]**

The ID of the list to update.

_opt_ **`name`: [`Option<String>`]** _default_: [`None`]

The new name for the list.

_opt_ **`description`: [`Option<String>`]** _default_: [`None`]

The new description for the list.

_opt_ **`private`: [`Option<bool>`]** _default_: [`None`]

Whether the list should be private (true) or public (false).

## Output Variants & Ports

**`ok`**

The list was updated successfully.

- **`ok.updated`** - Confirmation that the list was updated (true).

**`err`**

The list update failed.

- **`err.reason`: [`String`]** - The reason for the error. This could be:
  - Twitter API error
  - Network error
  - Response parsing error
  - Status code error
  - Other error types handled by the centralized error handling mechanism

---

# `xyz.taluslabs.social.twitter.add-member@1`

Standard Nexus Tool that adds a user to a Twitter list.
Twitter api [reference](https://developer.twitter.com/en/docs/twitter-api/lists/list-members/api-reference/post-lists-id-members)

## Input

**Authentication Parameters**

The following authentication parameters are provided as part of the TwitterAuth structure:

- **`consumer_key`: [`String`]** - Twitter API application's Consumer Key
- **`consumer_secret_key`: [`String`]** - Twitter API application's Consumer Secret Key
- **`access_token`: [`String`]** - Access Token for user's Twitter account
- **`access_token_secret`: [`String`]** - Access Token Secret for user's Twitter account

**Additional Parameters**

**`list_id`: [`String`]**

The ID of the list to which a member will be added.

**`user_id`: [`String`]**

The ID of the user to add to the list.

## Output Variants & Ports

**`ok`**

The user was successfully added to the list.

- \*\*`ok.is_member`
- **`ok.is_member`** - Confirmation that the user is a member of the list (true).

**`err`**

The user could not be added to the list.

- **`err.reason`: [`String`]** - The reason for the error. This could be:
  - Twitter API error
  - Network error
  - Response parsing error
  - Status code error
  - Other error types handled by the centralized error handling mechanism

---

# `xyz.taluslabs.social.twitter.remove-member@1`

Standard Nexus Tool that removes a user from a Twitter list.
Twitter api [reference](https://developer.twitter.com/en/docs/twitter-api/lists/list-members/api-reference/delete-lists-id-members-user_id)

## Input

**Authentication Parameters**

The following authentication parameters are provided as part of the TwitterAuth structure:

- **`consumer_key`: [`String`]** - Twitter API application's Consumer Key
- **`consumer_secret_key`: [`String`]** - Twitter API application's Consumer Secret Key
- **`access_token`: [`String`]** - Access Token for user's Twitter account
- **`access_token_secret`: [`String`]** - Access Token Secret for user's Twitter account

**Additional Parameters**

**`list_id`: [`String`]**

The ID of the list from which a member will be removed.

**`user_id`: [`String`]**

The ID of the user to remove from the list.

## Output Variants & Ports

**`ok`**

The user was successfully removed from the list.

- **`ok.is_member`** - Confirmation that the user is not a member of the list (false).

**`err`**

The user could not be removed from the list.

- **`err.reason`: [`String`]** - The reason for the error. This could be:
  - Twitter API error
  - Network error
  - Response parsing error
  - Status code error
  - Other error types handled by the centralized error handling mechanism

---

# `xyz.taluslabs.social.twitter.retweet-tweet@1`

Standard Nexus Tool that retweets a specific tweet.
Twitter api [reference](https://docs.x.com/x-api/posts/causes-the-user-in-the-path-to-retweet-the-specified-post)

## Input

**Authentication Parameters**

The following authentication parameters are provided as part of the TwitterAuth structure:

- **`consumer_key`: [`String`]** - Twitter API application's Consumer Key
- **`consumer_secret_key`: [`String`]** - Twitter API application's Consumer Secret Key
- **`access_token`: [`String`]** - Access Token for user's Twitter account
- **`access_token_secret`: [`String`]** - Access Token Secret for user's Twitter account

**Additional Parameters**

**`user_id`: [`String`]**

The ID of the authenticated user who will retweet the tweet.

**`tweet_id`: [`String`]**

The ID of the tweet to retweet.

## Output Variants & Ports

**`ok`**

The tweet was successfully retweeted.

- **`ok.tweet_id`: [`String`]** - The ID of the tweet that was retweeted
- **`ok.retweeted`: [`bool`]** - Confirmation that the tweet was retweeted (true)

**`err`**

The retweet operation failed.

- **`err.reason`: [`String`]** - A detailed error message describing what went wrong
- **`err.kind`: [`TwitterErrorKind`]** - The type of error that occurred. Possible values:
  - `network` - A network-related error occurred when connecting to Twitter
  - `connection` - Could not establish a connection to Twitter
  - `timeout` - The request to Twitter timed out
  - `parse` - Failed to parse Twitter's response
  - `auth` - Authentication or authorization error
  - `not_found` - The requested tweet was not found
  - `rate_limit` - Twitter's rate limit was exceeded
  - `server` - An error occurred on Twitter's servers
  - `forbidden` - The request was forbidden
  - `api` - An API-specific error occurred
  - `unknown` - An unexpected error occurred
- **`err.status_code`: [`Option<u16>`]** - The HTTP status code returned by Twitter, if available. Common codes include:
  - `401` - Unauthorized (authentication error)
  - `403` - Forbidden
  - `404` - Not Found
  - `429` - Too Many Requests (rate limit exceeded)
  - `5xx` - Server errors

It's important to note that some errors may have either a specific error kind (like `NotFound`, `Auth`, or `RateLimit`) or the more general `Api` error kind, and the status code may be a specific value or `None` depending on the error details.

---

# `xyz.taluslabs.social.twitter.get-recent-tweet-count@1`

Standard Nexus Tool that retrieves tweet counts for queries from the Twitter API. Twitter api [reference](https://developer.twitter.com/en/docs/twitter-api/tweets/counts/api-reference/get-tweets-counts-recent)

## Input

**`bearer_token`: [`String`]**

The bearer token for the user's Twitter account.

**`query`: [`String`]**

Search query for counting tweets.

_opt_ **`start_time`: [`Option<String>`]** _default_: [`None`]

The oldest UTC timestamp from which the tweets will be counted (YYYY-MM-DDTHH:mm:ssZ).

_opt_ **`end_time`: [`Option<String>`]** _default_: [`None`]

The newest UTC timestamp to which the tweets will be counted (YYYY-MM-DDTHH:mm:ssZ).

_opt_ **`since_id`: [`Option<String>`]** _default_: [`None`]

Returns results with a tweet ID greater than (more recent than) the specified ID.

_opt_ **`until_id`: [`Option<String>`]** _default_: [`None`]

Returns results with a tweet ID less than (older than) the specified ID.

_opt_ **`next_token`: [`Option<String>`]** _default_: [`None`]

Token for pagination to get the next page of results.

_opt_ **`pagination_token`: [`Option<String>`]** _default_: [`None`]

Alternative parameter for pagination (same as next_token).

_opt_ **`granularity`: [`Option<Granularity>`]** _default_: [`Granularity::Hour`]

Time granularity for the counts. Options are:

- `Minute`: Minute-by-minute counts
- `Hour`: Hourly counts (default)
- `Day`: Daily counts

_opt_ **`search_count_fields`: [`Option<Vec<String>>`]** _default_: [`None`]

A comma separated list of SearchCount fields to display.

## Output Variants & Ports

**`ok`**

The tweet counts were retrieved successfully.

- **`ok.data`: [`Vec<TweetCount>`]** - The collection of tweet count data:
  - `start`: Start time for the count bucket
  - `end`: End time for the count bucket
  - `tweet_count`: Number of tweets counted in this time period
- **`ok.meta`: [`Option<TweetCountMeta>`]** - Metadata about the counts:
  - `newest_id`: The newest tweet ID in the response
  - `next_token`: Token for the next page of results
  - `oldest_id`: The oldest tweet ID in the response
  - `total_tweet_count`: Total count of tweets matching the query

**`err`**

The tweet counts could not be retrieved due to an error.

- **`err.kind`: [`TwitterErrorKind`]** - The type of error that occurred. Possible values:

  - `network` - A network-related error occurred when connecting to Twitter
  - `connection` - Could not establish a connection to Twitter
  - `timeout` - The request to Twitter timed out
  - `parse` - Failed to parse Twitter's response
  - `auth` - Authentication or authorization error
  - `not_found` - The requested tweet or resource was not found
  - `rate_limit` - Twitter's rate limit was exceeded
  - `server` - An error occurred on Twitter's servers
  - `forbidden` - The request was forbidden
  - `api` - An API-specific error occurred
  - `unknown` - An unexpected error occurred

- **`err.reason`: [`String`]** - The reason for the error. This could be:

  - Twitter API error (e.g., "Twitter API returned errors: Invalid Request: One or more parameters to your request was invalid.")
  - Validation error (e.g., "Validation error: Invalid start_time format. Expected format: YYYY-MM-DDTHH:mm:ssZ")
  - Network error (e.g., "Network error: network error: Connection refused")
  - Response parsing error (e.g., "Response parsing error: expected value at line 1 column 1")
  - Status code error (e.g., "Twitter API status error: 429 Too Many Requests")
  - "No tweet count data found" when the API response doesn't contain count data
  - Unauthorized error (e.g., "Unauthorized")
  - Other error types handled by the centralized error handling mechanism

- **`err.status_code`: [`Option<u16>`]** - The HTTP status code returned by Twitter, if available. Common codes include:
  - `401` - Unauthorized (authentication error)
  - `403` - Forbidden
  - `404` - Not Found
  - `429` - Too Many Requests (rate limit exceeded)
  - `5xx` - Server errors

---

# Error Handling

The Twitter SDK includes a centralized error handling system that provides consistent error responses across all modules. This system includes:

## Error Types (TwitterErrorKind)

The `err.kind` field provides a categorized error type for easier programmatic handling:

- **`network`**: A network-related error occurred when connecting to Twitter
- **`connection`**: Could not establish a connection to Twitter
- **`timeout`**: The request to Twitter timed out
- **`parse`**: Failed to parse Twitter's response
- **`auth`**: Authentication or authorization error
- **`not_found`**: The requested tweet or resource was not found
- **`rate_limit`**: Twitter's rate limit was exceeded
- **`server`**: An error occurred on Twitter's servers
- **`forbidden`**: The request was forbidden
- **`api`**: An API-specific error occurred
- **`unknown`**: An unexpected error occurred

## Error Structure

Each error includes three primary components:

1. **`kind` (TwitterErrorKind)**: The categorized error type (as described above)
2. **`reason` (String)**: A descriptive message that provides details about the error
3. **`status_code` (Option<u16>)**: The HTTP status code returned by Twitter API, if available

### Common Status Codes

- `401`: Unauthorized (authentication error)
- `403`: Forbidden
- `404`: Not Found
- `429`: Too Many Requests (rate limit exceeded)
- `5xx`: Server errors

### Error Message Format

The `reason` field follows a consistent format:

- Network errors: `"Network error: [error details]"`
- Parse errors: `"Response parsing error: [error details]"`
- API errors: `"Twitter API error: [title] (type: [error_type]) - [detail]"`
- Status errors: `"Twitter API status error: [status code]"`
- Other errors: `"Unknown error: [message]"`

## Retryable Errors

Some error types are considered "retryable" and can be attempted again after appropriate backoff:

- `rate_limit`: Consider retrying after the duration specified in the error message
- `network`: Network errors may be temporary and can be retried
- `server`: Server errors (5xx) may be temporary and can be retried

Other error types typically require fixing the request (e.g., `auth`, `not_found`, `forbidden`) and should not be retried without modification.

## Error Handling in Modules

All modules use the `TwitterResult<T>` type for handling errors, which is a type alias for `Result<T, TwitterError>`. This ensures consistent error propagation and formatting throughout the SDK.

The error handling system makes it easier to debug issues with Twitter API calls and provides clear, actionable error messages to end users. The structured error information allows for programmatic handling of specific error conditions.
