




 
    /*

		
		Class : dados pessoais { } 

		-> Rust programming

		-> Classe -> associacao por agregacao às classes filhas Empresa e Cliente
 
	


 */


   // < .. importação das bases .. > 






   use serde::{Serialize , Deserialize}; 

   use getset::{Getters , Setters};

   
   use crate::interface::info::Info; 



   // class 

   #[derive(Debug , Clone , Serialize , Deserialize , Getters , Setters)] 

   pub struct DadosPessoais {


   			  /* get set */

   			  #[ get = "pub" ]

   			  #[ set = "pub" ]

   			  endereco : String , 

   			  #[ get = "pub" ]

   			  #[ set = "pub" ]

   			  bairro : String ,

   			  #[ get = "pub" ]

   			  #[ set = "pub" ]

   			  telefone : i64 ,



          /* para solicitação de emprestimo */

          #[ get = "pub" ]

          #[ set = "pub" ]

          rendafinanceira : f64,     // crucial para crivo + analise financeira




   }



   // < ... constructor ... > 

   impl DadosPessoais {


   			pub fn new(endereco : String , bairro : String , telefone : i64 , rendafinanceira : f64) -> DadosPessoais {

   					Self { endereco , bairro , telefone , rendafinanceira } 

   			}

   }



   // ___________ ... < interface - info de dados (exceção chave pix ) > ... ______________


    impl Info for DadosPessoais {


    		/* funcao */


    		fn painel(&self) {


    					println!(" ------- informações ---------");

    					println!("\n");

    					println!("Endereço  :  {} " , &self.endereco()); 

    					println!("Bairro : {} " , &self.bairro()); 

    					println!("Telefone  - Contato : {} " , &self.telefone());

    				 

    		}


    }




