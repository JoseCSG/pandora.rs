# Documentación del lenguaje

## Maquina virtual

En esta entrega no se crearon nuevos estatutos. Estuve trabajando en la maquina virtual. Actualmente, el programa soporta estatutos while, e if. Puede hacer operaciones e iteraciones. Las funciones están pendientes para la siguiente entrega.

## Creación de cuadruplos para estatutos While E If

Se añadio un nuevo stack a la clase ProgramManager llamado jumps_stack, el cual guarda la posicion para los estatutos no completos (GOTOF, GOTO) para los estatutos **If** Y **While**.

Se crearon puntos neuralgicos para hacer push al stack con la posición del estatuto a modificar, y para completar el cuadruplo correspondiente con la posición adecuada.

## Tabla de valores

En esta entrega implemente una tabla de valores para tener direcciones virtuales de las variables que son declaradas. Modifique las reglas de Termino, Expresion, Exp y Factor para que el vector polaco funcionara de forma correcta. Se crean cuadruplos con los valores de memoria y valores de operacion correspondientes.

Implemente una clase llamada ProgramManager en la que gestiono todas las operaciones del programa. En este tengo el vector polaco, las pilas para generarlo, la tabla de valores, la tabla de funciones, el cubo semántico, y la lista de cuadruplos. Asi como metodos para interactuar con estos atributos.

## Analisis Semantico

En esta entrega se implementaron clases de tablas de variables y funciones, para poder guardar la información de las variables declaradas en una función y globalmente. De igual manera, se implementó un cubo semántico, el cual permite determinar si las operaciones dentro de una operacion son validas o no.

## Herramientas utilizadas

Para realizar el lexer, opte por utilizar la herramienta Logos, ya que la implementación es sencilla. Sólo se definen las reglas de los tokens en un enum, y con eso ya puede tokenizar un texto. Otra razón por la cual escogi esta herramienta, es por su integración sencilla con la herramienta de mi parser, LALRPOP. Esta herramienta tiene documentación de cómo utilizarla con otros crates de rust, además de cómo funciona por si sola la herramienta. LALRPOP ofrece un lexer, sin embargo, me pareció más sencillo de utilizar el de Logos. LALRPOP es parser bottom to top, y utiliza un lenguaje LR (Left to Right), permite usar signos de expresiones regulares, como \*, +, ?, para simplificar ciertas reglas gramaticales, y evitar ambigüedades. Además ofrece una manera de especificar precedencia y asociatividad usando macros de rust.

El parser detecta si un programa esta escrito correctamente, de lo contrario marca error. Escribi cinco pruebas.

1. Es un programa sencillo donde se define una funcion antes del main, y se hace un while con un print adentro.
2. Escribimos un while antes del main, esto por nuestras reglas no esta permitido, y no es aceptado como un programa valido.
3. Define dos funciones con parametros antes del main un while con un print y una asignacion adentro, y una llamada a una funcion con una asignacion afuera del while
4. Hace falta escribir un identificador acompañando a la palabra reservada program, ademas de tener un "{" de más, creando un programa no valido.
5. Este programa incluye todo lo que un programa puede tener, declaracion de variables y funciones antes del main, dentro del main, condiciones, while, print, asignaciones, y llamada a funciones. Como tiene un orden correcto, genera un programa valido.

## Definición del Lenguaje

### Tokens en el lenguaje y regex

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

### Reglas Gramaticales

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
- => void id ( **<PARAMS\>** ) **<BODY\>** ;

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
