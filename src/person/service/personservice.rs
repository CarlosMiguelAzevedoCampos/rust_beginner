use crate::person::model::person::Person;
use crate::person::repository::personrepository::PersonRepositoryService;
use crate::person::repository::personrepository::personRepository;

pub trait personService{
    fn add_person(person : Person) -> bool;   
}
pub struct PersonService;

impl personService for PersonService {
    fn add_person(value: Person) -> bool{
        if value.age > 10 {
            return false;
        }
        return <PersonRepositoryService as personRepository>::add_person(value);
    }
}