use std::collections::HashMap;

pub fn load_services() -> HashMap<u16, &'static str> {
    let mut services = HashMap::new();

    // Basic well-known ports
    services.insert(20, "ftp-data");
    services.insert(21, "ftp");
    services.insert(22, "ssh");
    services.insert(23, "telnet");
    services.insert(25, "smtp");
    services.insert(53, "dns");
    services.insert(67, "dhcp");
    services.insert(68, "dhcp");
    services.insert(69, "tftp");
    services.insert(80, "http");
    services.insert(110, "pop3");
    services.insert(111, "rpcbind");
    services.insert(123, "ntp");
    services.insert(135, "msrpc");
    services.insert(139, "netbios-ssn");
    services.insert(143, "imap");
    services.insert(161, "snmp");
    services.insert(389, "ldap");
    services.insert(443, "https");
    services.insert(445, "smb");
    services.insert(514, "syslog");
    services.insert(587, "smtp-sub");
    services.insert(631, "ipp");
    services.insert(873, "rsync");
    services.insert(989, "ftps-data");
    services.insert(990, "ftps");
    services.insert(1080, "socks");
    services.insert(1433, "mssql");
    services.insert(1521, "oracle");
    services.insert(2049, "nfs");
    services.insert(2375, "docker");
    services.insert(2376, "docker-secure");
    services.insert(27017, "mongodb");
    services.insert(3306, "mysql");
    services.insert(3389, "rdp");
    services.insert(5432, "postgres");
    services.insert(5900, "vnc");
    services.insert(6379, "redis");
    services.insert(8080, "http-alt");
    services.insert(8443, "https-alt");

    services
}
