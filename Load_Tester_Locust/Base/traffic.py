# Se importa para leer archivo con estructura JSON
import json

from random import random, randrange
from sys import getsizeof
from locust import HttpUser, task, between

debug = True

def printDebug(msg):
    if debug:
        print(msg)

# Clase para leer el archivo.
class Reader():

    def __init__(self):
        self.array = []
        
    def pickRandom(self):
        length = len(self.array)
        
        if (length > 0):
            random_index = randrange(0, length - 1) if length > 1 else 0

            return self.array.pop(random_index)

        else:
            print (">>> No hay más valores para leer en el archivo.")
            return None
    
    def load(self):
        print (">>>Iniciando con la carga de datos")
        try:

            with open("traffic.json", 'r') as data_file:

                self.array = json.loads(data_file.read())

            
            print (f'>> Reader: Datos cargados correctamente, {len(self.array)} datos -> {getsizeof(self.array)} bytes.')
        except Exception as e:

            print (f'>> Reader: No se cargaron los datos {e}')


class MessageTraffic(HttpUser):

    wait_time = between(0.1, 0.9)


    def on_start(self):
        print (">> MessageTraffic: Iniciando el envio de tráfico")
        
        self.reader = Reader()
        
        self.reader.load()


    @task
    def PostMessage(self):
       
        random_data = self.reader.pickRandom()
        
        
        if (random_data is not None):

            data_to_send = json.dumps(random_data)
            printDebug (data_to_send)

            self.client.post("/", json=random_data)

        else:
            print(">> MessageTraffic: Envio de tráfico finalizado, no hay más datos que enviar.")

            self.stop(True) 
