import requests
import json

tweets_correctos = 0
tweets_incorrectos = 0

def peticion_post(tweet,endpoint):
    url = 'http://localhost:8080/iniciarCarga'
    response = requests.post(endpoint, json = tweet)
    print(response.content)
    if response.status_code == 200:
       return  True
    else:
        return False


def cargar_datos(ruta,endpoint):
    tweets_correctos = 0
    tweets_incorrectos = 0
    with open(ruta) as contenido:
        data = json.load(contenido)
        for tweet in data:
         bandera = peticion_post(tweet,endpoint)
         if bandera == True:
             tweets_correctos = tweets_correctos + 1
         else:
             tweets_incorrectos = tweets_incorrectos + 1
    return tweets_correctos, tweets_incorrectos



if __name__ == '__main__':
    tweetsC = 0
    tweetsI = 0
    ruta =  input('ingrese ruta del archivo: ')
    endpoint = input('ingrese la URL del balanceador')    
    tweetsC, tweetsI = cargar_datos(ruta,endpoint)    
    print("Tweets cargados satisfactoriamente: "+ str(tweetsC))
    print("Tweets cargados con errores: "+str(tweetsI))

