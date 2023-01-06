use crate::http_capnp::domain as Domain;
use crate::http_capnp::https as Https;
use crate::http_capnp::path as Path;
use capnp::capability::Promise;
use capnp_rpc::pry;

// TODO Ask if getting caps is that and deriving works in this case
#[derive(Clone)]
pub struct DomainImpl {
    https_cap: Https::Client,
    domain_name: String, // TODO Should be some sort of URI type that only gets domain
                         // In hyper: `use hyper::http::uri::Authority;`
                         // Quite possibly a list of domains that will be combined in path
}

impl DomainImpl {
    pub fn new(https_cap: Https::Client, domain_name: &str) -> Self {
        // TODO Should take any type that coerces to String / Url
        DomainImpl {
            https_cap,
            domain_name: domain_name.to_string(),
        }
    }
}

impl Domain::Server for DomainImpl {
    fn subdomain(
        &mut self,
        params: Domain::SubdomainParams,
        mut results: Domain::SubdomainResults,
    ) -> Promise<(), capnp::Error> {
        let original_domain_name = self.domain_name.clone();
        let name = pry!(pry!(params.get()).get_name());
        let new_domain_name = name.to_string() + "." + &original_domain_name;
        let domain: Domain::Client = capnp_rpc::new_client(DomainImpl::new(
            self.https_cap.clone(),
            new_domain_name.as_str(),
        ));
        results.get().set_result(domain);
        Promise::ok(())
    }

    fn path(
        &mut self,
        params: Domain::PathParams,
        mut results: Domain::PathResults,
    ) -> Promise<(), capnp::Error> {
        let name = pry!(pry!(params.get()).get_name());
        let domain_cap: Domain::Client = capnp_rpc::new_client(self.clone());
        let path: Path::Client = capnp_rpc::new_client(crate::http::path::PathImpl::new(
            name.to_string(),
            self.https_cap.clone(),
            domain_cap,
        ));
        results.get().set_result(path);
        Promise::ok(())
    }
}
