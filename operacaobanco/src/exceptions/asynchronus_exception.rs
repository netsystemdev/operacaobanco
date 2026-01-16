




   

   /*

		
		 *** <exceptions > ***


		 -> exception : tratamento e granulação + composição de cenários de erros 

		 -> ligacao com duas classes + interface 

		 	-> Classe-negocio

		 	-> Interface Departamento financeiro

	

   */





 #[derive(Debug , Clone)] 

 pub enum AsynchronusException {


 		// mapeamento de diversos cenários de erros { }

 			 INTEGER_ARGUMENT_INVALID_EXCEPTION ,

 			 FLOAT_ARGUMENT_INVALID_EXCEPTION,

 			 DATE_ARGUMENT_INVALID, 

 			 DATE_PARSE_FORMAT_EXCEPTION , 

 			 FLOAT_PARSE_EXCEPTION , 

 			 ARITHMETIC_MEMORY_ERROR , 

 			 JSON_COMMIT_PERSISTENCE_EXCEPTION,

 			 REGEX_SYSTEM_ERROR,

 			 INSUPORTABLE_ERROR


 }



      // Formatacao ---> 

    use std::fmt; 


    impl fmt::Display for AsynchronusException {


    		fn fmt(&self , f : &mut fmt::Formatter<'_>) -> fmt::Result 

    		{
 		
 					// mensagem , ao qual classificará os erros 

 					
 					 let _error = match self {

 					 	Self::INTEGER_ARGUMENT_INVALID_EXCEPTION => "Argument Invalid Exception : Falha na inserção de dados or type int value " , 

 					 	Self::FLOAT_ARGUMENT_INVALID_EXCEPTION => "Argument Invalid Exception : Float não pode ser nulo ou falha na inserção" , 

 					 	Self::DATE_ARGUMENT_INVALID => "Date Invalid : Data nula ou fora da margem de inserção válida" , 

 					 	Self::DATE_PARSE_FORMAT_EXCEPTION => "Date Format invalid : Padrão NaiveDate %d/%m/%Y inválido ", 

 					 	Self::FLOAT_PARSE_EXCEPTION => "E-Convert Parse Exception : Falha na conversão strToFloat<0,00> format!",

 					 	Self::ARITHMETIC_MEMORY_ERROR => "Arithmetic Exception : Falha na memória-base de cálculo !", 

 					 		/* para json comit */

 					 	Self::JSON_COMMIT_PERSISTENCE_EXCEPTION => "JSON Exception : Falha na persistência de Estrutura from_json ou to_json " ,

 					 	Self::REGEX_SYSTEM_ERROR => "Regex System Exception : Falha na formatação ou match invalidado !",


 					 	Self::INSUPORTABLE_ERROR => "Memory Abrupt / Insuportable Error : Por motivos de segurança , a aplicação será fechada !", 


 					 		_ => panic!("erro interno!"),



 					 };

 				 // formata a base



 				 write!(f , "{}" , _error) 

 			}



 	 }


 	 
 	 