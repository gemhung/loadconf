extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;
use syn::DataStruct;
use syn::DeriveInput;

// Load_conf is a function to load configuration like `systrl.conf`
#[proc_macro_derive(LoadConf)]
pub fn load_conf(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    // Find all fields
    let syn::Data::Struct(DataStruct { ref fields, .. }) = input.data else {
        panic!("Unknown struct");
    };
    // Scan fields to construct init values
    let inited_values = fields.iter().map(|f| {
        let syn::Type::Path(syn::TypePath {path: syn::Path { segments, .. }, ..}) = &f.ty else {
            panic!("Unknown type");
        };
        let ident = &f.ident;
        let arguments = segments.first().map(|inner| &inner.arguments);
        match arguments {
            // For Vec<T>
            Some(syn::PathArguments::AngleBracketed(
                syn::AngleBracketedGenericArguments { .. },
            )) => {
                quote! {
                    #ident:
                        map
                          .get(stringify!(#ident))
                          .ok_or_else(|| anyhow::anyhow!("No suhc key({})", stringify!(#ident)))
                          .and_then(|vec|{
                              vec
                                .iter()
                                .map(|inner| inner.as_str().parse().map_err(anyhow::Error::new))
                                .collect::<Result<Vec<_>, _>>()
                          })?,
                }
            }
            // For POD type such as usize, f64, String, ... etc
            _ => {
                let ty = &f.ty;
                quote! {
                    #ident:
                      map
                        .get(stringify!(#ident))
                        .ok_or_else(|| anyhow::anyhow!("No suhc key({})", stringify!(#ident)))
                        .and_then(|vec| {
                            match (vec.len(), vec.first()) {
                                (1, Some(val)) => Ok(val),
                                _ => Err(anyhow::anyhow!("Multi values found but {} isn't `Vec`, consider using Vec<{}>", stringify!(#ty), stringify!(#ty))),
                            }
                        })
                        .and_then(|val| val.as_str().parse().map_err(anyhow::Error::new))?,
                }
            }
        }
    })
    .collect::<Vec<_>>();

    let ident = input.ident;
    let imported_parser = parser();
    let expanded = quote! {
        impl #ident {
            pub fn load_with_raw(text: &str) -> Result<Self, anyhow::Error> {
                #imported_parser
                // Read everything and map into HashMap<String, Vec<String>>,
                let mut map = parse(&text)?;
                // Construct `field_name=init_value,` based on hashmap above
                Ok(Self {
                    #(#inited_values)*
                })
            }
            pub fn load<P: AsRef<std::path::Path>>(path: P) -> Result<Self, anyhow::Error> {
                // Open file
                let text = std::fs::read_to_string(path)?;
                Self::load_with_raw(&text)
            }
        }
    };

    TokenStream::from(expanded)
}

// Parser is a simple parser to load configuration based on the syntax of `sysctl.conf` (https://man7.org/linux/man-pages/man5/sysctl.conf.5.html)
fn parser() -> syn::__private::TokenStream2 {
    quote! {
        use nom::character::complete::anychar;
        use nom::error::ErrorKind;
        use nom::IResult;
        use nom::branch::alt;
        use nom::bytes::complete::is_a;
        use nom::bytes::complete::tag;
        use nom::bytes::complete::take;
        use nom::character::complete::alpha1;
        use nom::character::complete::alphanumeric1;
        use nom::character::complete::char;
        use nom::character::complete::multispace0;
        use nom::character::complete::space0;
        use nom::combinator::not;
        use nom::combinator::recognize;
        use nom::error::ParseError;
        use nom::multi::many0_count;
        use nom::multi::many1;
        use nom::sequence::delimited;
        use nom::sequence::pair;
        use nom::sequence::preceded;
        use nom::sequence::separated_pair;
        use nom::AsChar;
        use nom::InputTakeAtPosition;
        use nom::Parser;
        fn any_but_space<T, E: ParseError<T>>(input: T) -> IResult<T, T, E>
        where
            T: InputTakeAtPosition,
            <T as InputTakeAtPosition>::Item: AsChar,
        {
            input.split_at_position1_complete(|item| item.as_char() == ' ', ErrorKind::Space)
        }
        fn rust_identifier(input: &str) -> IResult<&str, &str> {
            recognize(pair(
                alpha1,
                many0_count(alt((alphanumeric1, tag("_")))),
            ))(input)
        }
        fn parse(text: &str) -> Result<std::collections::HashMap<String, Vec<String>>, anyhow::Error> {
            let mut lines = text.lines();
            let mut blank_line_parser = not(preceded(space0::<&str, nom::error::Error<_>>, anychar));
            let mut comment_parser = preceded(
                space0::<&str, nom::error::Error<_>>,
                take(1usize).and_then(|inner| is_a(";#")(inner)),
            );
            let mut keyval_parser = separated_pair(
                delimited(space0, rust_identifier, space0),
                nom::character::complete::char('='),
                many1(delimited(
                    multispace0,
                    any_but_space::<_, nom::error::Error<_>>,
                    multispace0,
                )),
            );
            // Handle line by line to extract `key=value`
            lines
                .filter(|line| !blank_line_parser(line).is_ok())
                .filter(|line| !comment_parser(line).is_ok())
                .map(|line| {
                    // Parsing `toekn=value`
                    keyval_parser(line)
                      .map_err(|err| anyhow::Error::msg(err.to_string()))
                      .map(|(_remain, (key, vec))| (key.to_string(), vec.iter().map(ToString::to_string).collect::<Vec<_>>()))
                })
                .collect::<Result<std::collections::HashMap<_, _>, _>>()
        }
    }
}
