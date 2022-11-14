use std::io;
extern crate timer;
extern crate chrono;
use std::sync::mpsc::channel;


fn main(){
    
    process_timer(2);

    //DEMO SCREEN
    println!("==================================\n\nWelcome to CLI Bank!\nGet access to free fund anytime!!\n\n==================================");
    println!("TAP ANY KEY TO CONTINUE");

    //waiting to receive user input
    let mut any_key = String::new();

    //read user input
    io::stdin().read_line(&mut any_key).expect("Failed to read line");
    //next screen
    enter_pin();

    fn enter_pin(){
        loop{
            println!("\n..........\nPlease enter your secret number\nP I N: \n");

            let mut user_pin = String::new();

            //read user input
            io::stdin().read_line(&mut user_pin).expect("Failed to read line");

            //checking to accept number
            let user_pin: u32 = match user_pin.trim().parse(){
                Ok(num) => num,
                Err(_) => continue,
            };

            if user_pin > 9999 {
                println!("x Please enter a four digits PIN");
            }else if user_pin <= 999{
                println!("x PIN can't be less than four digits");
            }else{
                wtdscreen(); break; 
            }
        }
    }
    

    fn wtdscreen(){
        //Timer (2s)
        process_timer(2);
        loop{
            println!("\n==================================\nWhat do you want to do?\nCHOOSE AN OPTION:\n==================================");

            let one: u32 = 1;
            let two: u32 = 2;

            // println!("1. Top up");
            println!("1. Withdraw");
            // println!("3. Enquiry");
            // println!("4. Transfer");
            println!("2. CANCEL");

            //waiting for user input
            let mut user_choose = String::new();

            //reading user input
            io::stdin().read_line(&mut user_choose).expect("Failed to read line");

            let user_choose: u32 = match user_choose.trim().parse(){
                Ok(num) => num,
                Err(_) => continue,
            };

            if user_choose == one {
                withdraw(); break;
            }else if user_choose == two{
                cancelscr(); break;
            }else{
                println!("x Enter a valid option between 1 and 2.");
            }
        }
        
    }    

    //Top up
    // fn topup(){
    //     println!("Top up Screen");
    // }

    //withdraw
    fn withdraw(){
        loop{
            println!("\nAccount To Withdraw:");
            println!("====================");

            println!("1. CLI Account");
            println!("2. CANCEL \n");
            println!("Enter 1. or 2.");

            let opt_one = 1;
            let opt_two = 2;

            //waiting for user input
            let mut acc_opt = String::new();
            //reading user input
            io::stdin().read_line(&mut acc_opt).expect("Failed");

            //
            let acc_opt: u32 = match acc_opt.trim().parse(){
                Ok(num) => num,
                Err(_) => continue,
            };    
        
        


        if acc_opt == opt_one {
            
                println!("\n=========================\nAmount to Withdraw\nCHOOSE AN OPTION:\n=========================");

                println!("1. 1,000,000");
                println!("2. CANCEL\n");
                println!("Enter 1. or 2. \n");

                let c1 = 1;
                let c2 = 2;

                //waiting for user input
                let mut amt_opt = String::new();
                //reading user input
                io::stdin().read_line(&mut amt_opt).expect("Failed");

                //check number
                let amt_opt: u32 = match amt_opt.trim().parse(){
                    Ok(num) => num,
                    Err(_) => continue,
                };

                if amt_opt == c1 {
                    successtr(); break
                }else if amt_opt == c2{
                    cancelscr(); break;
                }else{
                    println!("x Enter a valid option");
                }
            }else if acc_opt == opt_two{
                cancelscr(); break;
            }else{
                //do someth
                println!("x Enter a valid option"); 
            }

        }
        
    }


    //ATM Timer
    fn process_timer(stt_print: i64){
        let timer = timer::Timer::new();
        let(tx, rx) = channel();

        let _atm_timer = timer.schedule_with_delay(chrono::Duration::seconds(stt_print), move || {
            tx.send(()).unwrap();
        });

        rx.recv().unwrap();    
    }

    //cancel
    fn cancelscr(){
        //timer (2s)
        process_timer(2);
        println!("\nYou cancelled this process... \n");
        end_screen();
    }

    //successful transaction
    fn successtr(){
        //timer (4s)
        process_timer(4);
        println!("\n\nPlease hold, while your transaction is processing... \n");
        process_timer(3);
        println!("--Transaction Complete--\nTAKE YOUR CASH");
        
        end_screen();
    }

    //End transaction screen
    fn end_screen(){
        //timer (2s)
        process_timer(2);
        println!("Thank you for banking with us!!! \n SEE YOU SOON...");
    }
    
}
