# Documentation
--------------------------
Documentation for the web-api.

## Authentication?
----------------
Still unknown, what form of security is applicable to a local application?


Rough Target:
```json
GET Request: /search?top_left=(urlencoded lat,long)&bottom_right=(urlencoded lat,long) <- this is the URL that the front end will be sending through
Response: {
	"search_token": (unique token to look up search results)
}

GET Request: /results?token=(search_token) <- What the front is getting as a result to the above. Returns a token, we use this to update results
Response: {
	"results": [
		{
			MAP DESCRIPTOR HERE? FILE PATH?
		}
	],	
	"new": [
		(array of indexes)
	]
	
}
	
```
