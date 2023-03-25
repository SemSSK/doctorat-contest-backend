# Feature implementation order

1. Setup the project with the dependencies
2. Implement the account system with authentication and authorization.
3. adding account management.
3. Setup the virtual platform system.
4. Add Session Management.
5. Add the result management system.
6. Add theme creation,choices,and validation. 



##### vendredi 24 mars 2023
# Entry 1 implementing the authentication and authorization

![Image Explaining the authentication system briefly](https://www.vaadata.com/blog/wp-content/uploads/2016/12/JWT_tokens_EN.png)

Authentication will be set on a Http Post Listener that will receive a object containing the email and password of the user that will be verified in the database and that will return a JWT (Json Web Token) to the client.

The JWT will be used for authorizing all the subsequent requests.

To avoid CSRF the JWT will expire after 15 minutes.

To avoid having the client login every 15 minutes there will be a Get endpoint that refreshes the expiry of the JWT.

The JWT will contain the id, email, Role of the client.
