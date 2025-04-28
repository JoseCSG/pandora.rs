# Definición del Lenguaje

## Tokens en el lenguaje y regex

- **Identificación**
  - id: [a-zA-Z\_][a-zA-Z0-9_]\*
- **Constantes**
  - cte_string: \"[^\"\n]\*\"
  - cte_int: -?[0-9]+
  - cte_float: -?[0-9]+\ .[0.9]+
- **Operadores**
  - +, -, /, \*, >, <, !=, =
- **Símbolos**
  - ; , ( ) { } [ ] .
- **Keywords**
  - PROGRAM, MAIN, END, VAR, INT_TYPE, FLOAT_TYPE, VOID, PRINT, WHILE, DO, IF, ELSE
- **Tipo de dato**
  - int, float

## Reglas Gramaticales

**<CTE\>**

- => cte_int
- => cte_float

**<TYPE\>**

- => int
- => float

**<OP_ADITIVO>**

- => +
- => -

**<OP_MULTIPLICATIVO>**

- => \*
- => /

**<OP_LÓGICO>**

- => >
- => <
- => !=

**<EXPRESIÓN>**

- => **<EXP\>**
- => **<EXP\> <OP_LÓGICO> <EXP\>**

**<EXP'>**

- => **<OP_ADITIVIO> <EXP\>**

**<EXP\>**

- => **<TÉRMINO> <EXP'>**

**<TÉRMINO'>**

- => **<OP_MULTIPLICATIVO> <TÉRMINO>**

**<TÉRMINO>**

- => **<FACTOR\> <TÉRMINO'>**

**<FACTOR\>**

- => ( **<EXPRESIÓN>** )
- => **<OP_ADITIVO>** id
- => **<OP_ADITIVO> <CTE\>**
- => id
- => **<CTE\>**

**<ASSIGN\>**

- => id = **<EXPRESIÓN>** ;

**<CYCLE\>**

- => while ( **<EXPRESIÓN>** ) do **<BODY\>**;

**<CONDITION\>**

- => if ( **<EXPRESIÓN>** ) **<BODY\>** ;
- => if ( **<EXPRESIÓN>** ) else **<BODY\>** ;

**<F_CALL'>**

- => , **<EXPRESIÓN> <F_CALL'>**
- => ε

**<F_CALL>**

- => id ( **<EXPRESIÓN>** ) ;
- => id ( ) ;
- => id ( **<EXPRESIÓN> <F_CALL'>** )

**<PRINT''>**

- => , **<PRINT'>**
- => ε

**<PRINT'>**

- => cte_string
- => **<EXPRESIÓN>**
- => **<EXPRESIÓN> <PRINT''>**

**<PRINT\>**

- => print ( **<PRINT'>** ) ;

**<STATEMENT\>**

- => **<ASSIGN\>**
- => **<CONDITION\>**
- => **<CYCLE\>**
- => **<F_CALL>**
- => **<PRINT\>**

**<BODY'>**

- => ε
- => **<STATEMENT\>**
- => **<STATEMENT\> <BODY'>**

**<BODY\>**

- => {}
- => { **<BODY'>** }

**<IDENTIFIER'>**

- => ε
- => , **<IDENTIFIER\>**

**<IDENTIFIER\>**

- => id **<IDENTIFIER'>**

**<VARS'>**

- => **<IDENTIFIER\>** : **<TYPE\>** ;
- => **<IDENTIFIER\>** : **<TYPE\>** ; **<VARS'>**

**<VARS\>**

- => var **<VARS'>**

**<PARAM'>**

- => ε
- => , id : **<TYPE\>** **<PARAM'>**

**<PARAM\>**

- => id : **<TYPE\>** **<PARAM'>**

**<PARAMS\>**

- => ε
- => **<PARAM\>**

**<FUNCS\>**

- => void id ( **<PARAMS\>** ) { **<VARS\>** **<BODY\>** } ;
- => void id ( **<PARAMS\>** ) { **<BODY\>** } ;

**<VARS_PROGRAM>**

- => ε
- => **<VARS\>**

**<FUNCS_PROGRAM'>**

- => ε
- => **<FUNCS\>** **<FUNCS_PROGRAM>**

**<FUNCS_PROGRAM>**

- => ε
- => **<FUNCS\>** **<FUNCS_PROGRAM'>**

**<PROGRAM\>**

- => program id ; **<VARS_PROGRAM>** **<FUNCS_PROGRAM>** main **<BODY\>** end
