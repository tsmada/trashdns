# TrashDNS

Lightweight rust based DNS server. Sort of delicious if you're that type of panda.

# Features

- Basic DNS resolution
- (Coming soon) Recusrive DNS

# Usage

- Run the script

```
trashdns % cargo run
warning: trashdns v0.1.0 (/Users/adamstruthers/Projects/trashdns) ignoring invalid dependency `trust-dns` which is missing a lib target
   Compiling trashdns v0.1.0 (/Users/adamstruthers/Projects/trashdns)
    Finished dev [unoptimized + debuginfo] target(s) in 0.46s
     Running `target/debug/trashdns`
Starting DNS server on 53
Responding to localhost query A
```

Send a DNS query to it:

```
dig @localhost localhost A +notcp
```

Result:

```
; <<>> DiG 9.10.6 <<>> @localhost localhost A +notcp
; (2 servers found)
;; global options: +cmd
;; Got answer:
;; ->>HEADER<<- opcode: QUERY, status: NOERROR, id: 17850
;; flags: qr; QUERY: 1, ANSWER: 2, AUTHORITY: 0, ADDITIONAL: 0

;; QUESTION SECTION:
;localhost.			IN	A

;; ANSWER SECTION:
localhost.		3600	IN	A	127.0.0.1
localhost.		3600	IN	AAAA	::1

;; Query time: 2 msec
;; SERVER: 127.0.0.1#53(127.0.0.1)
;; WHEN: Mon Jun 19 22:07:59 EDT 2023
;; MSG SIZE  rcvd: 71
```
