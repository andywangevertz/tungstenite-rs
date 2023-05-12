openssl genrsa -des3 -out domain.key 2048
#openssl req -key domain.key -new -out domain.csr
#openssl x509 -signkey domain.key -in domain.csr -req -days 3650 -out domain.crt
openssl req -x509 -sha256 -newkey rsa:2048 -nodes -keyout domain.key -x509 -days 3650 -out domain.crt
openssl pkcs12 -inkey domain.key -in domain.crt -export -out domain.pfx

