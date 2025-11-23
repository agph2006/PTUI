
pub fn pci_menu(){
    println!("Rebinding PCI devices...");
    crate::modules::pci_engine::rebind_all();
}
