






	
	/*

          ->   Rust System Programming {} 

	
		  ->   campos de entrada de dados , para abertura de conta
		
		
		  -> 

	*/
	

   

    // ________ importacao das bibliotecas _____________

     
     use std::io;

     use std::io::prelude::*;

     use clearscreen;

     extern crate chrono;

 	 use chrono::{Utc , NaiveDate} ; 


 	 // formatacao do titulo

 	 use prettytable::{Table , row , Cell} ;  

    	
     use crate::private::abstract_class_banco::Banco;    // -> mentor classe-mae , que deixará o campo token + nome

     use crate::private::enum_tipo_pessoa::TipoPessoa;

 	 use crate::private::enum_tipo_conta::TipoConta; 

  	 use crate::private::class_dados_bancarios::DadosBancarios;

     use crate::private::class_dados_pessoais::DadosPessoais;

     use crate::exceptions::asynchronus_exception::AsynchronusException; 

        
     extern crate regex;

     use regex::Regex;



    	
     pub fn limpa_tela() {


     		io::stdin().read(&mut [0u8]).unwrap();

     		let _  = clearscreen::clear();

     }
	 	




     type TRuntimeException<T> = Result<T , Box<dyn std::error::Error>> ; 

     pub fn abertura_banco() -> TRuntimeException<()> 

     {      

            let mut tabela = Table::new();  // variavel auxiliar para formatacao de titulos em forma de tabela 

            let mut tabela02 : Table = Table::new(); 




            // aceitacao de contrato -> finalizacao dos termos 

            let mut continua : char = 'S';


            // enquanto o laço nao finaliza , continua 

            let _  = clearscreen::clear(); 


            while continua == 'S' || continua == 's' {

             // informa o titulo

             let _ = geracao_tabela_titulo();



		     // Geracao de token automatizada , apenas segue o cadastro das informações 

		     print!("NOVO ASSOCIADO  :  _  ") ;

		     io::stdout().flush()?; 

		     let camponome = input_nome()?; 


		     // checa de dados 

		     let _ = match is_checked_only_letters(&camponome) {

		     			Ok(val) => val , 

		     			 Err(erro) => { println!("{}" , erro) ; limpa_tela(); continue; } 

		     };

				     

		     // -------------------------------------------------------------------


		     println!("\n");
			     
		     // define o tipo de cliente {}  

		     // enum de carregamento

		     for it in TipoPessoa::iter() {

		     		  println!(" CONTA  ->  [ {} ]  :  {:?} " , it.clone() as i8 , it);

		     }

		     print!("TIPO DE PESSOA   _  "); 

		     io::stdout().flush()?; 

		     let campo_tipopessoa = input_tipo_pessoa()?;

		     //checa de dados 

		     let _ = match is_checked_only_digits(&campo_tipopessoa) {

		     			Ok(val) => val , 

		     			 Err(erro) => { println!("{}" , erro) ; limpa_tela(); continue; } 

		     };


		     // conversao

		     let conv_tipo_pessoa : i8 = match parse_integer_8(&campo_tipopessoa) {

		     			Ok(val) => val , 

		     				Err(erro) => {

		     						// try-catch exception

		     						panic!("ParseException Error :  {} " , erro);

		     				},

		     };




		      // -------------------------------------------------------------------


		     println!("\n");
			     
		     // define o tipo de cliente {}  

		     // enum de carregamento

		     for it in TipoConta::iter() {

		     		  println!(" TIPO DE CONTA ->  [ {} ]  :  {:?} " , it.clone() as i8 , it);

		     }

		     print!("TIPO DE CONTA   _  "); 

		     io::stdout().flush()?; 

		     let campo_tipoconta = input_tipo_conta()?;

		     //checa de dados 

		     let _ = match is_checked_only_digits(&campo_tipoconta) {

		     			Ok(val) => val , 

		     			 Err(erro) => { println!("{}" , erro) ; limpa_tela(); continue; } 

		     };


		     // conversao

		     let conv_tipo_conta : i8 = match parse_integer_8(&campo_tipoconta) {

		     			Ok(val) => val , 

		     				Err(erro) => {

		     						// try-catch exception

		     						panic!("ParseException Error :  {} " , erro);

		     				},

		     };



		     //----------------------------------------------------------------------//

		      
            // ________   redireciona o tipo de selecao de pessoa _________________ 


           /*

                Se o gerente escolher pessoa fisica : CPF 

                Se o gerente escolher pessoa juridica : CNPJ

           */

 
        

           let _ = match conv_tipo_pessoa {    

                       // chama a macro de comparacao 'Equals' da enumeracao TipoPessoa {} 


                       1 => {

                               // apresenta o campo cpf 

                               println!("\n"); 

                               print!("CPF  -  ex: 999.999.999-99 : ") ; 

                               io::stdout().flush()?; 

                               let campo_cpf = input_cpf();

                               // validacao regex 

                               let _ = match is_checked_cpf(&campo_cpf?.trim()) 

                               {
                                    Ok(val) => val , 

                                        Err(regex_error) => {

                                                 println!("Regex Error :  {} " , regex_error);

                                                 limpa_tela(); continue;    

                                        },


                               }; // encerra validacao




                       },



                      2  => {

                                 // apresenta o campo CNPJ 

                               println!("\n"); 

                               print!("CNPJ  -    : ") ; 

                               io::stdout().flush()?; 

                               let campo_cnpj = input_cnpj();

                               // validacao regex 

                                let _ = match is_checked_cnpj(&campo_cnpj?.trim()) 

                               {
                                    Ok(val) => val , 

                                        Err(regex_error) => {

                                                 println!("Regex Error :  {} " , regex_error);

                                                 limpa_tela(); continue;    

                                        },


                               }; // encerra validacao




                                  

                       },




                       0 => {

                                 limpa_tela(); continue; 

                       },


                                /*funcoes diferente */

                         _ => {

                                limpa_tela(); continue; 

                         },


           };



           // segue o percurso e devolve para o algotitmo sequencial de cadastro < > 

           println!("\n");

           print!("GERENTE RESPONSAVEL   _ "); 

           io::stdout().flush()?;   

           let campo_gerente = input_gerente()?;

           // checa de dados 

            
            let _ = match is_checked_only_letters(&campo_gerente) {

                        Ok(val) => val , 

                         Err(erro) => { println!("{}" , erro) ; limpa_tela(); continue; } 

             };



             // -- espera uma tecla //

             io::stdin().read(&mut [0u8])?;  


             println!("\n\n");

             // cria a tabela 

             tabela.add_row(row![" PREENCHIMENTO -  DADOS PESSOAIS  "]);

             tabela.printstd(); 


             println!("\n"); 

             print!("ENDEREÇO RESIDENCIAL  _  "); 

             io::stdout().flush()?; 

             let campo_endereco = input_endereco()?;

             // validacao simples !

             while campo_endereco.trim().is_empty() {

                    println!("atenção ! informe o endereço ! o campo está vazio !");

                    limpa_tela(); continue; 


             }



             println!("\n");

             print!("BAIRRO  _ "); 

             io::stdout().flush()?; 

             let campo_bairro = input_bairro()?; 


             if campo_bairro.trim().is_empty() {

                    println!("atenção ! informe o bairro ! o campo está vazio !");

                    limpa_tela(); continue; 


             }


             //


             println!("\n");

             print!("TELEFONE CONTATO  _ "); 

             io::stdout().flush()?; 

             let campo_telefone = input_telefone()?; 


             // checando o tamanho exato do telefone

             // ex: 21999989999
            
            


             // ------------------ informacao - dados bancarios ----------------------------


             println!("\n\n");

             // cria a tabela 

             tabela02.add_row(row![" PREENCHIMENTO -  DADOS BANCÁRIOS    "]);

             tabela02.printstd(); 


             println!("\n"); 

             print!("AGENCIA / LOCALIZACAO     _  ") ; 

             io::stdout().flush()?;

             let campo_agencia = input_agencia()?; 


              // checked erros 

             let _ = match is_checked_only_digits(&campo_agencia) {

		     			Ok(val) => val , 

		     			 Err(erro) => { println!("{}" , erro) ; limpa_tela(); continue; } 

		     };










            


            



		     	    
















            } // 


             // se parar , efetua a conexao com a base json  , para persistencia e inserir o novo cadastro




        io::stdout().flush()?; 

        io::stdin().read(&mut [0u8])?;     

     	Ok(())

     }






   //-------------- tabela --------------------------------------//

      fn geracao_tabela_titulo()  

     {


            // >> constroi a tabela , sentido horizontal (simples)

            let mut table = Table::new(); 

            table.add_row(row!["  ABERTURA DE CONTA - NOVO CLIENTE  "]);

           

                // cria a tabela e redesenha

                table.printstd();




        
     }



     // ________________ bibliotecas _________________________________


     // _________ entrada de virgula ___________- 


  fn decimal_is_comma(x : &String) -> Result<f64 , AsynchronusException> {


            let _trimmer = x.trim(); 


            // confere e ve se esta vazio o campo 

             if _trimmer.is_empty() {

                    return Err(AsynchronusException::REGEX_SYSTEM_ERROR);

             }



             // associa a devolucao em replace 0.00 + conversao de casting + chamada da exceção 


               let conv_final : f64 = match _trimmer.replace("," , ".").parse::<f64>() {


                        Ok(val) => {

                            return Ok(val);


                        },


                            /* se der erro , aciona o tratamento de exceção */

                             Err(parse_exception) => {

                                        // Erro destacado 
                                        
                                        return Err(AsynchronusException::FLOAT_PARSE_EXCEPTION); 

                                        panic!("Error Stacktrace  :  {} " , parse_exception);     


                             },


               };





  }







 // _____________________.. .______________________________________________________________


   type TStringTypeofException<T> = Result<T , Box<dyn std::error::Error>>;


  // string validations 

  
  fn is_checked_only_letters(x : &String) -> TStringTypeofException<String> {


            let _trimmer = x.trim(); 


            // confere e ve se esta vazio o campo 

             if _trimmer.is_empty() {

                    return Err("String.ToFixedError :: Erro ! O campo está vazio!".into()); 

             }


             // analisa se o campo contém somente letras

             if _trimmer.chars().any(|c| c.is_numeric()) {

                    // alerta como falso 

                    return Err("String Validations Error :: Atenção , não coloque digitos em letras !".into());

             }


             // se tudo estiver bem


            Ok(_trimmer.to_string()) // 

  }


 



  // integer validations 

  
  fn is_checked_only_digits(x : &String) -> TStringTypeofException<String> {


            let _trimmer = x.trim(); 


            // confere e ve se esta vazio o campo 

             if _trimmer.is_empty() {

                    return Err("String.ToFixedError :: Erro ! O campo está vazio!".into()); 

             }


             // analisa se o campo contém somente letras

             if _trimmer.chars().any(|c| c.is_alphabetic()) {

                    // alerta como falso 

                    return Err("String Validations Error :: Atenção , não coloque letras em números !".into());

             }


             // se tudo estiver bem


            Ok(_trimmer.to_string()) // 

  }





  //Parse Exception Type 

