#[macro_use]
extern crate serde;
use candid::{Decode, Encode};
use ic_cdk::{api::{time, caller}, init};
use candid::Principal;
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{BoundedStorable, Cell, DefaultMemoryImpl, StableBTreeMap, Storable};
use std::{borrow::Cow, cell::{RefCell, Cell as OtherCell}};

// Defined Memory And IdCell Types. 
type Memory = VirtualMemory<DefaultMemoryImpl>;
type IdCell = Cell<u64, Memory>;

// A Struct for Creating an Instance Of a Staff in Memory
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct Staff {
    id: u64,
    staff_address: Option<Principal>,
    first_name: String,
    last_name: String,
    role: Option<Roles>,
    created_at: u64,
    updated_at: Option<u64>,
}

// A Struct for Creating an Instance Of a Product in Memory
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct Product {
    id: u64,
    name: String,
    description: String,
    reservations: u64,
    r_requests: Vec<Reservation>,
    reservations_requests: u64,
    minimum_reservation: u64,
    maximum_reservation: u64,
    reservation_valid_duration: u64,
    reservations_serve: u64,
    price_per_item: u64,
    created_at: u64,
    updated_at: Option<u64>,
}

// A Struct for Creating an Instance Of a Reservation in Memory
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct Reservation {
    id: u64,
    product_id: u64,
    client_address: Option<Principal>,
    client: String,
    description: String,
    reserve: u64,
    serve: bool,
    total_cost: u64,
    invalid_at: u64,
    valid_duration_at_creation: u64,
    created_at: u64,
    updated_at: Option<u64>,
}

// A Struct for Creating an Instance Of a Client in Memory
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct Client {
    id: u64,
    client_address: Option<Principal>,
    first_name: String,
    last_name: String,
    reservations: Vec<Reservation>,
    created_at: u64,
    updated_at: Option<u64>,
}

// Storable traits that must be implemented for the struct's stored in a stable struct
// Storable Trait for the Staff Struct
impl Storable for Staff {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }
    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

// Storable Trait for the Product Struct
impl Storable for Product {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }
    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

// Storable Trait for the Reservation Struct
impl Storable for Reservation {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }
    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

// Storable Trait for the Client Struct
impl Storable for Client {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }
    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

// BoundedStorable traits that must be implemented for the struct's stored in a stable struct
// BoundedStorable Trait for the Staff Struct
impl BoundedStorable for Staff {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

// BoundedStorable Trait for the Product Struct
impl BoundedStorable for Product {
    const MAX_SIZE: u32 = 4096;
    const IS_FIXED_SIZE: bool = false;
}

// BoundedStorable Trait for the Reservation Struct
impl BoundedStorable for Reservation {
    const MAX_SIZE: u32 = 2048;
    const IS_FIXED_SIZE: bool = false;
}

// BoundedStorable Trait for the Client Struct
impl BoundedStorable for Client {
    const MAX_SIZE: u32 = 4096;
    const IS_FIXED_SIZE: bool = false;
}


// Initiate thread_local variables
thread_local! {
    
    //thread_local variable for the MEMORY_MANAGER
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
        MemoryManager::init(DefaultMemoryImpl::default()));

    //thread_local variable for the canister OWNER    
    static OWNER: OtherCell<Principal> = OtherCell::new(Principal::from_slice(&[]));

    //thread_local variable for the ID_COUNTER
    static ID_COUNTER: RefCell<IdCell> = RefCell::new(
        IdCell::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))), 0)
            .expect("Cannot create a counter"));
    
    //thread_local variable for the STAFF_STORAGE       
    static STAFF_STORAGE: RefCell<StableBTreeMap<u64, Staff, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1)))));
    
    //thread_local variable for the PRODUCT_STORAGE
    static PRODUCT_STORAGE: RefCell<StableBTreeMap<u64, Product, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(2)))));
    
    //thread_local variable for the RESERVATION_STORAGE        
    static RESERVATION_STORAGE: RefCell<StableBTreeMap<u64, Reservation, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(3)))));
    
    //thread_local variable for the CLIENT_STORAGE        
    static CLIENT_STORAGE: RefCell<StableBTreeMap<u64, Client, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(4)))));

}

// A Struct for the Payload i.e details for a Staff
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct StaffPayload {
    first_name: String,
    last_name: String,
    staff_address: Option<Principal>,
}

// A Struct for the Payload i.e details for a Product
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct ProductPayload {
    name: String,
    description: String,
    minimum_reservation: u64,
    maximum_reservation: u64,
    reservation_valid_duration: u64,
    price_per_item: u64,
}

// A Struct for the Payload i.e details for provisioning reservations
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct ProvisionPayload {
    reservations: u64,
}

