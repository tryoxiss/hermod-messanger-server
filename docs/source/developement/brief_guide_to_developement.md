# A Guide to Developement


<!-- We need to make a shell script to automatically generate certificates for 
     easier development and kickstart -->
This is created with help from [techschool guru][https://dev.to/techschoolguru/how-to-create-sign-ssl-tls-certificates-2aai]

When asked for a password, enter a simple password used ONLY for testing. Our choice is `admin`.

1. openssl req -x509 -newkey rsa:4096 -days 365 -keyout ca-key.pem -out ca-cert.pem

    The country code
    The state or province name
    The organisation name
    The unit name
    The common name (or domain name)
    The email address

2. openssl req -newkey rsa:4096 -keyout server-key.pem -out server-req.pem -subj "/C=FR/ST=Ile de France/L=Paris/O=PC Book/OU=Computer/CN=*.pcbook.com/emailAddress=pcbook@gmail.com"
3. openssl x509 -req -in server-req.pem -CA ca-cert.pem -CAkey ca-key.pem -CAcreateserial -out server-cert.pem
4. openssl pkcs12 -export -out identity.pfx -inkey server-key.pem -in server-cert.pem
5. (OPTIONAL, NOT-YET-WORKING) open your web browser of choice to the test site and download the chain of certificates from the more information > certificate, download and put it into your bonfire-server directory
6. (OPTIONAL, NOT-YET-WORKING) openssl pkcs12 -export -out identity.pfx -inkey server-key.pem -in server-cert.pem -certfile chain_certs.pem
7. You should now be able to delete all the certificate files except `identity.pfx`, which is required. The others were only requires to create identity.pfx.


To check use this:
openssl verify -CAfile ca-cert.pem server-cert.pem

## File Requirements

Every file MUST contain the licence notice found in `source/_filenotice.txt`. It is not a requirement of the licence we use (AGPL v3 only), but of this project. It is incredbily easy to de-assosiate a file with a project, and therefore its licence. This increses the chance are code is used in ways not compliant with the licence. If every file has the attached notice, it shows clear intent to dis-assosiate it with the licence if that section is removed.

## Compiling for release

in your project location:
```sh
cargo build --release
# Now your built file is located at target/release/hermod-server, and can be executed with `./hermod-server`,

# Now lets test everything is working! But first, we need a few files
cp log4rs target/release/
cp identity.pfx target/release/ # Alternativly: just run the program twice, and it will have no issue.

./hermod-server
# Open the link it gives you, and you should get a packet served 
# TODO: Make it serve a generic "Operational!" thing when you go to like /test or somth

# Now lets remove those files and bundle the release
rm target/release/identity.pfx
rm target/release/log4rs.yml
```
