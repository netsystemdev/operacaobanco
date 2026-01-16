


 




  /*

		==> Regex validation's

		--> interface responsável pela validação de dados , conforme regex 
			
	
  */


 


   // _____ ... importa o tratamento de excecao ...

   use crate::exceptions::asynchronus_exception::AsynchronusException;




 
   pub trait IValidationsSystem {


   			// validacao para dados 


   			fn is_checked_cpf(x : &String) -> Result<bool , AsynchronusException> ; 

   			fn is_checked_date_format(dt : &String) ->  Result<bool , AsynchronusException> ; 

   			fn is_checked_telefone(tel : &String) ->  Result<bool , AsynchronusException> ; 


   }










