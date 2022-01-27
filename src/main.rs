use registry::{
    Hive,
    Security,
};

fn main() {

    let path = "SOFTWARE\\Microsoft\\Windows\\Windows Error Reporting\\LocalDumps";
    let hive = Hive::LocalMachine;
    if let Ok(_key) = hive.open(path, Security::Read) { 
        // Remove Key
        println!("Removing Key for crash dumps");
        hive.delete(path, false).unwrap();
    } else { 
        // Add key 
        println!("Adding key for crash dumps");
        hive.create(path, Security::Write).unwrap();
    }
}
