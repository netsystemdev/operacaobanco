



 

	
    /*

		(Rust programming) 

		> enumeracao constante -> Tipo de pessoa a ser feita a operacao [ ] 
			
	
    */


   // < .. importação das bases .. > 



   use serde::{Serialize , Deserialize}; 


   //macro de aplicação

   #[derive(Debug , Clone , Serialize , Deserialize , Eq , PartialEq)]


   pub enum TipoPessoa {


   			PESSOA_FISICA = 1,

   			PESSOA_JURIDICA = 2,

   			SELECIONE = 0 , 

   }



// Formatacao ---> 

    use std::fmt; 


      impl fmt::Display for TipoPessoa {


    		fn fmt(&self , f : &mut fmt::Formatter<'_>) -> fmt::Result 

    		{

    			  	let _msg = match self {


    			  			Self::PESSOA_FISICA => "--  Pessoa fisica  --" , 

    			  			Self::PESSOA_JURIDICA => "--  Pessoa Jurídica --" , 

    			  			Self::SELECIONE => "-- Selecione ---" , 

    			  		 
    			  		    	  _ => panic!("erro!"), 


    			  	};


    			// formata a base 



    			write!(f , "{}" , _msg) 
    		}



    }



    impl TipoPessoa {


    	   pub const ALL_PERSON: &'static [TipoPessoa] = &[

    	   		// efetua uma variavel + array estático sobre as opções <> 


    	   		     TipoPessoa::PESSOA_FISICA ,

    	   			   TipoPessoa::PESSOA_JURIDICA , 

    	   		   

    	   		 


    	   ];



    	   // - funcao de iteração sobre as funcoes a serem criadas 

    	   pub fn iter() -> &'static [TipoPessoa] {        // iter() as i8

    	   				// repassa a variavel estatica com as constantes

    	   				Self::ALL_PERSON 


    	   }



    	   // ________ funcao de formatacao ___________ 


    	   pub fn get_format_value(e : i8) -> TipoPessoa {


    	   			match e {

    	   					1 => TipoPessoa::PESSOA_FISICA ,

    	   					2 => TipoPessoa::PESSOA_JURIDICA , 

    	   				
    	   						/* se der nada na selecao */

    	   					  _ =>  TipoPessoa::SELECIONE,


    	   			}

    	   }
 



    }