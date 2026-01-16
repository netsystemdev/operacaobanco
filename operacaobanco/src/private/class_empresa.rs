 






    /*


		Classe Empresa : 

			Classe-filha + recebe a herança de conteúdo 




    */



    // < .. importação da classe .. > 

  use crate::Banco;

  use crate::private::class_dados_bancarios::DadosBancarios;

  use crate::private::class_dados_pessoais::DadosPessoais;




   use serde::{Serialize , Deserialize}; 

   use getset::{Getters , Setters};

   
   use crate::interface::info::Info; 




   #[derive(Debug , Clone , Serialize , Deserialize , Getters , Setters)] 

 	// classe

 	pub struct Empresa {

 			 // get ; set 

   			#[get = "pub"]

   		    banco : Banco , 



   		    #[get = "pub"]

   		    cnpj : i128 ,


   		    #[get = "pub"]

   			#[set = "pub"] 

   			dadosbancarios : DadosBancarios,


   			#[get = "pub"]

   			#[set = "pub"] 

   			dadospessoais  : DadosPessoais,


 	}


 	 // >> constructor >> 


 	 impl Empresa {

 	 		pub fn new(token : i64 , nome : String , cnpj : i128 , dadosbancarios : &DadosBancarios , 

 	 				dadospessoais : &DadosPessoais) -> Empresa 

 	 		{

 	 				  Self {

 	 				  		 banco : Banco::new(token , nome) , 

 	 				  		 cnpj , 

 	 				  		 dadosbancarios : dadosbancarios.clone(), 

   		 				   	 dadospessoais : dadospessoais.clone() 

 	 				  }


 	 		}


 	 }



 
 	     // herança dos dados 


    impl AsRef<Banco> for Empresa {

    		fn as_ref(&self) -> &Banco {

    			   &self.banco 	

    		}


    }




    impl AsMut<Banco> for Empresa {


    		fn as_mut(&mut self) -> &mut Banco {

    			    &mut self.banco

    		}


    }



  



