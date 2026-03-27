# OpenWordGuess - Server
This project aims to provide an openly accessable, multy lingual and category based database for Taboo like word guesssing games.
Words will be submittable by the user and verified by administrative accounts, to prevent harmful content from being deployed.

## Routes
These routes should be implemented by all clients.

### Get words
```
GET /api/words/:language/:category
```
```json
[
  {
    "word" : "example",
    "taboos" : [
      "descriptive word for example"
    ],
  }
]
```
Category is an optional parameter. If left blank, it will return all words, for the given language.
The request will only return words, wich where already reviewed.

### Submit a new word
```
POST /api/addWord/
```
The request body, should look like this:
```json
{
  "word" : "example",
  "language": "en",
  "category": "testing",
  "taboos" : [
    "descriptive word for example"
  ],
}
```

## Administrative routes
The implementation of these routes is optional. A web interface is available at ```/review```

All requests in for the ```/api/review``` routes, should contain at least the following body:

```json
{
  WIP
}

```

### Get pending word requests
```
GET /api/review/words/
```

```json
[
  {
    "word" : "example",
    "reviewed" : false,
    "taboos" : [
      "descriptive word for example"
    ],
  }
]
```

### Accept pending word
```
POST /api/review/accept/:word
```

### Deny pending word
```
POST /api/review/deny/:word
```
