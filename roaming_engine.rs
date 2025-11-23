
use std::process::Command;

pub fn activate(){
    println!("Joining ZeroTier network...");
    let _=Command::new("zerotier-cli").arg("join").arg("3efa5cb78a804998").spawn();

    println!("Configuring advanced roaming bridge...");
    let _=Command::new("bash").arg("-c").arg("ip link set zt0 up").spawn();
    let _=Command::new("bash").arg("-c").arg("brctl addbr roamx").spawn();
    let _=Command::new("bash").arg("-c").arg("brctl addif roamx zt0").spawn();

    println!("Enabling DHCP relay...");
    let _=Command::new("bash").arg("-c").arg("systemctl restart isc-dhcp-relay").spawn();

    println!("Enabling mDNS reflection...");
    let _=Command::new("bash").arg("-c").arg("systemctl restart avahi-daemon").spawn();

    println!("Enabling UDP broadcast repeater...");
    let _=Command::new("bash").arg("-c").arg("systemctl restart udp-broadcast-relay").spawn();

    println!("Roaming engine fully activated.");
}
