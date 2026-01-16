




 

   /* 


		Rust programming {} 


		-> entrada da classe Pessoa 

		-> herança com a classe principal
	

   */


  extern crate chrono;

  use chrono::{Utc , NaiveDate} ; 



  // < .. importação da classe .. > 

  use crate::Banco;

  use crate::private::class_dados_bancarios::DadosBancarios;

  use crate::private::class_dados_pessoais::DadosPessoais;




   use serde::{Serialize , Deserialize}; 

   use getset::{Getters , Setters};

   
   use crate::interface::info::Info; 




   #[derive(Debug , Clone , Serialize , Deserialize , Getters , Setters)] 

   pub struct Pessoa {

   // class 

   			// get ; set 

   			#[get = "pub"]

   		    banco : Banco , 



   		    #[get = "pub"]

   			 

   			cpf : String ,


   			#[get = "pub"]

   			#[set = "pub"] 

   			idade : NaiveDate ,



   			#[get = "pub"]

   			#[set = "pub"] 

   			dadosbancarios : DadosBancarios,


   			#[get = "pub"]

   			#[set = "pub"] 

   			dadospessoais  : DadosPessoais,



   }


   // ...... <constructor > ......... 


   impl Pessoa {


   		 pub fn new(token : i64 , nome : String , cpf : String , 

   		 	idade: NaiveDate , dadosbancarios : &DadosBancarios , 

   		 		dadospessoais : &DadosPessoais) -> Pessoa 

   		 {

   		 		Self {

   		 				// herança 

   		 				banco : Banco::new(token , nome) , 

   		 				cpf , 

   		 				idade ,

   		 				dadosbancarios : dadosbancarios.clone(), 

   		 				dadospessoais : dadospessoais.clone() 

   		 		}



   		 }



   		 // ......... [ get set is ok ! ] .............




   }




    // herança dos dados 


    impl AsRef<Banco> for Pessoa {

    		fn as_ref(&self) -> &Banco {

    			    &self.banco

    		}


    }




    impl AsMut<Banco> for Pessoa {


    		fn as_mut(&mut self) -> &mut Banco {

    			  &mut self.banco

    		}


    }



  