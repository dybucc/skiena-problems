use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{
    FieldValue, Result as SynResult, Token, braced, parenthesized,
    parse::{Parse, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
    token::Brace,
};

struct Points(Vec<Punctuated<FieldValue, Token![,]>>);

impl Points {
    fn tokenize(self) -> TokenStream2 {
        let mut points = Punctuated::<TokenStream2, Token![,]>::new();

        for point in self.0 {
            let (x, y) = (
                &point
                    .first()
                    .expect("The point should an x-component.")
                    .expr,
                &point.last().expect("The point should a y-component.").expr,
            );

            points.push(quote! { Point2d { x: #x as f64, y: #y as f64 } });
        }

        (quote! { vec![#points] })
    }
}

impl Parse for Points {
    fn parse(input: ParseStream) -> SynResult<Self> {
        let (points_set, line_set);
        let mut output = Self(Vec::new());

        let _: Brace = braced!(points_set in input);
        let _: Brace = braced!(line_set in input);

        loop {
            if points_set.is_empty() {
                break;
            }

            let pair;
            let _ = parenthesized!(pair in points_set);

            output
                .point_list
                .push(pair.parse_terminated(FieldValue::parse, Token![,])?);
        }

        Ok(output)
    }
}

#[proc_macro]
pub fn points(input: TokenStream) -> TokenStream {
    let points = parse_macro_input!(input as Points).tokenize();

    TokenStream::from(quote! {
        GeoAdjacencyMatrix::from_point_set(#points)
    })
}

#[cfg(test)]
mod tests {
    use syn::parse_quote;

    use super::*;

    #[test]
    fn it_works() {
        let input: Points = parse_quote! {
            (x: 1.25, y: 2)
            (x: 1.3, y: 5)
            (x: 1.5, y: 3.5)
            (x: 2, y: 3.6)
            (x: 3, y: 0.75)
            (x: 3.75, y: 3.7)
            (x: 4.25, y: 3)
            (x: 4.3, y: 1.7)
            (x: 4.5, y: 5)
            (x: 5.8, y: 3.45)
            (x: 6, y: 1)
            (x: 6.2, y: 4.7)
            (x: 7, y: 3.45)
        };

        eprintln!("{}", input.tokenize(),);
    }
}
