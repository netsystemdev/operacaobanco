






 /* ------ <importacao das diretrizes > - -------------- */
 
 pub mod private;

 use private::abstract_class_banco::Banco;


 pub mod interface;

     use interface::info::Info;

 
 pub mod CRM; 

     use CRM::input_type_abertura_banco::abertura_banco;


 pub mod exceptions;


 



 
 // ------------ < bibliotecas > ------------------------------


     extern crate chrono;

     use chrono::{Utc , NaiveDate} ; 



     use std::io;

     use std::io::prelude::*; 

     use clearscreen;


     use prettytable::{Table , row , Cell} ;  







     // ---------- main class -------------


     type TRuntimeException<T> = Result<T , Box<dyn std::error::Error>> ; 


     fn main() -> TRuntimeException<()> 

     {

            // Aqui , carrega o menubanco


            let _ = menubanco()?;



        Ok(())
     }






     // ------------------- menu ------------------------

     use std::process::exit;


     fn menubanco() -> TRuntimeException<()> 

     {

          let _ = clearscreen::clear();


           // Enquanto for satisfeito


           while true 

           {

                    // invoca a funcao , que gera a tabela criada !


                    let _ = geracao_tabela_opcoes()?;


                    println!("\n");


                    print!("OPCAO  _ "); 

                    io::stdout().flush()?; 

                    let campox = match input_opcao() {

                            Ok(val) => val , 

                                Err(error) => {

                                        // mesmo com erro , nao derruba o programa!

                                        return Err(error) ; 

                                        continue; 


                                },

                    };



                    // convert and validation

                    let econvert : i8 = match get_exception_for_i8(campox) {

                            Ok(val) => val , 


                               Err(parse_exception) => {

                                      eprintln!("Exception value  :  {} " , parse_exception); 

                                      exit(0);  // encerra o processo

                               },


                    };



                    // se nao houve problemas , redirecionamento de selecao para cada tipo de caso [  ]  

                    let _ = match econvert {

                            /* para cada ação , sequencial de caso */

                            1 => {

                                    let _x = abertura_banco()?;


                            },


                            0 => {

                                      /* Encerra a base de dados */

                                      println!("\n");

                                      println!("SESSAO ENCERRADA - LOGOUT "); 

                                      println!("Desenvolvimento : Thiago V. Santos ");


                                      // encerramento : limpa a tela 

                                       io::stdin().read(&mut [0u8])?;

                                       let _ = clearscreen::clear(); 

                                       exit(0);






                            }, 



                              _ => {

                                    println!("opção invalida ! informe novamente !") ; 

                                    // aceita o buffer 

                                    io::stdin().read(&mut [0u8])?;

                                    // limpa a tela 

                                    clearscreen::clear() ;

                                    // devolve o ciclo do laço

                                    continue;


                              }, 

                    };







           }



        io::stdout().flush()?; 

        io::stdin().read(&mut [0u8])?;

        Ok(())

     }

    


     fn geracao_tabela_opcoes() -> TRuntimeException<()> 

     {


            // >> constroi a tabela , sentido horizontal (simples)

            let mut table = Table::new(); 

            table.add_row(row![" OPERAÇÃO BANCO - DESENVOLVIMENTO DE APLICATIVO - AUTOMAÇÃO CLI"]);

            table.add_row(row![""]);

            table.add_row(row![" MODO DE OPERAÇÂO " , " OPCAO "]);

            table.add_row(row!["ABERTURA DE CONTA" , "1"]); 

            table.add_row(row!["CONSULTA POR CPF " , "2"]); 

            table.add_row(row!["CONSULTA POR CNPJ " , "3"]); 

            table.add_row(row!["CONSULTA POR NOME" , "4"]); 

            table.add_row(row!["PEDIDO DE SAQUE" , "5"]);

            table.add_row(row!["DEPOSITO - MESMA CONTA" , "8"]); 

            table.add_row(row!["RELATORIO DE SAQUES - CPF" , "6"]);  

            table.add_row(row!["RELATORIO DE SAQUES - CNPJ" , "7"]); 

            table.add_row(row!["SAIR DO SISTEMA " , "0"]); 

                



                // cria a tabela e redesenha

                table.printstd();




        Ok(())
     }




     // -------------- campo ------------

     pub fn input_opcao() -> Result<String , Box<dyn std::error::Error>> {

            // campo de insercao

            let mut input_opcao : String = String::new(); 

            io::stdin().read_line(&mut input_opcao)?; 


            let _trimmer = input_opcao.trim(); 


            if _trimmer.is_empty() {

                    return Err("Codigo 2 : O campo de opção está vazio!".into()) ;
            }



            // procura e ve se ele está digitando letras ou algo de str[char]

            if _trimmer.chars().any(|c| c.is_alphabetic()) {

                    return Err("Atenção ! por favor , não coloque letras ou char[string] em campo int!".into());

            }




        Ok(_trimmer.to_string()) 
     } 




     use std::num::{ParseIntError};

     pub fn get_exception_for_i8(x : String) -> Result<i8 , ParseIntError> {

                x.trim().parse::<i8>()

     }