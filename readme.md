# HMAC-SHA256 Demo

This is an example implementation of HMAC-SHA256 originally created as a proof of concept for adding authentication to my tactile-tesla project.

HMAC is actually quite simple. The basic premise is that by hashing your key with your data, you create a string of data that is unreasonably difficult to reproduce without the key, and proves that the data was not tampered with, and the key was included in the hashing process (not including it would yield a significantly different hash)
