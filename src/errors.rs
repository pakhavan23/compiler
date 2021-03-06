/* A struct containing syntax errors */

pub struct errors{
    missingCaret: "error: expected '^' at the end of the line",
    misplacedCaret: "error: move '^' to the end of the line",
    missingVariable: "error: expected variable name after identifier (Ashari,Harf,Sahih)",
    missingValue: "error: expected a value after '='",
    missingCommaOrEqual: "error: expected ',' or '=' between variable names",
    missingComma: "error: expected ',' between variable names",
    missingOperator: "error: expected one of these 7 tokens: {Jam , Kam, Zarb, Tagsim, Bagimonde, YekiBala, YekiPain , =}",
    noArgBenevis: "error: missing an argument for Benevis \n Insert the argument to be printed between ()",
    noArgBegir: "error: missing arguments for Begir \n Use '%d' , '%c' or '%f' and a variable name to store input",
    wrongArBegir: "error: wrong arguments for Begir \n Use '%d' , '%c' or '%f' and a variable name to store input",
    noConditionAndBlockAgar: "error: agar has no condition and no blocks",
    noConditionAgar: "error: agar has no condition",
    noBlockAgar: "error: agar has no blocks",
    wrongConditionParanthesisAgar: "error: expected {condition} after agar",
    noConditionAndBlockTa: "error: ta has no condition and no blocks",
    noConditionTa: "error: ta has no condition",
    noBlockTa: "error: ta has no blocks",
    wrongConditionParanthesisTa: "error: expected {condition} after ta",
    wrongBlockParanthesis: "error: expected [statement] after condition",
    wrongComparison: "error: expected two values to compare",
    noVariableYekiBala: "error: expected a variable name after YekiBala",
    noVariableYekiPain: "error: expected a variable name after YekiPain"
}