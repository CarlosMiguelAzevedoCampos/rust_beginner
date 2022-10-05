mod person;


use person::{model};

use crate::person::service::personservice::personService;
use crate::person::service::personservice::PersonService;


fn main(){

    let player = model::person::Person{
        name : String::from("Carlos"),
        age : 20, 
        city : String::from("Trofa"),
    };
    let result = <PersonService as personService>::add_person(player);

    println!("{}", result);
}