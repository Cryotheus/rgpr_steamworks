use proc_macro::TokenStream;
use syn::{LitCStr, LitStr, Type};
use syn::parse::{Parse, ParseStream};

struct ExternCInterfaceMacro {}

impl Parse for ExternCInterfaceMacro {
	fn parse(input: ParseStream) -> syn::Result<Self> {
		let ifc_type = input.parse::<Type>()?;
		let name_ident = input.parse::<Type>()?;
		let version_str = input.parse::<LitCStr>()?;

		todo!()
	}
}

pub fn extern_c_interface(token_stream: TokenStream) -> TokenStream {
	token_stream
}
