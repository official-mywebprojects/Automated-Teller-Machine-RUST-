use std::io;


fn main(){
    //DEMO SCREEN
    println!("\n Welcome to CLI Bank! \n A place you can withdraw free fund!! \n");
    println!("Tap any key to start");

    //waiting to read user's input
    let mut any_key = String::new();

    //read user input
    io::stdin().read_line(&mut any_key).expect("Failed to read line");
    //next screen
    enter_pin();

    fn enter_pin(){
        loop{
            println!("\n Please enter your secret number");
            println!("ENTER PIN: \n");

            let mut user_pin = String::new();

            //read user input
            io::stdin().read_line(&mut user_pin).expect("Failed to read line");

            //checking to accept number
            let user_pin: u32 = match user_pin.trim().parse(){
                Ok(num) => num,
                Err(_) => continue,
            };

            if user_pin > 9999 {
                println!("Please enter a four digits PIN");
            }else if user_pin <= 999{
                println!("PIN can't be less than four digits");
            }else{
                wtdscreen(); break; 
            }
        }
    }
    

    fn wtdscreen(){
        loop{
            println!("\n What do you want to do?");
            println!("CHOOSE AN OPTION:");

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
                println!("Enter a valid option between 1 and 2.");
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
            println!("Account Type:");
            println!("===================");

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
            
                println!("Amount to Withdraw \n Choose an option:");
                println!("===================");

                println!("1. 1000");
                println!("2. CANCEL \n");
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
                    println!("Enter a valid option");
                }
            
        
            }else if acc_opt == opt_two{
                cancelscr(); break;
            }else{
                //do soemth
                println!("Enter a valid option"); 
            }

        }
        
    }

    //enquiry
    // fn enquiry(){
    //     println!("Enquiry Screen");
    // }

    // //transfer
    // fn transfers(){
    //     println!("Transfer Screen");
    // }

    //cancel
    fn cancelscr(){
        println!("You cancelled this process... \n");
        end_screen();
    }

    //successful transaction
    fn successtr(){
        println!("Successful transaction \n #### TAKE YOUR CASH ### \n");
        end_screen();
    }

    //End transaction screen
    fn end_screen(){
        println!("Thank you for banking with us!!! \n SEE YOU SOON...");
    }
    
}
