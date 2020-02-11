# Baka.js
![Baka.js](https://riku.club/yKB2E.png)

## THESE DOCS ARE NOT UPDATED, ALOT OF STUFF HAS CHANGED!

Baka.js is an open source screenshot/filehost api.

Make sure you have mongodb and postman installed(or any software that does post requests). This is specifically made for a program called [sharex](https://getsharex.com/), but it can be used with any program that uploads using multipart data.

## Configuration
Open the .env.example file in the configuration
```
MONGOURL=mongodb://database:port/dbname
DBNAME=dbname
UPLOAD_DIR=./uploads/
REGKEY=mompickmeupimscared
URL=http://localhost:8080/
```
Change the mongo URL accordingly and make sure the same dbname is used in the two different sections. The URL value is where you will be accessing this publicly, by default this is localhost. REGKEY is a special key you will use for registering users. UPLOAD_DIR is the upload directory for files

## Users
**Creating users**

First step is to generate the user, open postman and make sure you are in the *Body* tab, then send a request to the `/user/signup` endpoint similar to this.
```JS
{
	"username": "henlo",
	"password": "secure",
	"regkey": "asdu980qnsmd09a87240jd054"
}
```
The regkey value is a static value in .env that is required to make a user, this is done for security and can be easily removed(unless you are an idiot).

If you did everything properly you should recieve a message that says `Success!`


**Deleting users**

To delete the user you need to send a request to `/user/delete` that looks something like this
```JS
{
	"username": "henlo",
	"password": "secure"
}
```
This will delete every file that belongs to you from the database and the server itself.


**Changing password**

To change the password send a request to `/user/passreset` that looks something like this

```JS
{
	"username": "henlo",
	"password": "secure",
	"newpassword": "more_secure"
}
```
## Tokens
**Getting tokens**

In order to upload you must have a user token, these are automatically generated and can be regenerated if you wish. Post to `/token/get` with a request like this
```JS
{
	"username": "henlo",
	"password": "secure"
}
```
This should get you a token that looks like this `7bc9cc67-bf55-4afe-a449-3a92a431df65`


**Regenerating tokens**

Lets say someone has your user token and you dont want them to have access to it, posting to `/token/regen` with a request like this
```JS
{
	"username": "henlo",
	"password": "secure"
}
```
This will regenerate your token, you will need to get your token from the API after this aswell,
my token after regenerating was `58cb61e2-b2b6-4487-841d-5e3205514c31`

## File
**Uploads**

Uploading is done through the `/files/upload` endpoint. In this scenario I will be using sharex, however anything that can upload through multipart will work. The sharex configuration file looks somewhat like this
```JS
{
  "Name": "Baka",
  "DestinationType": "ImageUploader, TextUploader, FileUploader",
  "RequestMethod": "POST",
  "RequestURL": "http://localhost:8080/files/upload",
  "Headers": {
    "token": "<your token>"
  },
  "Body": "MultipartFormData",
  "FileFormName": "uploadFile",
  "URL": "$response$",
  "ThumbnailURL": "$response$"
}
```
Change the `RequestURL` to where your instance is hosted and make sure that the `token` is valid and filled in with your token you generated earlier. Now sharex should function normally.


**Listing**

Listing owned files is done through the `/files/list` You must send a request similar to this 
```JS
{
	"username": "henlo",
	"password": "secure"
}
```
This will return an array with all your uploaded files, it will look like this
```JS
[
	"lvPBvk8.png",
	"Rognd4..png"
]
```


**Deleting**

Deleting is done through the `/files/delete` endpoint. You must send a request similar to this 
```JS
{
	"username": "henlo",
	"password": "secure",
	"file": "lvPBvk8.png"
}
```
This will delete the file from the database and the upload directory

> NOTE: The database and the upload directory are not the same, the database tracks what user owns which files, deleting a file from the upload directory will not delete it from the database and vice versa.

