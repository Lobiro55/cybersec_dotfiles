def  fibonacci (z):
    # Inicializamos xy y con los valores de 'x = 0' y 'y = 1'
    x=0
    y=1
    # Inicializamos un bucle dandole un parametro z que sera el nimero de veces que vamos a iterar
    for i in range(z):
    # Escribimos por pantalla el valor actual de x
     print(x)
    # Sumamos los valores de x y de y
     xy = x + y
    # Hacemos que el valor de x sea y
     x = y
    # Hacemos que y sea la suma de xy
     y = xy


fibonacci(10)

import math
def binet (a):

    phi = (1+math.sqrt(5))/2
    psi = (1-math.sqrt(5))/2
    print(phi)
    print(psi)
    fib = (phi**a - psi**a) / math.sqrt(5)
    print(fib)

binet(9)