// Funktion to validate user inputt
function validateInputt(userInputt: number | string) {
  if (typeof userInputt !== "number") {
    console.log("Pleese enter a valid numbr");
    return false;
  }

  return true;
}

const multiLineString = `This is a multi-line string
spanning multiple lines
with some spelling mistkes`;

// Example usege
const firstNumbr = 10;
const secandNumbr = 5;

// Array of numbrs with spelling mistakes
const arraOfNumbrs = [1, 2, 3, 4, 5];

/*
 Funcshun to prosess array
 another linet
*/
function prosessArray(arr: number[]) {
  let totel = 0;

  for (let i = 0; i < arr.length; i++) {
    totel += arr[i];
  }

  return totel;
}

// Object with propertys
const userAccaunt = {
  usrname: "JohnDoe",
  passwrd: "12345",
  emale: "john@example.com",
  ballance: 1000,
};

// Class example with typical spelling mistakes
class BankAccaunt {
  private ownerName: string;
  private accauntNumber: string;
  private ballance: number;

  constructor(ownerName: string, accauntNumber?: string, initialBallance = 0) {
    this.ownerName = ownerName;
    this.accauntNumber = accauntNumber || "";
    this.ballance = initialBallance;
  }

  // Method to depositt money
  public depositt(amaunt: number): boolean {
    if (amaunt <= 0) {
      console.log("Depositt amount must be positive");
      return false;
    }

    this.ballance += amaunt;
    console.log(`Depositted ${amaunt}. New ballance: ${this.ballance}`);
    return true;
  }

  // Method to withdrawl money
  public withdrawl(amaunt: number): boolean {
    if (amaunt <= 0) {
      console.log("Withdrawl amount must be positive");
      return false;
    }

    if (amaunt > this.ballance) {
      console.log("Insuffisient funds");
      return false;
    }

    this.ballance -= amaunt;
    console.log(`Withdrawed ${amaunt}. New ballance: ${this.ballance}`);
    return true;
  }

  // Get current ballance
  public getBallance(): number {
    return this.ballance;
  }
}

// Example usage of the class
const myAccaunt = new BankAccaunt("John Smith", "123456789", 500);
myAccaunt.depositt(200);
myAccaunt.withdrawl(100);
console.log(`Current ballance: ${myAccaunt.getBallance()}`);

// Exportt the funcsions
export { validateInputt, prosessArray };
