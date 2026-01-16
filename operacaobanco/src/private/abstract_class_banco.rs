








  /*

		Rust Programming {} 


		-> OOP Class 


		** classe abstracta : apenas herda os componentes às classes filhas ** 

		Programador : Thiago V. Santos

  */






   // < .. importação das bases .. > 



   use serde::{Serialize , Deserialize}; 

   use getset::{Getters , Setters};




    // _____ -> classe _________ 


    #[derive(Debug , Clone , Serialize , Deserialize , Getters , Setters)] 

    pub struct Banco {

    		// criação das macros 

    		#[get = "pub"] 

    		#[set = "pub"] 

    		token : i64 , 


    		#[get = "pub"] 

    		#[set = "pub"] 

    		nome  : String , 


    }



    // < .. constructor .. > 

    impl Banco {

    		pub fn new(token : i64 , nome : String) -> Banco {


    				Self { token , nome } 

    		}


    		 // #get set is active ok !

    }





    // _______________________ ... ____________________________________ 