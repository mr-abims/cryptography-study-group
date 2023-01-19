# Questions from Week 4
Link to questions: https://uncloak.org/courses/rust+cryptography+engineering/course-2022-12-09+Session+4+Notes


## Exercise 1
a. `t = MAC(m)`, `c = E(m)`, send `(c, t)`

This is the scheme of Encrypt-and-MAC, which is a sound scheme. The recipient can verify authenticity by decrypting the message and computing the MAC of the decrypted value
to `t`. If they match then it has not been tampered with.

b. `t = MAC(m)`, `c = E(m||t)`, send `c`.

This scheme doesn't work since the recipient won't know how to partition the decrypted ciphertext into `m||t`. They would need additional data sent to specify how to
get `m` and `t` from `m||t`.

c. `c = E(m)`, `t = MAC(c)`, send `(c, t)`.

This is the Encrypt-then-MAC scheme, which works. The recipient can compute the MAC of `c` to check if it matches `t`. This is helpful as it doesn't require
decrypting before authenticity can be established. It is important however to use different keys for the MAC and encryption in order for this scheme to be secure.

## Exercise 2

1. The server sends its certificates.

(Authenticity) In this step, the adversary cannot impersonate the server since the certificate is publically verifiable by anyone and is only given to the authentic server. If the
adversary were to send a fake certificate, the client would stop the handshake since it would see that the certificate is false.

2. The server sends a Server Key Exchange message, initiating the key exchange and signing it with its public key.

(Confidentiality) The adversary cannot read the message since it does not have access to the server's private key. It is known that access to the public key plus a ciphertext is not sufficient for
obtaining the private key or the plainteixt.

3. The client sends a Client Key Exchange message, containing its part of the key exchange transaction.

(Integrity) If the adversary changes the message mid-transit, the key derivation step later on will fail so the handshake will terminate.