// A Struct for the Payload i.e details for a Reservation
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct ReservationPayload {
    description: String,
    reserve: u64,
}

// A Struct for the Payload i.e details for a Client
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct ClientPayload {
    first_name: String,
    last_name: String,
}



#[init]
fn init() {     
    OWNER.with(|s| s.set(caller())); 
}
//The above function is called when the canister is deployed, and it assings the caller() to the OWNER variable

// //checked
// #[ic_cdk::query]
// fn get_caller() -> Principal {
//     caller()
// }

// //checked
// #[ic_cdk::query]
// fn get_owner() -> Principal {
//     OWNER.with(|s| s.get())
// }

//Returns a product associated with the parsed Id to the caller()
#[ic_cdk::query]
fn get_product(id: u64, sid: u64) -> Result<Product, Error> {
    // make validations with the parsed Id's and caller
    // clear all invalid requests i.e outdated requests associated with the product
    // to get updated state of the product and returns the product from memory if present.
    match _get_staff(&sid) {
        Some(staff) => {
            let owner = OWNER.with(|s| s.get());
            let _ = _ifvalid(&(staff.staff_address == Some(caller()) || owner == caller()), "Unauthorized Operation For Caller".to_string())?;
            clear_invalid_requests(&id)?;
            match _get_product(&id) {
                Some(product) => {
                    Ok(product)},
                None => Err(Error::NotFound {
                    msg: format!("a product with id={} not found", id),
                }),
            }
        }    
        None => Err(Error::NotFound {
            msg: format!("a staff with id={} not found", sid),
        }),
    }

}

// Returns all products available on the canister 
#[ic_cdk::query]
fn get_products() -> Vec<(u64, Product)> {
    // iter through available, clear invalid reservations and return products 
    let mut _v: Vec<(u64, Product)> = vec![];
    PRODUCT_STORAGE.with(|service| for  mut pro in service.borrow().iter().enumerate(){
        for pr in pro.1.1.r_requests {
            if pr.invalid_at < time() && !pr.serve {
                pro.1.1.reservations_requests -= pr.reserve;
                pro.1.1.reservations += pr.reserve;
            } 
        }
        // assign an empty vec of request to the product to have a view of only product details.
        pro.1.1.r_requests = vec![];
        _v.push(pro.1);
    });
    _v
}

//Returns a staff associated with the parsed Id to the caller()
#[ic_cdk::query]
fn get_staff(sid: u64) -> Result<Staff, Error> {
    // make validations with the parsed Id and caller, and finally return a staff from memory
    match _get_staff(&sid) {
        Some(staff) => {
            let owner = OWNER.with(|s| s.get());
            let ___ = _ifvalid(&(staff.staff_address == Some(caller()) || owner == caller()), "Unauthorized Operation For Caller".to_string())?;
            Ok(staff)},
        None => Err(Error::NotFound {
            msg: format!("a staff with id={} not found", sid),
        }),
    }
}

// Returns all staffs available on the canister 
#[ic_cdk::query]
fn get_staffs(sid: u64) -> Result<Vec<(u64, Staff)>, Error>{
    // make validations with the parsed Id and caller, and finally return all staffs from memory
    match _get_staff(&sid) {
        Some(staff) => {
            let owner = OWNER.with(|s| s.get());
            let ___ = _ifvalid(&(staff.staff_address == Some(caller()) || owner == caller()), "Unauthorized Operation For Caller".to_string())?;
            Ok(STAFF_STORAGE.with(|service| service.borrow().iter().collect::<Vec<(u64, Staff)>>()))},
        None => Err(Error::NotFound {
            msg: format!("a staff with id={} not found", sid),
        }),
    }
}

//Returns a reservation associated with the parsed Id to the caller()
#[ic_cdk::query]
fn get_reservation(rid: u64) -> Result<Reservation, Error> {
    // make validations with the parsed Id and caller, and finally return a reservation from memory
    match _get_reservation(&rid) {
        Some(reservation) => {
            let _ = _ifvalid(&(reservation.client_address == Some(caller())), "Unauthorized Operation For Caller".to_string())?;
            Ok(reservation)},
        None => Err(Error::NotFound {
            msg: format!("a reservation with id={} not found", rid),
        }),
    }
}

// Returns all reservations available on the canister
#[ic_cdk::query]
fn get_reservations(sid: u64) -> Result<Vec<(u64, Reservation)>, Error>{
    // make validations with the parsed Id and caller, and finally return all reservations from memory
    match _get_staff(&sid) {
        Some(staff) => {
            let owner = OWNER.with(|s| s.get());
            let ___ = _ifvalid(&(staff.staff_address == Some(caller()) || owner == caller()), "Unauthorized Operation For Caller".to_string())?;
            Ok(RESERVATION_STORAGE.with(|service| service.borrow().iter().collect::<Vec<(u64, Reservation)>>()))},
        None => Err(Error::NotFound {
            msg: format!("a staff with id={} not found", sid),
        }),
    }
}

