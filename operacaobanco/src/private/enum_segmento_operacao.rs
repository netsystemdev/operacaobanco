





 

   /*


		Enum > Segmento de operação : 

		  - Enum de seleção para qual ramo designado de operação bancária -


		Rust programming {} 


   */






	
    /*

		(Rust programming) 

		> enumeracao constante -> Tipo de pessoa a ser feita a operacao [ ] 
			
	
    */


   // < .. importação das bases .. > 



   use serde::{Serialize , Deserialize}; 


   //macro de aplicação

   #[derive(Debug , Clone , Serialize , Deserialize , PartialEq)]


   pub enum SegmentoOperacao {


   		 	SAQUE = 1 , 

   		 	DEPOSITO = 2 , 

   		 	EMPRESTIMOS = 3,

   			SELECIONE = 0 , 

   }



// Formatacao ---> 

    use std::fmt; 


      impl fmt::Display for SegmentoOperacao {


    		fn fmt(&self , f : &mut fmt::Formatter<'_>) -> fmt::Result 

    		{

    			  	let _msg = match self {


    			  			Self::SAQUE => "--  Operação Saque --" , 

    			  			Self::DEPOSITO => "--  Deposito  --" , 

    			  			Self::EMPRESTIMOS => "-- Emprestimos de Fundos / Solicitação ---" , 

    			  			Self::SELECIONE => "--- Selecione a Operação -----",

    			  		 
    			  		    	  _ => panic!("erro!"),


    			  	};


    			// formata a base 



    			write!(f , "{}" , _msg) 
    		}



    }



    impl SegmentoOperacao {


    	   pub const ALL_OPERATIONS: &'static [SegmentoOperacao] = &[

    	   		// efetua uma variavel + array estático sobre as opções <> 


    	   		     SegmentoOperacao::SAQUE ,
 
    	   		 	   SegmentoOperacao::DEPOSITO , 

    	   		     SegmentoOperacao::EMPRESTIMOS , 

    	   		     SegmentoOperacao::SELECIONE,

    	   		 


    	   ];



    	   // - funcao de iteração sobre as funcoes a serem criadas 

    	   pub fn iter() -> &'static [SegmentoOperacao] {        // iter() as i8

    	   				// repassa a variavel estatica com as constantes

    	   				Self::ALL_OPERATIONS 


    	   }



    	   // ________ funcao de formatacao ___________ 


    	   pub fn get_format_value(e : i8) -> SegmentoOperacao {


    	   			match e {

    	   					1 => SegmentoOperacao::SAQUE ,

    	   					2 => SegmentoOperacao::DEPOSITO , 

    	   					3 => SegmentoOperacao::EMPRESTIMOS , 



    	   						/* se der nada na selecao */

    	   					  _ =>  SegmentoOperacao::SELECIONE,


    	   			}

    	   }
 



    }


    