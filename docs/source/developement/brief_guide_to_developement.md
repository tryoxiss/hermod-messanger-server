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