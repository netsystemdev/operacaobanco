



/*

		
		 

		-> Rust programming

		-> Enum : Responsavel por informar o tipo de conta (basica , corrente , conta salario )

		-> Dados 
	

 */



   // < .. importação das bases .. > 



   use serde::{Serialize , Deserialize}; 


   //macro de aplicação

   #[derive(Debug , Clone , Serialize , Deserialize , PartialEq)]

   pub enum TipoConta {


   			CONTA_CORRENTE = 1 ,

   			CONTA_SALARIO = 2 ,

   			CONTA_MASTERCLASS = 3 , 

   			PRIVATE_PERSONALITE = 4, 

   			SELECIONE = 0,


   }


    // Formatacao ---> 

    use std::fmt; 


      impl fmt::Display for TipoConta {


    		fn fmt(&self , f : &mut fmt::Formatter<'_>) -> fmt::Result 

    		{

    			  	let _msg = match self {


    			  			Self::CONTA_CORRENTE => "-- Conta Corrente --" , 

    			  			Self::CONTA_SALARIO => "-- Conta Salário --" , 

    			  			Self::CONTA_MASTERCLASS => "--Conta Masterclass--" , 

    			  			Self::PRIVATE_PERSONALITE => "--Conta Private : Perfil Personalitée --" ,

    			  		    Self::SELECIONE => "-- Selecione --" , 


    			  		    	  _ => panic!("erro!"),


    			  	};


    			// formata a base 



    			write!(f , "{}" , _msg) 
    		}



    }



    impl TipoConta {


    	   pub const ALL_CONTAS: &'static [TipoConta] = &[

    	   		// efetua uma variavel + array estático sobre as opções <> 


    	   		     TipoConta::CONTA_CORRENTE ,

       	   			 TipoConta::CONTA_SALARIO , 

    	   		     TipoConta::CONTA_MASTERCLASS , 

      	   			 TipoConta::PRIVATE_PERSONALITE , 

    	   		      


    	   ];



    	   // - funcao de iteração sobre as funcoes a serem criadas 

    	   pub fn iter() -> &'static [TipoConta] {        // iter() as i8

    	   				// repassa a variavel estatica com as constantes

    	   				Self::ALL_CONTAS 


    	   }



    	   // ________ funcao de formatacao ___________ 


    	   pub fn get_format_value(e : i8) -> TipoConta {


    	   			match e {

    	   					1 => TipoConta::CONTA_CORRENTE ,

    	   					2 => TipoConta::CONTA_SALARIO , 

    	   					3 => TipoConta::CONTA_MASTERCLASS , 

    	   					4 => TipoConta::PRIVATE_PERSONALITE , 

    	   						/* se der nada na selecao */

    	   					  _ => TipoConta::SELECIONE,


    	   			}

    	   }
 



    }





 