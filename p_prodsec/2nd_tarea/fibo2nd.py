def main():
    x=0
    y=1
    fibo(x,y)

def fibo(x,y):
    while True:
        try:
            z= int(input("introduce un numero: "))
            break
        except ValueError:
            print("El valor introducido no es un numero")

    fibo_calc(x,y,z)

def fibo_calc(x,y,z):
    for i in range(z):
        print(x)
        xy = x
        x = y
        y = xy +  x

main()