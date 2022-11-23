@0xec8fa8410eef13db;

interface HttpClient {
	domain @0 (domain :List(Text)) -> (domain :Domain);
}

interface Domain {
	subdomain @0 (subdomains :List(Text)) -> (subdomain :Domain);
	path @1 (segments :List(Text)) -> (path :Path);
}

interface Path {
	subpath @0 (segments :List(Text)) -> (path :Path);
	query @1 (parameters :List(QueryParameter)) -> (path :Path);
	get @2 () -> (response :Response);
}

struct QueryParameter {
	key @0 :Text;
	value @1 :Text;
}

struct Response {
	status @0 :Int16;
}