//Returns a client associated with the parsed Id to the caller()
#[ic_cdk::query]
fn get_client(cid: u64) -> Result<Client, Error> {
    // make validations with the parsed Id and caller, and finally return a client from memory
    match _get_client(&cid) {
        Some(client) => {
            let _ = _ifvalid(&(client.client_address == Some(caller())), "Unauthorized Operation For Caller".to_string())?;
            Ok(client)},
        None => Err(Error::NotFound {
            msg: format!("a client with id={} not found", cid),
        }),
    }
}

// Returns all clients available on the canister
#[ic_cdk::query]
fn get_clients(sid: u64) -> Result<Vec<(u64, Client)>, Error>{
    // make validations with the parsed Id and caller, and finally return all clients from memory
    match _get_staff(&sid) {
        Some(staff) => {
            let owner = OWNER.with(|s| s.get());
            let ___ = _ifvalid(&(staff.staff_address == Some(caller()) || owner == caller()), "Unauthorized Operation For Caller".to_string())?;
            Ok(CLIENT_STORAGE.with(|service| service.borrow().iter().collect::<Vec<(u64, Client)>>()))},
        None => Err(Error::NotFound {
            msg: format!("a staff with id={} not found", sid),
        }),
    }
}

//This function is called to create a staff on the canister
#[ic_cdk::update]
fn add_staff(staff: StaffPayload) -> Result<Staff, Error> {
    // Retrive the owners principal from storage, validate if the caller() is the owner and inputs
    // finally create a staff with the payload.
    let owner = OWNER.with(|s| s.get());
    let _ = _ifvalid(&(owner == caller()), "Unauthorized Operation For Caller".to_string())?;
    let __ = _ifvalid(&(staff.staff_address.is_some() && staff.first_name.len() > 0 &&
        staff.last_name.len() > 0), "Invalid Input Provided".to_string())?; 
    let staffrole = Roles::Specialist;
    match add_profile(staff, staffrole) {
        Some(staff) => Ok(staff),
        None => Err(Error::NotFound {
            msg: format!("could not create a staff")}),
    }
}

//Helper function to create a staff on the caniter
fn add_profile(staff: StaffPayload, role:Roles) -> Option<Staff> {
    // increment the id counter to get an id 
    let id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)})
        .expect("cannot increment id counter");
    // create and save an instance of a staff in the staff memory.    
    let staff = Staff {
        id, staff_address: staff.staff_address, first_name: staff.first_name,
        last_name: staff.last_name, role: Some(role), created_at: time(),
        updated_at: None};
    do_insert_staff(&staff);
    Some(staff)
}

//This funtion is called to create a client on the canister
#[ic_cdk::update]
fn add_client(client: ClientPayload) -> Result<Client, Error> {
    // increment the id counter to get an id and validate inputs
    let __ = _ifvalid(&(client.first_name.len() > 0 && client.last_name.len() > 0), "Invalid Input Provided".to_string())?;
    let id = ID_COUNTER.with(|counter| {
        let current_value = *counter.borrow().get();
        counter.borrow_mut().set(current_value + 1)}).expect("cannot increment id counter");
    // create and save an instance of a client in the client memory.     
    let client = Client {
        id, client_address: Some(caller()), first_name: client.first_name, reservations: vec![],
        last_name: client.last_name, created_at: time(), updated_at: None};
    do_insert_client(&client);
    Ok(client)
}

//This function is called by a staff to create a product on the canister
#[ic_cdk::update]
fn add_product(sid: u64, product: ProductPayload) -> Result<Product, Error> {
    // first look-up the memory for the presence of a staff with the parsed id and if present
    // validate the caller is the staff, validate inputs, increment the id counter to get an id for the product,
    // create and save an instance of the product in the product memory.
    match _get_staff(&sid) {
        Some(staff) => {
            let _ = _ifvalid(&(staff.staff_address == Some(caller())), "Unauthorized Operation For Caller".to_string())?;
            let _ = _ifvalid(&(product.name.len() > 0 && product.description.len() > 0 && product.minimum_reservation > 0  && 
            product.maximum_reservation > product.minimum_reservation && product.price_per_item > 0 && 
            product.reservation_valid_duration > 0), "Invalid Input Provided".to_string())?;
            let id = ID_COUNTER
                .with(|counter| {
                    let current_value = *counter.borrow().get();
                    counter.borrow_mut().set(current_value + 1)})
                .expect("cannot increment id counter");
            let product = Product {
                id, name: product.name, description: product.description, minimum_reservation: product.minimum_reservation,
                maximum_reservation: product.maximum_reservation, reservations: 0, reservations_requests: 0, 
                r_requests: vec![], reservations_serve: 0, price_per_item : product.price_per_item,
                reservation_valid_duration: product.reservation_valid_duration, created_at: time(),
                updated_at: None, };
            do_insert_product(&product);
            Ok(product)
        } 
        None => Err(Error::NotFound {
            msg: format!("a staff with id={} not found", sid)}),
    }
}

