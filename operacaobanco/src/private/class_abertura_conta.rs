 





    /* 

	
	       Rust Programming 


	       -> 1a. Classe de negocio : Abertura de conta

	       -> Relacionamento das classes 

	       -> Associacoes + verificacao de tipo pessoa 

	       -> interface 

	       -> cadastramento dos dados 



			Obs : inserção também de metodo para a geração da conta corrente -> via PDF 


    */ 





  use crate::private::enum_tipo_pessoa::TipoPessoa;

  use crate::private::enum_tipo_conta::TipoConta; 

  use crate::private::class_dados_bancarios::DadosBancarios;

  use crate::private::class_dados_pessoais::DadosPessoais;


  // extern crate pdfgen;

   

   use serde::{Serialize , Deserialize}; 

   use getset::{Getters , Setters};

     extern crate chrono;

     use chrono::{Utc , NaiveDate} ; 
 

   
   use crate::interface::info::Info; 


   #[derive(Debug , Clone , Serialize , Deserialize , Getters , Setters)]   

   

   pub struct AberturaConta {


   			   // Tipo de pessoa que abrirá a conta ? 

   			   #[get = "pub"] 

   			   #[set = "pub"] 

   			   tipopessoa : TipoPessoa, 


   			   // Tipo de Conta a ser aberta ?? 

   			   #[get = "pub"] 

   			   #[set = "pub"] 

   			   tipoconta : TipoConta, 


   			   // Data da abertura da conta 

   			   #[get = "pub"] 

   			   data_abertura : NaiveDate , 


   			   // quem é o gestor responsável ? 

   			   #[get = "pub"] 

   			   #[set = "pub"] 

   			   gerente : String,


   }



   // -------------- < impl > ------------------------------------

   impl AberturaConta {


   			pub fn new(tipopessoa : &TipoPessoa , tipoconta : &TipoConta , 

   				   data_abertura : NaiveDate , gerente : String) -> AberturaConta {


   						Self {

   							  tipopessoa : tipopessoa.clone() , 

   							  tipoconta  : tipoconta.clone() ,

   							  data_abertura ,

   							  gerente,


   						}


   			}

   }














  
   
 
  