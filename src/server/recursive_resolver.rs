// src/server/recursive_resolver.rs
use trust_dns_proto::rr::{Name, RecordType};
use trust_dns_resolver::config::{ResolverConfig, ResolverOpts};
use trust_dns_resolver::TokioAsyncResolver;

pub struct RecursiveResolver {
    resolver: TokioAsyncResolver,
}

impl RecursiveResolver {
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let resolver = match TokioAsyncResolver::tokio(ResolverConfig::default(), ResolverOpts::default()) {
            Ok(r) => r,
            Err(e) => return Err(Box::new(e)),
        };

        Ok(Self { resolver })
    }

    pub async fn resolve(&self, domain: &str, record_type: RecordType) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let lookup = self.resolver.lookup(Name::from_utf8(domain)?, record_type).await?;
        let results = lookup.iter().map(|r| r.to_string()).collect();
        Ok(results)
    }
}