//This function is called by a staff to make provisionings for reservations on products on the canister.
#[ic_cdk::update]
fn provision_reservations(pid: u64, sid: u64, payload: ProvisionPayload) -> Result<Product, Error> {
    // first look-up the memory for the presence of a staff and product with the parsed id's and if present
    // make validations on the caller and payload. finally provision reservations and update the product in memory.
    match _get_staff(&sid) {
        Some(staff) => {
            let  _ = _ifvalid(&(staff.staff_address == Some(caller())), "Unauthorized Operation For Caller".to_string())?;
            let __ = _ifvalid(&(payload.reservations > 0), "Insufficent Payload For Reservations".to_string())?;
            match _get_product(&pid) {                
                Some(mut product) =>{
                        product.reservations +=  payload.reservations;
                        product.updated_at = Some(time());
                        do_insert_product(&product);
                        Ok(product)
                }
                None => Err(Error::NotFound {
                    msg: format!("couldn't update a product with id={}. not found", pid)}),
            } 
        }
        None => Err(Error::NotFound {
            msg: format!("couldn't access a staff with id={}. not found",sid)}),
    }
}

//this function is called by a client to make reservations for products on the canister
#[ic_cdk::update]
fn make_reservations(clid: u64, pid: u64, payload: ReservationPayload) -> Result<Reservation, Error> {
    // first look-up the memory for the presence of a client and product with the parsed id's and if present
    // validate the caller is the client, and make other validations and then increment the id counter to get an id for the
    // reservation. Then create and save an instance of the reservation in the reservation memory and make updates.
    // if all checks are passed
    match _get_client(&clid) {
        Some(mut client) => {
            let ___ = _ifvalid(&(client.client_address == Some(caller())), "Unauthorized Operation For Caller".to_string())?;
            clear_invalid_requests(&pid)?;
            match _get_product(&pid) {
                Some(mut product) => {
                   let _ = _ifvalid(&(product.maximum_reservation >= payload.reserve && payload.reserve >= product.minimum_reservation 
                    && payload.description.len() > 0),"Invalid Payload For Reservations".to_string())?;
                   let __ = _ifvalid(&(product.reservations - payload.reserve > 0),
                        "Insufficent Product In Reserve For Reservations".to_string())?;
                    let id = ID_COUNTER.with(|counter| {let current_value = *counter.borrow().get();
                        counter.borrow_mut().set(current_value + 1)}).expect("cannot increment id counter");
                    let reservation = Reservation {id, product_id: product.id, client_address: client.client_address,
                        client: format!("{}, {}", client.first_name, client.last_name),                   
                        description: payload.description, reserve: payload.reserve,
                        serve: false, invalid_at: (time() + product.reservation_valid_duration),
                        valid_duration_at_creation: product.reservation_valid_duration,
                        total_cost: product.price_per_item * payload.reserve, created_at: time(), updated_at: None                
                    };
                    product.reservations -= payload.reserve;
                    product.reservations_requests += payload.reserve;
                    product.r_requests.push(reservation.clone()); 
                    product.updated_at = Some(time());                    
                    client.reservations.push(reservation.clone());
                    client.updated_at = Some(time());
                    do_insert_reservation(&reservation);
                    do_insert_product(&product);
                    do_insert_client(&client);
                    Ok(reservation)
                }        
                None => Err(Error::NotFound {        
                    msg: format!("a product with id={} not found", pid)}),        
            }
        } 
        None => Err(Error::NotFound {
            msg: format!("a client with id={} not found", clid)}),
    }
}

