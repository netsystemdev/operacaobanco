




 /*

		
		Class : dados bancários { } 

		-> Rust programming

		-> Classe -> associacao por agregacao às classes filhas Empresa e Cliente

		-> Dados 
	

 */



   // < .. importação das bases .. > 



   use serde::{Serialize , Deserialize}; 

   use getset::{Getters , Setters};

   
   use crate::interface::info::Info;

   use crate::private::enum_tipo_conta::TipoConta;


    // _____ -> classe _________ 


    #[derive(Debug , Clone , Serialize , Deserialize , Getters , Setters)] 

    pub struct DadosBancarios {

    		   // get ; set 

    		   #[get = "pub"]

    		   #[set = "pub"]

    		   agencia : i16 , 


    		   // <enum associada >> 

    		   #[get = "pub"]

    		   #[set = "pub"]

    		   tipoconta : TipoConta , 


    		   #[get = "pub"]

    		   #[set = "pub"]

    		   contacorrente : i64 , 

    		    #[get = "pub"]

    		    #[set = "pub"]

    		    saldo_conta : f64,      // !! <atencao!> 


    		    // !! <atencao ! chave pix , somente será feito metodo get > 

    		     #[get = "pub"]

    		     chave_pix : String,    // email , telefone , ....
    		                   


    }


    // < ------- constructor --------- > 

    impl DadosBancarios {

    		pub fn new(agencia : i16 , tipoconta : &TipoConta , 

    				  contacorrente : i64 , saldo_conta : f64 , chave_pix : String) -> DadosBancarios {

    			      Self {agencia , tipoconta : tipoconta.clone() , contacorrente , saldo_conta , chave_pix }

    		}


    }



    // ___________ ... < interface - info de dados (exceção chave pix ) > ... ______________


    impl Info for DadosBancarios {


    		/* funcao */


    		fn painel(&self) {


    					println!(" ------- informações bancárias ---------");

    					println!("\n");

    					println!("Agencia   :  {} " , &self.agencia()); 

    					println!("Tipo de Conta : {:?} " , &self.tipoconta()); 

    					println!("Conta corrente : {} " , &self.contacorrente());

    					println!("Saldo Conta  R$ :  {:.2} " , &self.saldo_conta());


    		}


    }








