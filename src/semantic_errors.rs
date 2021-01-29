/* A struct containing semantic errors */

pub struct semantic_errors{
    undeclaredVariable: "error: variable is not declared",
    multipleDeclaration: "error: variable had been declared before",
    typeMismatched: "error: types mismatched \n variable doesn't have a value matched with the identifier",
    assignMismatched: "error: can't assign values to different types",
    wrongCondition: "error: wrong condition \n conditions must return a boolean (true or false)",
    wrongOperation: "error: can't perform arithmetic operations on 'Harf'",
    printMismatched: "error: use to print variables with type ",
}