//This function is called by a staff to serve valid reservations on the canister.
#[ic_cdk::update]
fn serve_reservation(clid: u64, rid: u64, sid: u64) -> Result<Reservation, Error> {
    // first look-up the memory for the presence of a client, reservation and staff with the parsed id's and if present
    // make validations on the caller and reservation and finally serve the reservation and make updates if all checks are passed.
    match _get_staff(&sid) {        
        Some(staff) => {
            let _ = _ifvalid(&(staff.staff_address == Some(caller())), "Unauthorized Operation For Caller".to_string())?;
            match _get_client(&clid) {
                Some(mut client) => {
                    let idx = client.reservations.iter().position(|rev| rev.id == rid);
                    let __ = _ifvalid(&(idx.is_some()), "Invalid Reservation For the Client".to_string())?;
                    match _get_reservation(&rid) {    
                        Some(mut reservation) => {
                           let __ = _ifvalid(&(!reservation.serve && reservation.invalid_at >= time()), "Reservation Has been Attended To Or Oudated".to_string())?;
                            match _get_product(&reservation.product_id){
                                Some(mut product) => {
                                    reservation.serve = true;
                                    reservation.updated_at = Some(time());
                                    product.reservations_serve  += reservation.reserve;
                                    product.reservations_requests -= reservation.reserve;
                                    let idx = product.r_requests.iter().position(|rev| rev.id == reservation.id);
                                    match idx {
                                        Some(va) => { product.r_requests.remove(va);
                                                product.r_requests.insert(va, reservation.clone())
                                        },
                                        None => ()};
                                    let cidx: Option<usize> = client.reservations.iter().position(|rev| rev.id == rid);
                                    match cidx {
                                        Some(va) => {client.reservations.remove(va);
                                            client.reservations.insert(va, reservation.clone());},
                                        None => ()};
                                    product.updated_at = Some(time());
                                    client.updated_at = Some(time());
                                    do_insert_client(&client);    
                                    do_insert_reservation(&reservation);
                                    do_insert_product(&product);
                                    Ok(reservation)
                                }
                                None => Err(Error::NotFound {        
                                    msg: format!("a product with id={} not found", reservation.product_id)}),
                            }
                        }
                        None => Err(Error::NotFound {        
                            msg: format!("a reservation with id={} not found", rid)}),        
                    }
                } 
                None => Err(Error::NotFound {
                    msg: format!("a client with id={} not found", clid)})
            }
        }
        None => Err(Error::NotFound {
            msg: format!("a staff with id={} not found", sid)})
    }
}

//this function is called by a staff to update a profile on the canister.
#[ic_cdk::update]
fn update_profile(id: u64, payload: StaffPayload) -> Result<Staff, Error> {
    // first look-up the memory for the presence of a staff with the parsed id and if present
    // make validations on the caller, inputs and finally update the profile if all checks are passed.
    match _get_staff(&id) {
        Some(mut staff) => {
            let _ = _ifvalid(&(Some(caller()) == staff.staff_address), "Unauthorized Operation For Caller".to_string())?;
            let __ = _ifvalid(&(payload.staff_address.is_some() && payload.first_name.len() > 0 &&
            payload.last_name.len() > 0), "Invalid Input Provided".to_string())?;
            staff.first_name = payload.first_name;
            staff.last_name = payload.last_name;
            staff.staff_address = payload.staff_address;
            staff.updated_at = Some(time());
            do_insert_staff(&staff);
            Ok(staff)
        }
        None => Err(Error::NotFound {
            msg: format!("a staff with id={} not found", id)}),
    }
}

//This function is called by a client to update a profile
#[ic_cdk::update]
fn update_client(id: u64, payload: ClientPayload) -> Result<Client, Error> {
    // first look-up the memory for the presence of a client with the parsed id and if present
    // make validations on the caller, inputs and finally update the client if all checks are passed.
    match _get_client(&id) {
        Some(mut client) => {
            let _ = _ifvalid(&(client.client_address == Some(caller())), "Unauthorized Operation For Caller".to_string())?;
            let __ = _ifvalid(&(payload.first_name.len() > 0 && payload.last_name.len() > 0), "Invalid Input Provided".to_string())?;
            client.first_name = payload.first_name;
            client.last_name = payload.last_name;
            client.updated_at = Some(time());
            do_insert_client(&client);
            Ok(client)
        }
        None => Err(Error::NotFound {
            msg: format!("a client with id={} not found", id)}),
    }
}

