This crate demonstrates the use of c-gull's DNS resolver.

c-gull parses the output of the `getent` command, so its behavior should
respect the system DNS and NSS configuration, without using `dlopen`.