// <informa a biblioteca parsing> 

  use std::num::{ParseIntError} ;  


  fn parse_integer_8(x : &String) -> Result<i8 , ParseIntError> {

               x.trim().parse::<i8>()

  }


  
  fn parse_integer_16(x : &String) -> Result<i16 , ParseIntError> {

               x.trim().parse::<i16>()

  }



    fn parse_integer_64(x : &String) -> Result<i64 , ParseIntError> {

               x.trim().parse::<i64>()

    }




    /* Espaço validações -> Regex */ 


    fn is_checked_cpf(x : &str) -> Result<bool , AsynchronusException> {

            let re = Regex::new(r"^\d{3}\.\d{3}\.\d{3}-\d{2}$").unwrap();    

                
            // avalia a expressao

            if re.is_match(&x.trim()) {

                     return Ok(true)

            }


            else {

                    // dispara o erro em si

                    return Err(AsynchronusException::REGEX_SYSTEM_ERROR);

            }



    }





     fn is_checked_cnpj(cnpj : &str) ->  Result<bool , AsynchronusException> {


             let re = Regex::new(r"^\d{2}\.\d{3}\.\d{3}\/\d{4}-\d{2}$").unwrap();
    
                

            // avalia a expressao

            if re.is_match(&cnpj.trim()) {

                     return Ok(true)

            }


            else {

                    // dispara o erro em si

                    return Err(AsynchronusException::REGEX_SYSTEM_ERROR);

            }

    
     } 



      
    

     fn is_checked_date_format(dt : &String) ->  Result<bool , AsynchronusException> {
        

               let re = Regex::new(r"^\d{2}\/\d{2}/\d{4}$").unwrap();


               if re.is_match(&dt.trim().replace("r" , "")) {

                    return Ok(true)

               }


               else {

                      // dispara o erro em si

                    return Err(AsynchronusException::REGEX_SYSTEM_ERROR);

               }

    } 

    /*

    fn is_checked_telefone(tel : &String) ->  Result<bool , AsynchronusException> {

              


    } 

    */






 


     // _______________ <entrada de dados > _______________

    
     pub fn input_nome() -> TStringTypeofException<String> 

     {

     	  let mut input_nome : String = String::new(); 

     	  io::stdin().read_line(&mut input_nome)?; 




     	Ok(input_nome.to_string())

     }





     pub fn input_tipo_pessoa() -> TStringTypeofException<String> 

     {

     	  let mut input_tipo_pessoa : String = String::new(); 

     	  io::stdin().read_line(&mut input_tipo_pessoa)?; 




     	Ok(input_tipo_pessoa.to_string())

     }





     pub fn input_tipo_conta() -> TStringTypeofException<String> 

     {

     	  let mut input_tipo_conta : String = String::new(); 

     	  io::stdin().read_line(&mut input_tipo_conta)?; 




     	Ok(input_tipo_conta.to_string())

     }
 

    
     // para pessoa fisica -> cpf ! < > 

      pub fn input_cpf() -> TStringTypeofException<String> 

     {

          let mut input_cpf : String = String::new(); 

          io::stdin().read_line(&mut input_cpf)?; 




        Ok(input_cpf.to_string())

     }



     // para pessoa juridica 



        pub fn input_cnpj() -> TStringTypeofException<String> 

     {

          let mut input_cnpj : String = String::new(); 

          io::stdin().read_line(&mut input_cnpj)?; 




        Ok(input_cnpj.to_string())

     }


  	   



     pub fn input_gerente() -> TStringTypeofException<String> 

     {

          let mut input_gerente : String = String::new(); 

          io::stdin().read_line(&mut input_gerente)?; 




        Ok(input_gerente.to_string())

     }



     // ---------> dados pessoais -----------------------> 





     pub fn input_endereco() -> TStringTypeofException<String> 

     {

          let mut input_endereco : String = String::new(); 

          io::stdin().read_line(&mut input_endereco)?; 




        Ok(input_endereco.to_string())

     }





     pub fn input_bairro() -> TStringTypeofException<String> 

     {

          let mut input_bairro : String = String::new(); 

          io::stdin().read_line(&mut input_bairro)?; 




        Ok(input_bairro.to_string())

     }




     pub fn input_telefone() -> TStringTypeofException<String> 

     {

          let mut input_telefone : String = String::new(); 

          io::stdin().read_line(&mut input_telefone)?; 




        Ok(input_telefone.to_string())

     }



  	   
     // ---------> dados bancarios!----------------------> 
        


     pub fn input_agencia() -> TStringTypeofException<String> 

     {

          let mut input_agencia : String = String::new(); 

          io::stdin().read_line(&mut input_agencia)?; 




        Ok(input_agencia.to_string())

     }


   