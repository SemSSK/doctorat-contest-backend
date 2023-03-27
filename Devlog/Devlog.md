# Feature implementation order
symbol meaning 

👍 validated, ✅ implemented


1. Setup the project with the dependencies ✅
2. Implement the account system with authentication and authorization. ✅
3. adding account management. ✅
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
```
started 26/03/2023 19:08
```
virtual platform is created by the admin who affects it a name and a VD.

one post api should do the trick

```
finished 26/03/2023 19:41 (may need tweeking) 
```

# Entry 5 Adding bulk insert of user data
```
started 26/03/2023 00:05
```

this is necessary as it allows the client to load multiple user data like in a csv and insert it

to achieve this it is necessary to change tactics in our query method and use query builder instead of query! macro

we use it like so: 

```rust
   sqlx::QueryBuilder::new("insert into Edl.User(email,password,role,specialty)") // creating the request 
          .push_values(users, // giving it the data source 
          |mut b,u| {  // closure detailing how to map the data 
            b.push_bind(u.email) 
              .push_bind(u.password)
              .push_bind(u.role)
              .push_bind(u.specialty);
          })
          .build()
          .execute(pool)
          .await?;
```

Note that the api receives a json array object as the csv parsing needs to be done on the client side this allows it to be more versitile on the data types it receives not just csv.

```
started 26/03/2023 00:22
```

***And with that administrator apis should be done now a peer review is the next step***


# Entry 6 Adding Virtual Platform management from the Vice doyen

the vice doyen needs to:

* create and alter a session which is done with a post rest api.
* affecting applicants and monitors to a session two  post rest apis.
* make announcements to sessions one post rest api.