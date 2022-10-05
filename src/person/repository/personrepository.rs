use crate::person::model::person::Person;


pub trait personRepository{
    fn add_person(person : Person) -> bool;   
}
pub struct PersonRepositoryService;


impl personRepository for PersonRepositoryService {
    fn add_person(person : Person) -> bool { return true; }
}