



  


   /*

		-- { Rust Programming }
	

		-- interface (trait) criada , para operações bancarias de amostra em sistema


		-- interligacao com a Asyncronus Exception > 


		-- Calculo de fatores 
   */




     use crate::exceptions::asynchronus_exception::AsynchronusException; 




     pub trait IDepartamentoFinanceiro {


     		    // <funcoes de operacao > 

     		    fn saque(&self , valor : f64) -> Result<f64 , AsynchronusException> 

     		    fn deposito(&self , dep : f64) -> Result<f64 , AsynchronusException>

     }




     

