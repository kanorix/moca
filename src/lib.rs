pub mod token;
pub mod lexical_analizer;

// pub struct Moca {
// }

// impl Moca {
//     fn new(src: &str) -> Self {
//         Moca {}
//     }
//     fn eval(&self, src: &str) {}
// }

#[cfg(test)]
mod tests {
    use crate::lexical_analizer::LexicalAnalizer;

    #[test]
    fn start_vm() {

        let src = "
        class Moca {
            public static main(String[]) {
                System.out.println(\"Hello world\");
                System.out.println(1234 + 23.4 / (34*30.2));
            }
        }
        ";
        let mut la = LexicalAnalizer::new(src);
        while let Some(token) = la.next_token() {
            println!("{}", token.to_string());
        }
    }
}
