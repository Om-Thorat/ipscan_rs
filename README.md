# Blazingly fast IP Scanner in rust

A little script to scan all the IPs on your local network to check which ones are alive, using pinger (that is part of (gping)[https://github.com/orf/gping]) to ping IPs and taking full advantage of the cpu with threads to deliver a blazing fast performance.

`ps made this mainly as a dependency for another project`