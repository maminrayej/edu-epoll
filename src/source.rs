use crate::{err, interest, registry, token};

pub trait Source {
    fn register(
        &mut self,
        registry: &registry::Registry,
        token: token::Token,
        interests: interest::Interest,
    ) -> err::Result<()>;

    fn reregister(
        &mut self,
        registry: &registry::Registry,
        token: token::Token,
        interests: interest::Interest,
    ) -> err::Result<()>;

    fn deregister(&mut self, registry: &registry::Registry) -> err::Result<()>;
}
