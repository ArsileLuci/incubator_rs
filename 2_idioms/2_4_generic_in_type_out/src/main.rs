use std::net::{IpAddr, SocketAddr};

fn main() {
    println!("Refactor me!");

    let mut err = Error::new("NO_USER");
    err.status(404).message("User not found");
}

#[derive(Debug)]
pub struct Error {
    code: String,
    status: u16,
    message: String,
}

impl Default for Error {
    #[inline]
    fn default() -> Self {
        Self {
            code: "UNKNOWN".to_string(),
            status: 500,
            message: "Unknown error has happened.".to_string(),
        }
    }
}

impl Error {
    pub fn new<T: AsRef<str>>(code: T) -> Self {
        let mut err = Self::default();
        err.code = code.as_ref().to_string();
        err
    }

    pub fn status(&mut self, s: u16) -> &mut Self {
        self.status = s;
        self
    }

    pub fn message<T: AsRef<str>>(&mut self, m: T) -> &mut Self {
        self.message = m.as_ref().to_string();
        self
    }
}

#[derive(Debug, Default)]
pub struct Server(Option<SocketAddr>);

impl Server {
    pub fn bind(&mut self, ip: IpAddr, port: u16) {
        self.0 = Some(SocketAddr::new(ip, port))
    }
}

impl ToString for Server {
    fn to_string(&self) -> String {
        self.0.as_ref().unwrap().to_string()
    }
}

#[cfg(test)]
mod server_spec {
    use super::*;

    mod bind {
        use std::net::Ipv4Addr;

        use super::*;

        #[test]
        fn sets_provided_address_to_server() {
            let mut server = Server::default();

            server.bind(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);
            assert_eq!(format!("{}", server.to_string()), "127.0.0.1:8080");

            server.bind("::1".parse().unwrap(), 9911);
            assert_eq!(format!("{}", server.to_string()), "[::1]:9911");
        }
    }
}