//This function is called by a staff to make updates to existing products on the canister.
#[ic_cdk::update]
fn update_product(pid: u64, sid: u64, payload: ProductPayload) -> Result<Product, Error> {
    // first look-up the memory for the presence of a staff and product with the parsed id's and if present
    // make validations on the caller, inputs and finally update the product in memory.
    match _get_staff(&sid) {
        Some(staff) => {
            let _ = _ifvalid(&(staff.staff_address == Some(caller())), "Unauthorized Operation For Caller".to_string())?;
            let _ = _ifvalid(&(payload.name.len() > 0 && payload.description.len() > 0 && payload.minimum_reservation > 0  && 
            payload.maximum_reservation > payload.minimum_reservation && payload.price_per_item > 0 && 
            payload.reservation_valid_duration > 0), "Invalid Input Provided".to_string())?;
            match _get_product(&pid) {
                Some(mut product) =>{
                        product.name = payload.name;
                        product.description = payload.description;
                        product.minimum_reservation = payload.minimum_reservation;
                        product.maximum_reservation = payload.maximum_reservation;
                        product.reservation_valid_duration = payload.reservation_valid_duration;
                        product.price_per_item = payload.price_per_item;
                        product.updated_at = Some(time());
                        do_insert_product(&product);
                        Ok(product)
                }
                None => Err(Error::NotFound {
                    msg: format!("couldn't update a product with id={}. not found",pid)}),
            } 
        }
        None => Err(Error::NotFound {
            msg: format!("a staff with id={} not found",sid)}),
    }
}

//This function is called by a client to update valid reservations on the canister.
#[ic_cdk::update]
fn update_reservation(rid: u64, cid: u64, payload: ReservationPayload) -> Result<Reservation, Error> {
    // first look-up the memory for the presence of a client and reservation with the parsed id's and if present
    // make validations on the caller, inputs and reservation and finally update the reservation, client and product in memory.
    // if all checks are passed 
    match _get_client(&cid) {
        Some(mut client) => {
            match _get_reservation(&rid) {
                Some(mut reservation) => {
                    let _ = _ifvalid(&(Some(caller()) == reservation.client_address), "Unauthorized Operation For Caller".to_string())?;
                    let __ = _ifvalid(&(reservation.invalid_at > time() && !reservation.serve), "Reservation Outdated Or Has been Proccessed".to_string())?;
                    clear_invalid_requests(&reservation.product_id)?;
                    match _get_product(&reservation.product_id){
                        Some(mut product) => {
                            let ___ = _ifvalid(&(product.maximum_reservation >= payload.reserve && payload.reserve >= product.minimum_reservation
                            && payload.description.len() > 0),"Invalid Payload For Reservations".to_string())?;
                            let ____ = _ifvalid(&((product.reservations + reservation.reserve) - payload.reserve > 0 ),
                            "Insufficent Product In Reserve For Reservations".to_string())?;
                            product.reservations  = product.reservations + reservation.reserve - payload.reserve;
                            product.reservations_requests = product.reservations_requests - reservation.reserve + payload.reserve;
                            reservation.reserve = payload.reserve;
                            reservation.total_cost = product.price_per_item * payload.reserve;
                            reservation.description = payload.description;
                            reservation.updated_at = Some(time());
                            let idx = product.r_requests.iter().position(|rev| rev.id == reservation.id);
                            match idx {
                                Some(va) => { product.r_requests.remove(va);
                                            product.r_requests.insert(va, reservation.clone())},
                                None => (),
                            };
                            let cidx: Option<usize> = client.reservations.iter().position(|rev| rev.id == rid);
                            match cidx {
                                Some(va) => {client.reservations.remove(va);
                                    client.reservations.insert(va, reservation.clone());},
                                None => (),
                            };
                            product.updated_at = Some(time());
                            client.updated_at = Some(time());
                            do_insert_client(&client);
                            do_insert_product(&product);
                            do_insert_reservation(&reservation);
                            Ok(reservation)
                        }
                        None => Err(Error::NotFound {        
                            msg: format!("a product with id={} not found", reservation.product_id)}),
                    }    
                }
                None => Err(Error::NotFound {
                    msg: format!("couldn't update a reservation with id={}. not found",rid)}),
            }
        }    
        None => Err(Error::NotFound {
            msg: format!("couldn't find a client with id={}. not found", cid)})    
    }
}

//This function is called by a staff or the canister owner to delete a profile from the Canister.
#[ic_cdk::update]
fn delete_staff(sid: u64) -> Result<Staff, Error> {
    // first look-up the memory for the presence of a staff with the parsed id and if present
    // make validations on the caller. and finally remove the staff from memory.
    match _get_staff(&sid) {
        Some(staff) => {
            let owner = OWNER.with(|s| s.get());
            let _ = _ifvalid(&(owner == caller() || staff.staff_address == Some(caller())), "Unauthorized Operation For Caller".to_string())?;
            STAFF_STORAGE.with(|service| service.borrow_mut().remove(&sid));
            Ok(staff)
        }
        None => Err(Error::NotFound {
            msg: format!("couldn't delete a staff with id={}. staff not found.", sid),})
        }

}

