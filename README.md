##### Intro
There is no database involve in this application, it uses in memory global 
hashmap to store information on the server. All data manipulation is done in the 
data structure on local memory map.

###### Server
```
.
├── server
├── lib
├── client
└── ...
```

##### Prerequisites
- rustup

##### To boot server
`cargo run --bin server`


##### To start client
The client will just loop forever and fires 10 simultaneous request every seconds

`cargo run --bin client`

##### Sample curl
```
curl  -X DELETE -d '{"table":1,"item":1}' 'http://localhost:3000/order'

curl  -d '{"table":1}' 'http://localhost:3000/order'

curl  -X POST -d '{"table":1,"item":1}' 'http://localhost:3000/order'
```

