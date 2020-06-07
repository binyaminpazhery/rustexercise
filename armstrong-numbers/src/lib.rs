pub fn is_armstrong_number(number: u32) -> bool 
{
  let mut c=0; 
  let mut reminder: u32=0;
  let mut sum=0;
  let mut no=number;
  let mut a=number;
  while no>0 
  {
    no=no/10;
    c+=1;  
  }
  while a>0 {
    reminder=a%10;
    sum += u32::pow(reminder,c);
    a /=10;
  }
  println!("{:?}  {} {}",c,sum,reminder );

  if sum==number{
    true
  }  
  else {
    false
  }
}