//This function is called by a client to delete a profile from the canister.
#[ic_cdk::update]
fn delete_client(cid: u64) -> Result<Client, Error> {
    // first look-up the memory for the presence of a client with the parsed id and if present
    // make validations on the caller, and finally remove the client from memory.
    match _get_client(&cid) {
        Some(client) => {
            let _ = _ifvalid(&(client.client_address == Some(caller())), "Unauthorized Operation For Caller".to_string())?;
            for rev in &client.reservations {
                if !rev.serve && rev.invalid_at > time() {
                    match _get_product(&rev.product_id) {
                        Some (mut product) =>{
                            let idx = product.r_requests.iter().position(|rv| rv.id == rev.id);
                            match idx {
                                Some(va) => { product.r_requests.remove(va);},
                                None => (),
                            };
                            product.reservations += rev.reserve;
                            product.reservations_requests -= rev.reserve;
                            do_insert_product(&product);
                        }
                        None => ()
                    }
                }
                RESERVATION_STORAGE.with(|service| service.borrow_mut().remove(&rev.id));
            } 
            CLIENT_STORAGE.with(|service| service.borrow_mut().remove(&cid));
            Ok(client)
        },
        None => Err(Error::NotFound {
            msg: format!("couldn't delete a client with id={}. not found.", cid)}),
    }
}

//this function is called by a staff to delete products from the canister.
#[ic_cdk::update]
fn delete_product(pid: u64, sid:u64) -> Result<Product, Error> {
    // first look-up the memory for the presence of a staff and product with the parsed id's and if present
    // make validations on the caller. then look-up the product for any existing valid reservation   
    // and finally remove the product from memory if there are none.
    match _get_staff(&sid) {
        Some(staff) => {
            let _ = _ifvalid(&(staff.staff_address == Some(caller())), "Unauthorized Operation For Caller".to_string())?;
            match _get_product(&pid) {
                Some (product) =>{
                    let idx = product.r_requests.iter().position(|rev| !rev.serve && rev.invalid_at > time());
                    let __ = _ifvalid(&(!idx.is_some()), "This Product Still Holds a Valid Reservation".to_string())?;
                    PRODUCT_STORAGE.with(|service| service.borrow_mut().remove(&pid));
                    Ok(product)
                }
                None => Err(Error::NotFound {
                    msg: format!("couldn't delete a product with id={}. product not found.", pid),}),
            }            
        }        
        None => Err(Error::NotFound {
            msg: format!("couldn't find a staff with id={}. staff not found", sid)}),
    }
}

//This function is called by a client to cancle valid reservation on the canister.
#[ic_cdk::update]
fn cancel_reservation(clid: u64, rid: u64) -> Result<Reservation, Error> {
    match _get_client(&clid) {
        // first look-up the memory for the presence of a client and reservation with the parsed id's and if present
        // make validations on the caller, reservation and finally remove the reservation from memory and make updates.
        // if all checks are passed.
        Some(mut client) => {
            let _ = _ifvalid(&(Some(caller()) == client.client_address), "Unauthorized Operation For Caller".to_string())?; 
           let idx = client.reservations.iter().position(|rev| rev.id == rid);
           let __ = _ifvalid(&(idx.is_some()), "Invalid Reservation For Caller".to_string())?;
            match _get_reservation(&rid) {
                Some(reservation) => {
                    let ___ = _ifvalid(&(reservation.client_address == client.client_address), "Unauthorized Operation For Caller".to_string())?;
                    let ____ = _ifvalid(&(!reservation.serve && reservation.invalid_at > time()),
                     "Reservation has been attended to or outdated consider deleting".to_string())?;                    
                    match _get_product(&reservation.product_id){
                        Some(mut product) => {
                            let idx: Option<usize> = product.r_requests.iter().position(|rev| rev.id == rid);
                            let mut _idx: usize;
                            if idx.is_some(){_idx = match idx {None => 0, Some(va) => va};
                                product.r_requests.remove(_idx);
                            }
                            let cidx: Option<usize> = client.reservations.iter().position(|rev| rev.id == rid);
                            let mut _cidx: usize;
                            if cidx.is_some(){_cidx = match cidx {None => 0, Some(va) => va};
                                client.reservations.remove(_cidx);
                            }                                                            
                            product.reservations += reservation.reserve;
                            product.reservations_requests -= reservation.reserve;
                            client.updated_at = Some(time());
                            product.updated_at = Some(time());
                            do_insert_product(&product);
                            do_insert_client(&client);
                            RESERVATION_STORAGE.with(|service| service.borrow_mut().remove(&rid)); 
                            Ok(reservation)
                        }
                        None => Err(Error::NotFound {        
                            msg: format!("a product with id={} not found", reservation.product_id)}),
                    }
                }
                None => Err(Error::NotFound {        
                    msg: format!("a reservation with id={} not found", rid)}),        
            }
        }
        None => Err(Error::NotFound {
            msg: format!("a client with id={} not found", clid)})
    }
}

