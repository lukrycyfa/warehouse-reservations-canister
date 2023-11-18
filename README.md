# warehouse_reservations_canister

- On this canister developed to run on the ICP (Internet Computer Protocol) are implemented functions and utilities written in Rust, Rust-CDK (Canister Development Kit *ic_cdk), other libraries, and a CRUD modal. Meant to assist manufacturing or wholesale companies in managing and serving available goods for reservations to their clients needing reservations on goods, with these reservations having a valid period of which they are paired with these clients, and depends on the client's response to the manufacturer or the wholesale company (i.e could be a payment process implemented on another canister) for this reservations to be served.

## Guide on Deploying and Testing the canister 🛠.

### Tech Stacks Required

* [node.js](https://nodejs.org/en/download). -v18 or >.

* Download the canister contract from the provided repository 
```bash
$ git clone https://github.com/lukrycyfa/warehouse-reservations-canister.git
```

* rustc 1.64 or higher
```bash
$ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
$ source "$HOME/.cargo/env"
```
* rust wasm32-unknown-unknown target
```bash
$ rustup target add wasm32-unknown-unknown
```
* candid-extractor
```bash
$ cargo install candid-extractor
```
* install `dfx`
```bash
$ DFX_VERSION=0.15.0 sh -ci "$(curl -fsSL https://sdk.dfinity.org/install.sh)"
$ echo 'export PATH="$PATH:$HOME/bin"' >> "$HOME/.bashrc"
$ source ~/.bashrc
```
* Restart your terminal if required

* You could optionally use Github Codespaces to test the project by clicking on the "Code" button, navigating to the codespaces tab then select "Create codespace on main". to generate a new Codespace, pre-configured with everything you need to start testing this project.

* Start The Internet Computer Replica
```bash
$ dfx start --background --clean
```

* You could create extra accounts to interact with the canister after deployment via terminal.

- Create a new identity
```bash
$ dfx identity new [OPTIONS] <NEW_IDENTITY>
```
- Select a prefered identity
```bash
$ dfx identity use [OPTIONS] <IDENTITY>
```

* Navigate into the canister cloned repository

```bash
$ cd warehouse-reservations-canister
$ npm run gen-deploy
```
* If you get permission errors after running `npm run gen-deploy`
.

```bash
$ cd warehouse-reservations-canister
$ chmod u+x did.sh or chmod +x did.sh 
```
if by any chance that you get error as :

note: to keep the current resolver, specify `workspace.resolver = "1"` in the workspace root's manifest
note: to use the edition 2021 resolver, specify `workspace.resolver = "2"` in the workspace root's manifest


you can fix it by following the instruction and adding the `resolver ="2"` to the workspace root's manifest it is in the file `Cargo.toml` eg:

        [workspace]
        members = [
            "src/warehouse_reservations_canister",
        ]
        resolver="2"

* After the canister is deployed on the internet computer replica, a link will be provided for you to interact with the canister via the candid interface provided.

### Example function Calls and Args on the Canister.

* Test via terminal

- Get products `(product_id, staff_id ) ~`

```bash
$ dfx canister call warehouse_reservations_canister get_product '(pid,sid)'
$ dfx canister call warehouse_reservations_canister get_products '()'
```
- Get staffs `(staff_id) ~`

```bash
$ dfx canister call warehouse_reservations_canister get_staff '(sid)'
$ dfx canister call warehouse_reservations_canister get_staffs '(sid)'
```
- Get reservations  `(reservation_id) / (staff_id ) ~`

```bash
$ dfx canister call warehouse_reservations_canister get_reservation '(rid)'
$ dfx canister call warehouse_reservations_canister get_reservations '(sid)'
```
- Get clients `(client_id) / (staff_id) ~`

```bash
$ dfx canister call warehouse_reservations_canister get_client '(cid)'
$ dfx canister call warehouse_reservations_canister get_clients '(sid)'
```
- Add a staff ~
 
```bash
$ dfx canister call warehouse_reservations_canister add_staff '( record {'first_name' = 'string'; 
'last_name' = 'string'; 'staff_address' = opt principal " "; })'
```
- Update a profile `(staff_id) ~`

```bash
$ dfx canister call warehouse_reservations_canister update_profile '( sid, record {'first_name' = "string"; 
'last_name' = "string"; 'staff_address' = opt principal " "; })'
```
- Delete a staff `(staff_id) ~`

```bash
$ dfx canister call warehouse_reservations_canister delete_staff '(sid)'
```
- Add a client ~

```bash
$ dfx canister call warehouse_reservations_canister add_client '(record {'first_name' = "string"; 'last_name' = "string";})'
```
- Update a client `(client_id) ~`

```bash
$ dfx canister call warehouse_reservations_canister update_client '( cid , record { 'first_name' = "string"; 'last_name' = "string";})'
```
- Delete a client `(client_id) ~`

```bash
$ dfx canister call warehouse_reservations_canister delete_client '(cid)'
```
- Add a product `(staff_id) ~`

```bash
$ dfx canister call warehouse_reservations_canister add_product '( sid, record { 'name'= "product name"; 'description' = "product description"; 'minimum_reservation' = 6; 'maximum_reservation' = 25; 'reservation_valid_duration' = 420000000000; 'price_per_item'= 90000; })'
```    
- Update a product `(product_id, staff_id) ~`

```bash
$ dfx canister call warehouse_reservations_canister update_product '( pid, sid, record { 'name'= "product name"; 'description' = "product description updated"; 'minimum_reservation' = 8; 'maximum_reservation' = 27; 'reservation_valid_duration' = 520000000000; 'price_per_item'= 90000; })'
```   

- Provision reservations `(product_id, staff_id) ~`

```bash
$ dfx canister call warehouse_reservations_canister provision_reservations '(pid, sid, record { 'reservations' = 95; })'
```
- Delete a product `(product_id, staff_id) ~`

```bash
$ dfx canister call warehouse_reservations_canister delete_product '(pid,sid)'
```
- Make reservations `(client_id, product_id) ~`

```bash
$ dfx canister call warehouse_reservations_canister make_reservations '( cid, pid, record { 'description' = "reservation description";
 'reserve' = 16; })'
``` 
- Update a reservation `(rservation_id, client_id) ~`

```bash
$ dfx canister call warehouse_reservations_canister update_reservation '( rid, cid, record { 'description' = "reservation description";
 'reserve' = 10; })'
``` 
- Cancel a reservation `(client_id, reservation_id) ~`

```bash
dfx canister call warehouse_reservations_canister cancel_reservation '(cid, rid)'
```
- Delete a reservation `(client_id, reservation_id) ~`

```bash
$ dfx canister call warehouse_reservations_canister delete_reservation '(cid, rid)'
```
- Serve a reservation `(client_id, reservation_id, staff_id) ~`

```bash
dfx canister call warehouse_reservations_canister serve_reservation '(cid, rid, sid)'
```


## Test Backend Canister via Candid Interface 

 After running the command `npm run gen-deploy` at the end of succesful Installation of Canisters.
 You will recieve a message showing:
 The UI canister on the "local" network is "xxxxxxxxxxxxxxxxxxxxx"
Installing code for canister warehouse_reservations_canister, with canister ID "yyyyyyyyyyyyyyyy"



Deployed canisters.
URLs:
Backend canister via Candid interface:
    Deployed canisters.
URLs:




 Backend canister via Candid interface:
    warehouse_reservations_canister: http://127.0.0.1:4943/?canisterId=xxxxx-xxxxx-xxxxx-xxxxx-xxx&id=yyyyy-yyyyy-yyyyy-yyyyy-yyy

```bash
Where  xxxxxxx is the local network CanisterId yyyyyyyy is the id
```
Copy the URL address and access it on your browser