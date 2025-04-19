# Smart CLI Calculator

The **Smart CLI Calculator** is a command-line application that allows users to perform various mathematical and unit conversion operations using natural language inputs. It is designed to be intuitive and user-friendly, supporting a wide range of operations.

## Features

- **Basic Arithmetic**:
  - Addition: `add 5 and 3`
  - Subtraction: `subtract 2 from 10`
  - Multiplication: `multiply 4 by 6`
  - Division: `divide 8 by 2`

- **Percentage Calculations**:
  - `what is 20% of 50`

- **Power and Roots**:
  - Square root: `square root of 16` or `sqrt of 16`
  - Power: `what is 2 to the power of 3`

- **Unit Conversions**:
  - Kilometers to miles: `convert 10 km to miles`
  - Kilograms to pounds: `convert 5 kg to pounds`

## How to Use

1. Clone the repository:
   ```sh
   git clone https://github.com/your-username/smart-cli-calculator.git
   cd smart-cli-calculator
2. Build the project using cargo:
   ```sh
   cargo build 
3. Run the application:
   ```sh
   cargo run
4. Enter commands in the terminal to perform calculations. For example:

> add 5 and 3= 8
> convert 10 km to miles= 6.21371

5.To exit the application, type exit or quit.

## Error Handling
Division by zero is not allowed and will display an error message.
Square root of a negative number is not supported and will display an error message.
Invalid inputs will result in a message indicating that the input could not be understood.


##Dependencies
This project uses the following Rust crates:
-lazy_static: For defining static regular expressions.
-regex: For parsing natural language inputs.

Project Structure
src/main.rs: Contains the main logic for handling user input and performing calculations.

##License
This project is licensed under the MIT License. See the LICENSE file for details.

Contributing
Contributions are welcome! Feel free to open issues or submit pull requests to improve the functionality or add new features.

Contact
For questions or feedback, please contact samir1672007@gmail.com. ```



