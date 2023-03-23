struct Employee{
    emp_id: u16,
    emp_name: String
}
impl Employee {
    fn show_details(&self) {
        println!("Employee id is {}", self.emp_id);
        println!("Employee name is {}", self.emp_name);
    }
    fn mut_show_employee_name(&mut self, new_name:String){
        self.emp_name = new_name;
        println!("Employee new name is {}", self.emp_name );
    }
    fn create_employee(id: u16, name: String) -> Employee{
        Employee { emp_id: id, emp_name: name }
    }
}
fn main() {
    let em1 = Employee{emp_id:1, emp_name: "amole".to_string()};
    println!("{:?}", em1.show_details());
    let employee = Employee::create_employee(22, "ayesha".to_string());
    employee.show_details();

}