//This function is called by a client to delete reservations from the canister.
#[ic_cdk::update]
fn delete_reservation(cid: u64, rid: u64) -> Result<Reservation, Error> {
    // first look-up the memory for the presence of a client and reservation with the parsed id's and if present
    // make validations on the caller. then look-up the reservation if it's been serve or invalid    
    // and finally remove the reservation from memory if so and make updates.
    match _get_client(&cid) {
        Some(mut client) => {            
            match _get_reservation(&rid) {
                Some(reservation) => {
                    let __ = _ifvalid(&(reservation.client_address == Some(caller())), "Unauthorized Operation for Caller()".to_string())?;
                    let ___ = _ifvalid(&(reservation.serve || reservation.invalid_at < time()), 
                        "This Reservation Is Still Valid Consider Cancling The Reservation".to_string())?;
                    let cidx: Option<usize> = client.reservations.iter().position(|rev| rev.id == rid);
                    let mut _cidx: usize;
                    if cidx.is_some(){_cidx = match cidx {None => 0, Some(va) => va};
                        client.reservations.remove(_cidx);
                    }
                    do_insert_client(&client);
                    RESERVATION_STORAGE.with(|service| service.borrow_mut().remove(&rid));                       
                    Ok(reservation)                           
                },
                None => Err(Error::NotFound {
                    msg: format!("couldn't find a reservation with id={}. reservation not found.", rid)}),
            }
            },
        None => Err(Error::NotFound {
            msg: format!("couldn't find a client with id={}. client not found.",cid),}),
    }
}

//An Enum for Error Messages
#[derive(candid::CandidType, Deserialize, Serialize)]
enum Error {
    NotFound { msg: String },
    InvalidValidation { msg: String},
}

//An Enum For a Staff Role
#[derive(candid::CandidType, Deserialize, Clone, Serialize, PartialEq)]
enum Roles { Specialist }

//A helper function to clear invalid requests and update state to a product, when getting the product, making reservation to
// or updating reservation the product
fn clear_invalid_requests(id: &u64) -> Result<(), Error> {
    // first look-up the memory for the presence of a product with the parsed id and if present
    // loop throug the reservations, remove invalid reservations and update the product.
    match _get_product(&id) {
        Some(mut product) => {
            let mut newvec: Vec<Reservation> = vec![];
            for req in product.r_requests {
                if req.invalid_at < time() && !req.serve {
                    product.reservations_requests -= req.reserve;
                    product.reservations += req.reserve;
                } else {
                    newvec.push(req.clone());
                }
            }
            product.r_requests = newvec;
            product.updated_at = Some(time());
            do_insert_product(&product);
            Ok(())},
        None => Err(Error::NotFound {
            msg: format!("a product with id={} not found", id),
        }),
    }
}

//Helper function to insert a staff into memory
fn do_insert_staff(staff: &Staff) {
    STAFF_STORAGE.with(|service| service.borrow_mut().insert(staff.id, staff.clone()));
}

//Helper function to insert a product into memory
fn do_insert_product(product: &Product) {
    PRODUCT_STORAGE.with(|service| service.borrow_mut().insert(product.id, product.clone()));
}

//Helper function to insert a reservation into memory
fn do_insert_reservation(reservation: &Reservation) {
    RESERVATION_STORAGE.with(|service| service.borrow_mut().insert(reservation.id, reservation.clone()));
}

//Helper function to insert a client into memory
fn do_insert_client(client: &Client) {
    CLIENT_STORAGE.with(|service| service.borrow_mut().insert(client.id, client.clone()));
}

//Helper function to make validations
fn _ifvalid(valid: &bool, message: String) -> Result<(), Error> {
    if *valid { Ok(())
    } else { Err(Error::InvalidValidation {msg:message }) }
}

//Helper function to get a product from memory
fn _get_product(id: &u64) -> Option<Product> {
    PRODUCT_STORAGE.with(|service| service.borrow().get(id))
}

//Helper function to get a reservation from memory
fn _get_reservation(id: &u64) -> Option<Reservation> {
    RESERVATION_STORAGE.with(|service| service.borrow().get(id))
}

//Helper function to get a client from memory
fn _get_client(id: &u64) -> Option<Client> {
    CLIENT_STORAGE.with(|service| service.borrow().get(id))
}

//Helper function to get a staff from memory
fn _get_staff(id: &u64) -> Option<Staff> {
    STAFF_STORAGE.with(|service| service.borrow().get(id))
}

// need this to generate candid
ic_cdk::export_candid!();