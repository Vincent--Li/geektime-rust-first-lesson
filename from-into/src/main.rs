use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

fn print(v: impl Into<IpAddr>) {
    println!("{:?}", v.into());
}

fn main() {
    let v4: Ipv4Addr = "2.2.2.2".parse().unwrap();
    let v6: Ipv6Addr = "::1".parse().unwrap();

    print([1,1,1,1]);
    print([0xfe80,0,0,0,0xaede,0x48ff,0xfe00,0x1122]);
    print(v4);
    print(v6);
}
