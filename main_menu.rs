
use crossterm::event::{read,Event,KeyCode};

pub fn main_menu(){
    loop{
        println!("
============ PTUI ULTRA BEYOND ============
1) Full Roaming Engine
2) PCI Engine (Rebind + DB)
q) Quit
");
        if let Event::Key(k)=read().unwrap(){
            match k.code{
                KeyCode::Char('1')=>crate::ui::roaming_menu::roaming_menu(),
                KeyCode::Char('2')=>crate::ui::pci_menu::pci_menu(),
                KeyCode::Char('q')=>return,
                _=>{}
            }
        }
    }
}
