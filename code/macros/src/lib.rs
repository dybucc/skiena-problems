use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{
    FieldValue, Result as SynResult, Token, parenthesized,
    parse::{Parse, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
};

struct Points(Punctuated<Point, Token![,]>);

impl Points {
    fn tokenize(self) -> TokenStream2 {
        let mut points = Punctuated::<_, Token![,]>::new();

        for point in self.0 {
            let (x, y) = (
                &point
                    .0
                    .first()
                    .expect("All points should have an x-component.")
                    .expr,
                &point
                    .0
                    .last()
                    .expect("All points should have a y-component.")
                    .expr,
            );

            points.push(quote! { Point2d { x: #x as f64, y: #y as f64 } });
        }

        quote! { vec![#points] }
    }
}

impl Parse for Points {
    fn parse(input: ParseStream) -> SynResult<Self> {
        Ok(Self(input.parse_terminated(Point::parse, Token![,])?))
    }
}

struct Point(Punctuated<FieldValue, Token![,]>);

impl Point {
    fn parse(input: ParseStream) -> SynResult<Self> {
        let content;
        let _ = parenthesized!(content in input);

        Ok(Self(
            content.parse_terminated(FieldValue::parse, Token![,])?,
        ))
    }
}

#[proc_macro]
pub fn points(input: TokenStream) -> TokenStream {
    let points = parse_macro_input!(input as Points).tokenize();

    if points.is_empty() {
        panic!("There's no points.");
    }

    TokenStream::from(quote! { GeoAdjacencyMatrix::from_point_set(#points) })
}

#[cfg(test)]
mod tests {
    use syn::parse_quote;

    use super::*;

    #[test]
    fn it_works() {
        // The actual macro invocation would look something like:
        // ```
        // points! {
        //     <the below list>
        // }
        // ```
        let input: Points = parse_quote! {
            (x: 1.25, y: 2),
            (x: 1.3, y: 5),
            (x: 1.5, y: 3.5),
            (x: 2, y: 3.6),
            (x: 3, y: 0.75),
            (x: 3.75, y: 3.7),
            (x: 4.25, y: 3),
            (x: 4.3, y: 1.7),
            (x: 4.5, y: 5),
            (x: 5.8, y: 3.45),
            (x: 6, y: 1),
            (x: 6.2, y: 4.7),
            (x: 7, y: 3.45),
        };

        eprintln!("{}", input.tokenize());
    }
}
