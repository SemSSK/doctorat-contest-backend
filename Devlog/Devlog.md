# Feature implementation order

1. Setup the project with the dependencies
2. Implement the account system with authentication and authorization.
3. adding account management.
3. Setup the virtual platform system.
4. Add Session Management.
5. Add the result management system.
6. Add theme creation,choices,and validation. 



# Entry 1 implementing the authentication and authorization
```
started 24/03/2023 
```
![Image Explaining the authentication system briefly](https://www.vaadata.com/blog/wp-content/uploads/2016/12/JWT_tokens_EN.png)

Authentication will be set on a Http Post Listener that will receive a object containing the email and password of the user that will be verified in the database and that will return a JWT (Json Web Token) to the client.

The JWT will be used for authorizing all the subsequent requests.

To avoid CSRF the JWT will expire after 15 minutes.

To avoid having the client login every 15 minutes there will be a Get endpoint that refreshes the expiry of the JWT.

The JWT will contain the id, email, Role of the client.
```
finished 25/03/2023 01:16 
```

# Entry 2 Implementing account management
```
started 25/03/2023 13:46
```
Account management will be implemented by 5 Http apis only accessible by an admin client.

* get http api that takes consumes an ```id:i32``` and returns the corresponding user. Returns 404 in case of not found, 200 in the happy path.

* get http api that consumes diffrent fields and returns a list of users corresponding to them. Returns.

* post http api that adds a user to the database. Returns 404 in case of not found, 200 in the happy path.

* put http api that updates a user's field. Returns 404 in case of not found, 200 in the happy path.

* delete http api that deletes a user row. Returns 404 in case of not found, 200 in the happy path.
```
finished 25/03/2023 22:26
```


# Entry 3 Basic account modification
```
started 26/03/2023 14:51
```
Adding the option for the users to change their email and password note that specialty cannot be changed
security will be implemented directly into the sql request as it will use conditional update queries

will be implemented with two api endpoints as the password should not be modified with any other field

the other endpoint will containt email and any other data that may be added to the future tha each user should be allowed to modify
```
started 26/03/2023 15:17
```

# Entry 4 Virtual platform and Session creation and management