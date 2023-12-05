use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, AttributeArgs, Ident, ItemFn, Lit, NestedMeta};

// CM: Note:  I got these macros from here:
// https://github.com/AxlLind/AdventOfCode2022

#[proc_macro_attribute]
pub fn main(args: TokenStream, input: TokenStream) -> TokenStream {
    let input_path = match &parse_macro_input!(args as AttributeArgs)[..] {
        [NestedMeta::Lit(Lit::Int(day))] => format!("../../inputs/{}.in", day.token()),
        _ => panic!("Expected one integer argument"),
    };

    let mut aoc_solution = parse_macro_input!(input as ItemFn);
    aoc_solution.sig.ident = Ident::new("aoc_solution", aoc_solution.sig.ident.span());

    let tokens = quote! {
        use clap::Parser;
        const INPUT: &str = include_str!(#input_path);

        #[derive(Parser)]
        struct Cli {
            #[arg(short, long, default_value_t = false)]
            perf: bool
        }

        #aoc_solution

        fn main() {
        let args = Cli::parse();
        let mut tot = 0.0;
        let mut out : (usize, usize) = (0, 0);
        let mut num_tries = 1;
        if args.perf == true {
            num_tries = 1000
        }
        for _ in 0..num_tries {
        let now = ::std::time::Instant::now();
        out = aoc_solution(INPUT.trim_end());
        let elapsed = now.elapsed();
        tot += elapsed.as_micros() as f32;
        }
        tot = tot / num_tries as f32;
        if !args.perf {
            println!("\nPart one: {}", out.0);
            println!("Part two: {}", out.1);
            println!("Time: {}μs", tot as usize);
        }
        else {
            println!("Time: {}μs", tot as usize);
        }
        //if elapsed.as_millis() > 0 {
        //  println!("Time: {}ms", elapsed.as_millis());
        //} else {
        //}
    }
    };
    TokenStream::from(tokens)
}
