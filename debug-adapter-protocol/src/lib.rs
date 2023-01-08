mod debugAdapterProtocol;
use crate::debugAdapterProtocol::get_debug_adapter_protocol_json;
use syn::LitStr;

pub fn parse() -> Result<(), syn::Error> {
    let json = get_debug_adapter_protocol_json();
    let json_value: LitStr = syn::parse2(quote::quote! { "hello" "world" })?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        parse().unwrap();
    }
}
