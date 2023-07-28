#[cfg(test)]
mod tests {
    
    #[test]
    fn trait_impl_test() {

        // animal traits
        #[hereditary::trait_info]
        trait Cannis
        {
            fn bark(&self)-> String;
            fn sniff(&self)-> bool;
            fn roam(&mut self, distance:f64) -> f64;
            fn position(&self) -> f64;            
        }

        #[hereditary::trait_info]
        trait Bird
        {
            fn sing(&self) -> String;
            fn fly(&mut self, elevation:f64) -> f64;
            fn altitude(&self) -> f64;
        }

        // implementations
        struct Bulldog
        {
            position:f64
        }

        impl Cannis for Bulldog
        {
            fn bark(&self)-> String {
                "Guau!".into()
            }

            fn sniff(&self)-> bool {
                true
            }

            fn roam(&mut self, distance:f64) -> f64 {
                self.position += distance;
                self.position
            }

            fn position(&self) -> f64 {
                self.position
            }
        }

        // Bird implementation
        struct Seagull
        {
            elevation:f64
        }

        impl Bird for Seagull
        {
            fn sing(&self) -> String {
                "EEEYA!".into()
            }

            fn fly(&mut self, elevation:f64) -> f64 {
                self.elevation += elevation;
                self.elevation
            }

            fn altitude(&self) -> f64 {
                self.elevation
            }
        }

        // Heritance for an hybrid animal
        #[derive(hereditary::Forwarding)]        
        struct KimeraSphinx
        {
            #[forward_derive(Cannis)]
            dogpart:Bulldog,
            birdpart:Seagull
        }

        impl KimeraSphinx
        {
            fn new() -> Self
            {
                Self { dogpart: Bulldog { position: 0f64 } , birdpart: Seagull { elevation: 0f64 } }
            }
        }

        #[hereditary::forward_trait(birdpart)]
        impl Bird for KimeraSphinx
        {
            fn sing(&self) -> String
            {
                // because is a dog, it barks
                self.dogpart.bark()
            }
        }

        // Instance kimera
        let mut kimera = KimeraSphinx::new();
        assert_eq!(kimera.bark(), kimera.sing());
        assert_eq!(kimera.sniff(), true);

        let dis = kimera.fly(50f64);
        assert_eq!(kimera.roam(-dis), -50f64);
        assert_eq!(kimera.altitude(), 50f64);

    }
}
