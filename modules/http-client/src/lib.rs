include!(concat!(env!("OUT_DIR"), "/capnp_include.rs"));

pub mod http_client {
    use capnp::{capability::Promise, Error, Result};
    use crate::domain::Domain;
    use crate::schema_capnp::{domain::Client, http_client::{Server, DomainParams, DomainResults}};
    
    pub struct HttpClient {}

    impl Server for HttpClient {
        fn domain(&mut self, params: DomainParams, results: DomainResults) -> Promise<(), Error> {
            let domain_result = params.get()
                .and_then(|r| r.get_domain())
                .and_then(|r| r.iter().collect::<Result<Vec<&str>>>());

            match domain_result {
                Ok(domain) => {
                    results.get().set_domain(Domain::new(domain.to_owned()));
                },
                Err(err) => Promise::err(err)
            }
        }
    }
}

pub mod domain {
    use capnp::{capability::Promise, Error, Result};
    use crate::schema_capnp::domain::{Server, SubdomainParams, SubdomainResults};

    pub struct Domain {
        domain: Vec<String>
    }

    impl Domain {
        pub fn new(domain: Vec<String>) -> Self {
            Self { domain }
        }
    }

    impl Server for Domain {
        fn subdomain(&mut self, params: SubdomainParams, results: SubdomainResults) -> Promise<(), Error> {
            let subdomains_result = params.get()
                .and_then(|r| r.get_subdomains())
                .and_then(|r| r.iter().collect::<Result<Vec<&str>>>());


        }
    }
}
